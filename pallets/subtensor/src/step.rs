use super::*;
use sp_std::if_std; // Import into scope the if_std! macro.
use sp_std::convert::TryInto;
use substrate_fixed::types::I65F63;
use substrate_fixed::transcendental::exp;
use frame_support::IterableStorageMap;

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
        
        // Get current block.
        let current_block: u64 = Self::get_current_block_as_u64(); 

        // Constants.
        let active_threshold: u64 = 10000;
        let u64_max: I65F63 = I65F63::from_num( u64::MAX );
        let u32_max: I65F63 = I65F63::from_num( u32::MAX );
        let one: I65F63 = I65F63::from_num( 1.0 );
        let rho: I65F63 = I65F63::from_num( 10.0 );
        let kappa: I65F63 = I65F63::from_num( 0.5 );
        let self_ownership: I65F63 = I65F63::from_num( 0.5 );
        let block_emission: I65F63 = I65F63::from_num( 1000000000 ); 

        // To be filled.
        let mut stake: Vec<u64> = vec![0;n];
        let mut rank: Vec<u64> = vec![0;n];
        let mut trust: Vec<u64> = vec![0;n];
        let mut consensus: Vec<u64> = vec![0;n];
        let mut incentive: Vec<u64> = vec![0;n];
        let mut inflation: Vec<u64> = vec![0;n];
        let mut dividends: Vec<u64> = vec![0;n];
        let mut bond_totals: Vec<u64> = vec![0; n];
        let mut bonds: Vec<Vec<u64>> = vec![vec![0;n]; n];
        let mut weights: Vec<Vec<(u32,u32)>> = vec![];
        let mut active_uids: Vec<u32> = vec![];
        let mut active: Vec<u64> = vec![0;n];

        // Pull active data into local cache.
        let mut total_active_stake: u64 = 0;
        for ( uid_i, neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {

            // Filter on active.
            if current_block - neuron_i.last_update < active_threshold {

                // Set as active.
                active [ uid_i as usize ] = 1;
                active_uids.push( uid_i );

                // Record stake.
                stake [ uid_i as usize ] = neuron_i.stake;
                total_active_stake += neuron_i.stake;

                // Save weights for later iteration.
                weights.push( neuron_i.weights );

                // Fill bonds matrix.
                let mut bonds_row: Vec<u64> = vec![0; n];
                for (uid_j, bonds_ij) in neuron_i.bonds.iter() {
                    bonds_row [ *uid_j as usize ] = *bonds_ij;
                    bond_totals [ *uid_j as usize ] += *bonds_ij;
                }
                bonds[ uid_i as usize ] = bonds_row;
            }
        }

        // Compute trust and ranks.
        let mut total_ranks: u64 = 0;
        let mut total_trust: u64 = 0;
        let mut total_bonds_purchased: u64 = 0;
        for (index_i, uid_i) in active_uids.iter().enumerate() {

            // Only accumulate rank, trust and bonds for active neurons.
            //let mut neuron_i: NeuronMetadataOf<T> = neurons[ uid_to_index[ *uid_i as usize ] as usize ];
            let stake_i: I65F63 = I65F63::from_num( stake[ *uid_i as usize ] );
            let weights_i: &Vec<(u32, u32)> = &weights[ index_i as usize ];

            // Iterate over weights.
            for ( uid_j, weight_ij ) in weights_i.iter() {

                // Self loop is ignored.
                if *uid_j == *uid_i { continue };
                // Only count weights from active to active.
                if active[ *uid_j as usize ] == 0 { continue };
                
                // Normalized weight from i to j.
                let weight_ij: I65F63 = I65F63::from_num( *weight_ij ) / u32_max;
                let trust_increment_ij: I65F63 = stake_i;
                let rank_increment_ij: I65F63 = stake_i * weight_ij;
                let bond_increment_ij: I65F63 = (rank_increment_ij * block_emission)/ I65F63::from_num( total_active_stake );
                if_std! {
                    println!("weight_ij: {:?} | trust_increment_ij: {:?} | rank_increment_ij: {:?} | bond_increment_ij: {:?}", weight_ij, trust_increment_ij, rank_increment_ij, bond_increment_ij);
                }

                // Increment neuron scores.
                rank[ *uid_j as usize ] += rank_increment_ij.to_num::<u64>();
                trust[ *uid_j as usize ] += trust_increment_ij.to_num::<u64>();
                total_ranks += rank_increment_ij.to_num::<u64>();
                total_trust += trust_increment_ij.to_num::<u64>();
                
                // Distribute bonds.
                bond_totals [ *uid_j as usize ] += bond_increment_ij.to_num::<u64>();
                bonds [ *uid_i as usize  ][ *uid_j as usize ] += bond_increment_ij.to_num::<u64>();
                total_bonds_purchased += bond_increment_ij.to_num::<u64>();
            }
        }
        if_std! {
            println!("ranks: {:?}, {:?}", rank, total_ranks);
            println!("trust: {:?}, {:?}", trust, total_trust);
            println!("bonds: {:?}, {:?}", bonds, bond_totals);
        }

        // Compute consensus, incentive, and inflation.
        let mut total_incentive: I65F63 = I65F63::from_num(0.0);
        if total_ranks != 0 && total_trust != 0 {
            for uid_i in active_uids.iter() {
                let rank_i: u64 = rank[ *uid_i as usize ];
                let trust_i: u64 = trust[ *uid_i as usize ];
                if trust_i != 0 {

                    // Consensus function.
                    let normalized_trust: I65F63 = I65F63::from_num( trust_i ) / I65F63::from_num( total_active_stake );
                    let shifted_trust: I65F63 = normalized_trust - kappa;
                    let temperatured_trust: I65F63 = shifted_trust * rho;
                    let exponentiated_trust: I65F63 = exp( -temperatured_trust ).expect( "temperatured_trust is on range(-rho, rho)");
                    let consensus_i: I65F63 = one / (one + exponentiated_trust);
                    if_std! {
                        println!("normalized_trust: {:?} | shifted_trust: {:?} | temperatured_trust: {:?} | exponentiated_trust: {:?} | consensus_i: {:?}", normalized_trust, shifted_trust, temperatured_trust, exponentiated_trust, consensus_i);
                    }

                    // Incentive function.
                    let normalized_rank: I65F63 = I65F63::from_num( rank_i ) / I65F63::from_num( total_ranks );
                    let incentive_i: I65F63 = normalized_rank * consensus_i;
                    total_incentive += incentive_i;

                    // Increment neuron scores.
                    let consensus_i: u64 = (consensus_i * u64_max).to_num::<u64>();
                    let incentive_i: u64 = (incentive_i * u64_max).to_num::<u64>();
                    consensus[ *uid_i as usize ] = consensus_i;
                    incentive[ *uid_i as usize ] = incentive_i;
                }
            }
        }
        if_std! {
            println!("consensus: {:?}", consensus);
            println!("incentive: {:?} {:?}", incentive, total_incentive);
        }

        // Compute consensus, incentive, and inflation.
        let mut total_inflation: u64 = 0;
        if total_incentive != 0 {
            for uid_i in active_uids.iter() {
                // Inflation function.
                let incentive_i: I65F63 = I65F63::from_num( incentive[ *uid_i as usize ] ) / u64_max;
                let inflation_i: I65F63 = (block_emission * incentive_i) / total_incentive;
                inflation[ *uid_i as usize ] = inflation_i.to_num::<u64>();
                total_inflation += inflation_i.to_num::<u64>();
                if_std! {
                    println!("incentive_i: {:?} | inflation_i: {:?}", incentive_i, inflation_i);
                }
            }
        }
        if_std! {
            println!("inflation: {:?}, {:?}", inflation, total_inflation);
        }

        // Compute trust and ranks.
        let mut total_dividends: u64 = 0;
        let mut sparse_bonds: Vec<Vec<(u32,u64)>> = vec![vec![]; n];
        for uid_i in active_uids.iter() {

            // To be filled: Sparsified bonds.
            let mut sparse_bonds_row: Vec<(u32, u64)> = vec![];

            // Only count bond dividends between active uids.
            for uid_j in active_uids.iter() {
                
                // Get denomenator.
                let bonds_ij: u64 = bonds[ *uid_i as usize ][ *uid_j as usize ];
                let total_bonds_j: u64 = bond_totals[ *uid_j as usize ];
                if total_bonds_j != 0 && bonds_ij != 0 {
                    // Get bonds from i to j.
                    sparse_bonds_row.push( (*uid_j as u32, bonds_ij) );

                    // Ownership fraction.
                    let ownership_fraction_ij: I65F63 = I65F63::from_num( bonds_ij ) / I65F63::from_num( total_bonds_j );

                    // Dividend from ownership.
                    let dividends_ji: I65F63 = (one - self_ownership) * ownership_fraction_ij * I65F63::from_num( inflation[ *uid_j as usize ] );

                    // Increment dividends.
                    dividends[ *uid_i as usize ] += dividends_ji.to_num::<u64>();
                    total_dividends += dividends_ji.to_num::<u64>();

                    if_std! {
                        println!("bonds_ij: {:?} | ownership_fraction_ij: {:?} | dividends_ji: {:?}", bonds_ij, ownership_fraction_ij, dividends_ji);
                    }
                }
            }
            // Fill sparse bonds row.
            if_std! {
                println!("sparse_bonds: {:?}", sparse_bonds_row );
            }
            sparse_bonds[ *uid_i as usize ] = sparse_bonds_row;
        }
        for uid_i in active_uids.iter() {
            let total_bonds_i: u64 = bond_totals[ *uid_i as usize ];
            let dividends_ii: u64;
            if total_bonds_i == 0 {
                dividends_ii = I65F63::from_num( inflation[ *uid_i as usize ] ).to_num::<u64>();
            } else {
                dividends_ii = (I65F63::from_num( inflation[ *uid_i as usize ] ) * self_ownership).to_num::<u64>();
            }
            dividends[ *uid_i as usize ] += dividends_ii;
            total_dividends += dividends_ii;
            stake[ *uid_i as usize ] += dividends[ *uid_i as usize ];

        }
        if_std! {
            println!("dividends: {:?}, {:?}", dividends, total_dividends);
        }

        for ( uid_i, mut neuron_i ) in <Neurons<T> as IterableStorageMap<u32, NeuronMetadataOf<T>>>::iter() {
            // Update table entry.
            if active[ uid_i as usize ] == 0 {
                neuron_i.active = 0;
                neuron_i.rank = 0;
                neuron_i.trust = 0;
                neuron_i.consensus = 0;
                neuron_i.inflation = 0;
                neuron_i.dividends = 0;
            } else {
                neuron_i.active = 1;
                neuron_i.stake = stake[ uid_i as usize ];
                neuron_i.rank = rank[ uid_i as usize ];
                neuron_i.trust = trust[ uid_i as usize ];
                neuron_i.consensus = consensus[ uid_i as usize ];
                neuron_i.incentive = incentive[ uid_i as usize ];
                neuron_i.inflation = inflation[ uid_i as usize ];
                neuron_i.dividends = dividends[ uid_i as usize ];
                neuron_i.bonds = sparse_bonds[ uid_i as usize ].clone();
            }
            Neurons::<T>::insert( neuron_i.uid, neuron_i );
        }

        // Update totals.
        TotalIssuance::<T>::mutate( |val| *val += total_dividends );
        TotalStake::<T>::mutate( |val| *val += total_dividends );
    }

    pub fn get_current_block_as_u64( ) -> u64 {
        let block_as_u64: u64 = TryInto::try_into( system::Pallet::<T>::block_number() ).ok().expect("blockchain will not exceed 2^64 blocks; QED.");
        block_as_u64
    }

}



