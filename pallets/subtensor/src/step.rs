use super::*;
use sp_std::convert::TryInto;
use substrate_fixed::types::I65F63;
use substrate_fixed::transcendental::exp;

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
    /// 
    pub fn pr ( value: I65F63 ) -> u64 {
        // let str1: std::string::String = (value * 1000).to_num::<u64>().to_string();
        // let str_vec: Vec<char> = str1.chars().collect();
        // let str2: std::string::String = str_vec.into_iter().collect();
        return value.to_num::<u64>()
    }

    pub fn block_step () {

        // Number of peers.
        let n: usize = N::<T>::get() as usize;
        
        // Get current block.
        let current_block: u64 = Self::get_current_block_as_u64(); 

        // Constants.
        let active_set_cliff: u64 = 10000;
        let one: I65F63 = I65F63::from_num( 1.0 );
        let rho: I65F63 = I65F63::from_num( 10.0 );
        let kappa: I65F63 = I65F63::from_num( 0.5 );
        let self_ownership: I65F63 = I65F63::from_num( 0.5 );
        let block_emission: I65F63 = I65F63::from_num( 1000000000 ); 
        
        // Allocate sums.
        // DB Reads/Writes: O(0), Decoding: O(0), Operations: O(1)
        let mut ranks_total: I65F63 = I65F63::from_num( 0.0 );
        let mut trust_total: I65F63 = I65F63::from_num( 0.0 );
        let mut bonds_total: I65F63 = I65F63::from_num( 0.0 );
        let mut consensus_total: I65F63 = I65F63::from_num( 0.0 );
        let mut incentive_total: I65F63 = I65F63::from_num( 0.0 );
        let mut inflation_total: I65F63 = I65F63::from_num( 0.0 );
        let mut dividends_total: I65F63 = I65F63::from_num( 0.0 );

        // Allocate Vectors.
        // DB Reads/Writes: O(0), Decoding: O(0), Operations: O(n)
        let mut ranks: Vec<I65F63> = vec![]; 
        let mut trust: Vec<I65F63> = vec![]; 
        let mut consensus: Vec<I65F63> = vec![];
        let mut incentive: Vec<I65F63> = vec![];
        let mut inflation: Vec<I65F63> = vec![];
        let mut dividends: Vec<I65F63> = vec![];
        let mut ranks_u64: Vec<u64> = vec![0; n]; 
        let mut trust_u64: Vec<u64> = vec![0; n]; 
        let mut consensus_u64: Vec<u64> = vec![0; n]; 
        let mut incentive_u64: Vec<u64> = vec![0; n];
        let mut inflation_u64: Vec<u64> = vec![0; n];
        let mut dividends_u64: Vec<u64> = vec![0; n];
        let mut active_u64: Vec<u64> = vec![0; n];
        for _ in 0..n {
            let r_i = I65F63::from_num( 0.0 );
            let t_i = I65F63::from_num( 0.0 );
            let c_i = I65F63::from_num( 0.0 );
            let inc_i = I65F63::from_num( 0.0 );
            let inf_i = I65F63::from_num( 0.0 );
            let div_i = I65F63::from_num( 0.0 );
            ranks.push( r_i );
            trust.push( t_i );
            consensus.push( c_i );
            incentive.push( inc_i );
            inflation.push( inf_i );
            dividends.push( div_i );
        }

        // Active set.
        let mut total_active_stake: u64 = 0;
        let mut active_uids: Vec<u64>  = vec![];
        let mut active_stake: Vec<u64> = vec![];
        for uid_i in 0..n {

            // Check if uid is active.
            let last_emit: u64 = Self::get_last_emit_for_uid( uid_i as u64 );
            if current_block - last_emit < active_set_cliff {

                // Get stake for active uid.
                let stake_i: u64 = Stake::<T>::get( uid_i as u64 );

                // Append values.
                active_uids.push( uid_i as u64 );
                active_stake.push( stake_i );
                active_u64 [ uid_i ] = 1;

                // Increment totals.
                total_active_stake += stake_i;
                total_active_uids += 1;
            }
        }

        // Get state.
        let stake: Vec<u64> = Self::get_stake();
        let bonds: Vec<Vec<(u64,u64)>> = Self::get_bonds();
        let weights: Vec<Vec<u64,u32>> = Self::get_weights();

        // Compute Rank + Trust scores and Bond updates.
        // DB Reads/Writes: O( n^2 ), Decoding: O( n^2 ), Operations: O( n^2 )
        // r_i = SUM(j) s_j * w_ji
        // t_i = SUM(j) s_j if w_ji != 0
        // b_ij = w_ij * s_i
        let mut total_distributed_stake: I65F63 = I65F63::from_num( 0.0 );
        if total_active_stake != 0 {
            for (index_i, uid_i) in active_uids.iter().enumerate() {
                let uid_i: u64 = *uid_i;
                
                // Get stake + get weights. 
                let stake_i: I65F63 = I65F63::from_num( active_stake[ index_i ]);
                let trust_increment_ij: I65F63 = stake_i;
                let weight_uids_i: Vec<u64> = WeightUids::<T>::get( uid_i as u64 ); 
                let weight_vals_i: Vec<u32> = WeightVals::<T>::get( uid_i as u64 ); 

                for ( index_j, uid_j ) in weight_uids_i.iter().enumerate() {
                    let uid_j: u64 = *uid_j;
                    
                    // Normalized weight from i to j.
                    let weight_ij: I65F63 = I65F63::from_num( weight_vals_i[ index_j ] ) / I65F63::from_num( u32::MAX as f64 );

                    // Compute increments.
                    let rank_increment_ij: I65F63 = stake_i * weight_ij;
                    let bond_increment_ij: I65F63 = rank_increment_ij / I65F63::from_num( total_active_stake );

                    // Self loop does not add to rank or purchase bonds.
                    if uid_i == uid_j {
                        continue
                    }
                    
                    // Increment rank.
                    ranks[ uid_j as usize ] += rank_increment_ij;
                    ranks_u64[ uid_j as usize ] += rank_increment_ij.to_num::<u64>();

                    // Increment trust.
                    trust[ uid_j as usize ] += stake_i;
                    trust_u64[ uid_j as usize ] += trust_increment_ij.to_num::<u64>();
                    
                    // Increment bonds.
                    Bonds::<T>::mutate( uid_i, uid_j, |el| *el += bond_increment_ij.to_num::<u64>() );
                    BondTotals::<T>::mutate( uid_j, |el| *el += bond_increment_ij.to_num::<u64>() );

                    // Increment totals.
                    ranks_total += rank_increment_ij;
                    bonds_total += bond_increment_ij;
                    trust_total += trust_increment_ij;
                    total_distributed_stake += rank_increment_ij;
                }
            }
        }

        // Compute Incentive
        // DB Reads/Writes: O( 0 ), Decoding: O( 0 ), Operations: O( n )
        // C = 1 / ( 1 + e^(-rho(t - kappa)))
        if total_distributed_stake != 0 {
            for uid_i in 0..n {

                // 0 trusted peers attain no incentive or consensus.
                if trust [ uid_i ] == 0 {
                    continue;
                }

                // Trust threshold function.
                let normalized_trust: I65F63 = trust [ uid_i ] / total_distributed_stake;
                let shifted_trust: I65F63 = normalized_trust - kappa;
                let temperatured_trust: I65F63 = shifted_trust * rho;
                let exponentiated_trust: I65F63 = exp( -temperatured_trust ).expect( "temperatured_trust is on range(-rho, rho)");

                // Compute consensus score.
                let consensus_i: I65F63 = one / (one + exponentiated_trust);

                // Incentive function.
                let incentive_i: I65F63 = ranks[ uid_i ] * consensus_i;

                // Increment totals.
                incentive_u64[ uid_i ] += incentive_i.to_num::<u64>();
                consensus_u64[ uid_i ] += consensus_i.to_num::<u64>();

                incentive[ uid_i ] = incentive_i;
                consensus[ uid_i ] = consensus_i;

                incentive_total += incentive_i;
                consensus_total += consensus_i;
            }
        }
    
        // Compute Inflation
        // DB Reads/Writes: O( 0 ), Decoding: O( 0 ), Operations: O( n )
        // Inf_i = Inc_i * tau
        if incentive_total != 0 {
            for uid_i in 0..n {

                // Compute inflation.
                let incentive_fraction: I65F63 = incentive[ uid_i ] / incentive_total;
                let inflation_i: I65F63 = block_emission * incentive_fraction;

                // Increments totals.
                inflation_u64[ uid_i ] += inflation_i.to_num::<u64>();
                inflation[ uid_i ] = inflation_i;
                inflation_total += inflation_i;
            }
        }

        // Compute dividends.
        // DB Reads/Writes: O( n^2 ), Decoding: O( n^2 ), Operations: O( n^2 )
        // d_i = SUM(j) b_ij * Inf_j
        if inflation_total != 0 {
            for uid_j in 0..n {
                
                // Get inflation into peer i.
                let inflation_j: I65F63 = inflation[ uid_j ];
                let total_bonds_j: u64 = BondTotals::<T>::get( uid_j as u64 );
                
                // Compute self ownership dividends. 
                let dividends_j: I65F63 = inflation_j * self_ownership;

                // Increment self ownership.
                dividends_u64[ uid_j ] += dividends_j.to_num::<u64>();
                dividends[ uid_j ] += dividends_j;
                dividends_total += dividends_j;

                if total_bonds_j == 0 {
                    // Distribute dividends to owner if there are no bond holders.
                    let remaining_dividends: I65F63 = inflation_j * (one - self_ownership);
                    dividends_u64[ uid_j ] += remaining_dividends.to_num::<u64>();
                    dividends[ uid_j ] += remaining_dividends;
                    dividends_total += remaining_dividends;

                    // There are no bonds owned in j.
                    continue
                }
                // Iterate over all other peers and get dividends.
                for uid_i in 0..n {

                    // Get bonds owned in j by i.
                    let bonds_ij: u64 = Bonds::<T>::get( uid_i as u64, uid_j as u64 );
                    if bonds_ij == 0  {
                        // There are no bonds owned by i in j.
                        continue
                    }

                    // Compute bond ownership fraction.
                    let ownership_fraction_ij: I65F63 = ( I65F63::from_num( bonds_ij ) ) / I65F63::from_num( total_bonds_j );

                    // Compute dividends for i from j.
                    // Take ownership from 50% of inflation.
                    let dividends_ij: I65F63 = ownership_fraction_ij * inflation_j * (one - self_ownership);

                    dividends_u64[ uid_i ] += dividends_ij.to_num::<u64>();
                    dividends[ uid_i ] += dividends_ij;
                    dividends_total += dividends_ij;
                }
            }
        } 

        // Distribute Dividends
        // DB Reads/Writes: O( n ), Decoding: O( n ), Operations: O( n )
        // s_i = s_i + d_i
        if dividends_total != 0 {
            for uid_i in 0..n {
                // Get uid_i dividends
                let dividends_i: I65F63 = dividends[ uid_i as usize ];

                // Get dividends as u64 increment.
                let converted_dividends_i:u64 = dividends_i.to_num::<u64>();

                // Increment stake vector.
                Stake::<T>::mutate( uid_i as u64, |el| *el += converted_dividends_i );
            }
        }

        // Update new total stake ammount.
        TotalStake::<T>::mutate( |el| *el += dividends_total.to_num::<u64>() );

        // Update new total issuance.
        TotalIssuance::<T>::mutate( |el| *el += dividends_total.to_num::<u64>() );

        // Set vectors.
        Ranks::<T>::set( ranks_u64 );
        Trust::<T>::set( trust_u64 );
        Active::<T>::set( active_u64 );
        Consensus::<T>::set( consensus_u64 );
        Incentive::<T>::set( incentive_u64 );
        Inflation::<T>::set( inflation_u64 );
        Dividends::<T>::set( dividends_u64 );
    }

    pub fn get_current_block_as_u64( ) -> u64 {
        let block_as_u64: u64 = TryInto::try_into( system::Pallet::<T>::block_number() ).ok().expect("blockchain will not exceed 2^64 blocks; QED.");
        block_as_u64
    }

}



