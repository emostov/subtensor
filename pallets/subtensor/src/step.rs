use super::*;
use sp_std::convert::TryInto;
use substrate_fixed::types::I65F63;
use substrate_fixed::transcendental::exp;
use frame_support::IterableStorageMap;
use sp_std::if_std; // Import into scope the if_std! macro.

impl<T: Config> Pallet<T> {

    /// Block setup: Computation performed each block which updates the incentive mechanism and distributes new stake as dividends.
    /// 
    /// The following operations are performed in order.
    /// 
    /// 
    /// 
    /// ------ Requires ------:
    /// 
    /// Stake: 
    ///     -- S (Vec[n, u64])
    ///     -- s_i = tokens staked by peer i
    /// 
    /// Weights: 
    ///     -- W (Vec[n, Vec[n, u64]]): 
    ///     -- w_i = weights set by peer i
    ///     -- w_ij = weight set by peer i to peer j
    /// 
    /// Bonds: 
    ///     -- B (Vec[n, Vec[n, u64]]): 
    ///     -- b_i = bonds held by peer i
    ///     -- b_ij = bonds held by peer i in peer j
    /// 
    /// tau:
    ///     -- tau (u64):
    ///     -- tokens released this block.
    /// 
    /// 
    /// 
    /// ------ Computes ------:
    /// 
    /// Ranks: 
    ///    -- ranks Vec[u64] = R = (W^T * S)
    ///    -- r_i = SUM(j) s_j * w_ji
    ///    -- DB Reads/Writes: O( n^2 ), Decoding: O( n^2 ), Operations: O( n^2 )
    /// 
    /// Trust: 
    ///    -- trust Vec[u64] = T = (C^T * S) where c_ij = 1 iff w_ji != 0 else 0
    ///    -- t_i = SUM(j) s_j if w_ji != 0
    ///    -- DB Reads/Writes: O( n^2 ), Decoding: O( n^2 ), Operations: O( n^2 )
    /// 
    /// Incentive: 
    ///    -- incentive Vec[u64] = Icn = R * (exp(T) - 1)
    ///    -- icn_i = r_i * ( exp( t_i * temp ) - 1 ) )
    ///    -- DB Reads/Writes: O( 0 ), Decoding: O( 0 ), Operations: O( n )
    ///
    /// Inflation: 
    ///    -- inflation Vec[u64] = Inf = Icn * tau
    ///    -- inf_i = icn_i * tau
    ///    -- DB Reads/Writes: O( 0 ), Decoding: O( 0 ), Operations: O( n )
    /// 
    /// Dividends: 
    ///    -- dividends Vec[u64] = Div = B * Inf 
    ///    -- d_i = 0.5 * (SUM(j) b_ij * inf_j) + ( 0.5 * inf_i)
    ///    -- DB Reads/Writes: O( n^2 ), Decoding: O( n^2 ), Operations: O( n^2 )
    /// 
    /// 
    /// 
    /// ------ Updates ------:
    /// 
    /// Delta Stake:
    ///    -- S = S + D
    ///    -- s_i = s_i + d_i
    /// 
    /// Delta Bonds:
    ///    -- B = B + (W * S)
    ///    -- b_ij = b_ij + (w_ij * s_i)  
    ///
    /// 
    /// Note, operations 1 and 2 are computed together. 
    ////
    pub fn block_step () {

        // Number of peers.
        let n: usize = Self::get_neuron_count() as usize;
        
        // Get total stake
        let total_stake: u64 = TotalStake::<T>::get();

        // Constants.
        let u64_max: I65F63 = I65F63::from_num( u64::MAX );
        let u32_max: I65F63 = I65F63::from_num( u32::MAX );
        let one: I65F63 = I65F63::from_num( 1.0 );
        let rho: I65F63 = I65F63::from_num( 10.0 );
        let kappa: I65F63 = I65F63::from_num( 0.5 );
        let self_ownership: I65F63 = I65F63::from_num( 0.5 );
        let block_emission: I65F63 = I65F63::from_num( 1000000000 ); 

        // To be filled.
        let mut uids: Vec<u32> = vec![];
        let mut active: Vec<u32> = vec![0; n];
        let mut priority: Vec<u64> = vec![0;n];
        let mut bond_totals: Vec<u64> = vec![0; n];
        let mut bonds: Vec<Vec<u64>> = vec![vec![0;n]; n];
        let mut weights: Vec<Vec<(u32,u32)>> = vec![];
        let mut total_stake: I65F63 = I65F63::from_num( 0.0 );
        let mut total_normalized_stake: I65F63 = I65F63::from_num( 0.0 );
        let mut stake: Vec<I65F63> = vec![ I65F63::from_num( 0.0 ) ; n];
        for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
             uids.push( uid_i );
             active [ uid_i as usize ] = 1;
             stake [ uid_i as usize ] = I65F63::from_num( neuron_i.stake );
             total_stake += I65F63::from_num( neuron_i.stake );
             priority [ uid_i as usize ] = neuron_i.priority;
             weights.push( neuron_i.weights );             
             let mut bonds_row: Vec<u64> = vec![0; n];
             for (uid_j, bonds_ij) in neuron_i.bonds.iter() {
                 bonds_row [ *uid_j as usize ] = *bonds_ij;
                 bond_totals [ *uid_j as usize ] += *bonds_ij;
             }
             bonds[ uid_i as usize ] = bonds_row;
        }
        // Normalize stake.
        if total_stake != 0 {
            for uid_i in uids.iter() {
                let normalized_stake:I65F63 = stake[ *uid_i as usize ] / total_stake;
                stake[ *uid_i as usize ] = normalized_stake;
                total_normalized_stake += normalized_stake;
            }
        }   
        if_std! {
            println!( "stake-: {:?}", stake );
        }
    
        
        // Compute ranks and trust.
        let mut total_bonds_purchased: u64 = 0;
        let mut total_ranks: I65F63 = I65F63::from_num( 0.0 );
        let mut total_trust: I65F63 = I65F63::from_num( 0.0 );
        let mut ranks: Vec<I65F63> = vec![ I65F63::from_num( 0.0 ) ; n];
        let mut trust: Vec<I65F63> = vec![ I65F63::from_num( 0.0 ) ; n];
        for uid_i in uids.iter() {

            // Get vars for i.uids
            let stake_i: I65F63 = stake[ *uid_i as usize ];
            let weights_i: &Vec<(u32, u32)> = &weights[ *uid_i as usize ];

            // Iterate over weights.
            for ( uid_j, weight_ij ) in weights_i.iter() {

                // Normalize weight_ij
                let weight_ij: I65F63 = I65F63::from_num( *weight_ij ) / u32_max; // Range( 0, 1 )
                let trust_increment_ij: I65F63 = stake_i; // Range( 0, 1 )                
                let rank_increment_ij: I65F63 = stake_i * weight_ij; // Range( 0, total_active_stake )
                let bond_increment_ij: I65F63 = rank_increment_ij * block_emission; // Range( 0, block_emission )
                if_std! {
                    println!( "-----: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", weight_ij, stake_i, rank_increment_ij, trust_increment_ij, bond_increment_ij, bond_increment_ij.to_num::<u64>());
                }

                // Distribute self weights as priority
                if *uid_i == *uid_j {
                    priority[ *uid_i as usize ] += bond_increment_ij.to_num::<u64>(); // Range( 0, block_emission )

                } else {
                    // Increment neuron scores.
                    ranks[ *uid_j as usize ] += rank_increment_ij;  // Range( 0, total_active_stake )
                    trust[ *uid_j as usize ] += trust_increment_ij;  // Range( 0, total_active_stake )
                    total_ranks += rank_increment_ij;  // Range( 0, total_active_stake )
                    total_trust += trust_increment_ij;  // Range( 0, total_active_stake )
                    
                    // Distribute bonds.
                    bond_totals [ *uid_j as usize ] += bond_increment_ij.to_num::<u64>(); // Range( 0, block_emission )
                    bonds [ *uid_i as usize  ][ *uid_j as usize ] += bond_increment_ij.to_num::<u64>(); // Range( 0, block_emission )
                    total_bonds_purchased += bond_increment_ij.to_num::<u64>(); // Range( 0, block_emission )
                }
            }
        }
        // Normalize ranks + trust.
        if total_trust > 0 && total_ranks > 0 {
            for uid_i in uids.iter() {
                ranks[ *uid_i as usize ] = ranks[ *uid_i as usize ] / total_ranks; // Vector will sum to u64_max
                trust[ *uid_i as usize ] = trust[ *uid_i as usize ] / total_normalized_stake; // Vector will sum to u64_max
            }
        }
        if_std! {
            println!("ranks: {:?}", ranks );
            println!("trust: {:?}", trust );
            println!("bonds: {:?}, {:?}, {:?}", bonds, bond_totals, total_bonds_purchased);
        }

        // Compute consensus, incentive.
        let mut total_incentive: I65F63 = I65F63::from_num( 0.0 );
        let mut consensus: Vec<I65F63> = vec![ I65F63::from_num( 0.0 ) ; n];
        let mut incentive: Vec<I65F63> = vec![ I65F63::from_num( 0.0 ) ; n];
        if total_ranks != 0 && total_trust != 0 {
            for uid_i in uids.iter() {                    
                // Get exponentiated trust score.
                let trust_i: I65F63 = trust[ *uid_i as usize ];
                let shifted_trust: I65F63 = trust_i - kappa; // Range( -kappa, 1 - kappa )
                let temperatured_trust: I65F63 = shifted_trust * rho; // Range( -rho * kappa, rho ( 1 - kappa ) )
                let exponentiated_trust: I65F63 = exp( -temperatured_trust ).expect( "temperatured_trust is on range( -rho * kappa, rho ( 1 - kappa ) )"); // Range( exp(-rho * kappa), exp(rho ( 1 - kappa )) )
                  
                // Compute consensus.
                let ranks_i: I65F63 = ranks[ *uid_i as usize ];
                let consensus_i: I65F63 = one / (one + exponentiated_trust); // Range( 0, 1 )
                let incentive_i: I65F63 = ranks_i * consensus_i; // Range( 0, 1 )
                consensus[ *uid_i as usize ] = consensus_i; // Range( 0, 1 )
                incentive[ *uid_i as usize ] = incentive_i; // Range( 0, 1 )
                total_incentive += incentive_i;
            }
        }
        // Normalize Incentive.
        if total_incentive > 0 {
            for uid_i in uids.iter() {
                incentive[ *uid_i as usize ] = incentive[ *uid_i as usize ] / total_incentive; // Vector will sum to u64_max
            }
        }
        if_std! {
            println!("incentive: {:?} ", incentive);
            println!("consensus: {:?} ", consensus);
        }

        // Compute dividends.
        let mut total_dividends: I65F63 = I65F63::from_num( 0.0 );
        let mut dividends: Vec<I65F63> = vec![ I65F63::from_num( 0.0 ) ; n];
        let mut sparse_bonds: Vec<Vec<(u32,u64)>> = vec![vec![]; n];
        for uid_i in uids.iter() {

            // To be filled: Sparsified bonds.
            let mut sparse_bonds_row: Vec<(u32, u64)> = vec![];

            // Distribute dividends from self-ownership.
            let incentive_i: I65F63 = incentive[ *uid_i as usize ];
            let total_bonds_i: u64 = bond_totals[ *uid_i as usize ]; // Range( 0, total_emission );
            let mut dividends_ii: I65F63 = incentive_i * self_ownership;
            if total_bonds_i == 0 {
                dividends_ii += incentive_i * ( one - self_ownership ); // Add the other half.
            }
            dividends[ *uid_i as usize ] += dividends_ii; // Range( 0, block_emission / 2 );
            total_dividends += dividends_ii; // Range( 0, block_emission / 2 );

            // Distribute dividends from other-ownership.
            for uid_j in uids.iter() {
                
                // Get i -> j bonds.
                let bonds_ij: u64 = bonds[ *uid_i as usize ][ *uid_j as usize ]; // Range( 0, total_emission );
                let total_bonds_j: u64 = bond_totals[ *uid_j as usize ]; // Range( 0, total_emission );
                if total_bonds_j == 0 { continue; } // No bond ownership in this neuron.

                // Compute bond fraction.
                let bond_fraction_ij: I65F63 = I65F63::from_num( bonds_ij ) / I65F63::from_num( total_bonds_j ); // Range( 0, 1 );

                // Compute incentive owenership fraction.
                let mut ownership_ji: I65F63 = one - self_ownership; // Range( 0, 1 );
                ownership_ji = ownership_ji * bond_fraction_ij; // Range( 0, 1 );

                // Compute dividends
                let dividends_ji: I65F63 = incentive[ *uid_j as usize ] * ownership_ji; // Range( 0, 1 );
                dividends[ *uid_i as usize ] += dividends_ji; // Range( 0, block_emission / 2 );
                total_dividends += dividends_ji; // Range( 0, block_emission / 2 );
                sparse_bonds_row.push( (*uid_j as u32, bonds_ij) );
            }
            sparse_bonds[ *uid_i as usize ] = sparse_bonds_row;
        }
        // Normalize dividends. Sanity check.
        let mut total_emission: u64 = 0;
        let mut emission: Vec<u64> = vec![ 0; n];
        if total_dividends != 0 {
            for uid_i in uids.iter() {
                let dividends_i: I65F63 = dividends[ *uid_i as usize ] / total_dividends;
                let emission_i: u64 = (block_emission * dividends_i).to_num::<u64>();
                dividends[ *uid_i as usize ] = dividends_i;
                emission[ *uid_i as usize ] = emission_i;
                total_emission += emission_i;
            }
        }
        if_std! {
            println!( "dividends: {:?}", dividends );
            println!( "emission: {:?}", emission );
        }

        for ( uid_i, mut neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
            // Update table entry.
            neuron_i.active = active[ uid_i as usize ];
            neuron_i.priority = priority[ uid_i as usize ];
            neuron_i.emission = emission[ uid_i as usize ];
            neuron_i.stake = neuron_i.stake + emission[ uid_i as usize ];
            neuron_i.rank = (ranks[ uid_i as usize ] * u64_max).to_num::<u64>();
            neuron_i.trust = (trust[ uid_i as usize ] * u64_max).to_num::<u64>();
            neuron_i.consensus = (consensus[ uid_i as usize ] * u64_max).to_num::<u64>();
            neuron_i.incentive = (incentive[ uid_i as usize ] * u64_max).to_num::<u64>();
            neuron_i.dividends = (dividends[ uid_i as usize ] * u64_max).to_num::<u64>();
            neuron_i.bonds = sparse_bonds[ uid_i as usize ].clone();
            Neurons::<T>::insert( neuron_i.uid, neuron_i );
        }

        // Update totals.
        TotalEmission::<T>::set( total_emission );
        TotalBondsPurchased::<T>::set( total_bonds_purchased );
        TotalIssuance::<T>::mutate( |val| *val += total_emission );
        TotalStake::<T>::mutate( |val| *val += total_emission );
    }
    
    pub fn get_current_block_as_u64( ) -> u64 {
        let block_as_u64: u64 = TryInto::try_into( system::Pallet::<T>::block_number() ).ok().expect("blockchain will not exceed 2^64 blocks; QED.");
        block_as_u64
    }

}
