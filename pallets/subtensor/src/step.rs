use super::*;
use substrate_fixed::types::I65F63;
use substrate_fixed::transcendental::exp;

impl<T: Config> Pallet<T> {

    /// Block setup: Computation performed on the initialize of a block. We perform the entirety of the incentive mechanism computation. Performing the following operations.
    /// 1) Computation of Active Stake. 
    ///     -- Stake becomes stale if the owning peer has not performed an subscription of a weight set operation with x blocks.
    ///     We compute the active set A. All computations past this point are assumed to be computed over the active set. 
    ///
    /// 2) Computation of Ranks. 
    ///     -- The ranks vector is given by the weight matrix multiplication R = (W^T * S) / total_stake
    ///     -- This is the stake normalized score for each peer in the system
    /// 
    /// 3) Compute Trust scores.
    ///     -- Trust is calculated as the proportion of stake in the network which has an inward edge to this peer which is non-zero.
    ///     -- We compute a trust score for each peer by enumerating this edge set using a linear pass over the entire weight matrix 
    ///         and accumulating the number of non-zero edges to peer i multiplied by their stake.__rust_force_expr!
    ///     -- t_i = sum j (s_j/total_stake) iff w_ji != 0 else 0
    ///     -- t_i is in range [0, 1], 1 is all stake is connected, 0 is no stake is connected.
    /// 
    /// 4) Computation of stake inflation.
    ///     -- We compute inflation by translating the Rank vector. R * (exp(t_i) - exp(0.0))
    ///     -- We normalize the inflation vector SUM I = 1
    ///     -- Peers attain as inflation equivalent to i_i * tau
    /// 
    ///
    pub fn block_step () {

        // Pull state.
        // DB Reads/Writes: O(1), Decoding: O(1), Operations: O(1)
        let n: usize = Self::get_next_uid() as usize;
        let temperature: I65F63 = I65F63::from_num(75.0);
        let block_emission: I65F63 = I65F63::from_num(1.0); 
        let stake_total: I65F63 = I65F63::from_num( Self::get_total_stake() );

        // Allocate sums.
        // DB Reads/Writes: O(0), Decoding: O(0), Operations: O(1)
        let mut ranks_total: I65F63 = I65F63::from_num( 0.0 );
        let mut trust_total: I65F63 = I65F63::from_num( 0.0 );
        let mut incentive_total: I65F63 = I65F63::from_num( 0.0 );
        let mut inflation_total: I65F63 = I65F63::from_num( 0.0 );
        let mut dividends_total: I65F63 = I65F63::from_num( 0.0 );

        // Allocate Vectors.
        // DB Reads/Writes: O(0), Decoding: O(0), Operations: O(n)
        let mut ranks: Vec<I65F63> = vec![]; 
        let mut trust: Vec<I65F63> = vec![]; 
        let mut incentive: Vec<I65F63> = vec![];
        let mut inflation: Vec<I65F63> = vec![];
        let mut dividends: Vec<I65F63> = vec![];
        for _ in 1..n {
            let r_i = I65F63::from_num( 0.0 );
            let t_i = I65F63::from_num( 0.0 );
            let inc_i = I65F63::from_num( 0.0 );
            let inf_i = I65F63::from_num( 0.0 );
            let div_i = I65F63::from_num( 0.0 );
            ranks.push( r_i );
            trust.push( t_i );
            incentive.push( inc_i );
            inflation.push( inf_i );
            dividends.push( div_i );
        }

        // Compute Rank, Trust and Bonds scores.
        // DB Reads/Writes: O( n^2 ), Decoding: O( n^2 ), Operations: O( n^2 )
        // r_i = SUM(j) s_j * w_ji
        // t_i = SUM(j) s_j if w_ji != 0
        // b_ij = w_ij * s_i
        for uid_i in 0..n {
            let stake_i: u64 = Stake::<T>::get( uid_i as u64 );
            let weight_uids_i: Vec<u64> = WeightUids::<T>::get( uid_i as u64 ); 
            let weight_vals_i: Vec<u32> = WeightVals::<T>::get( uid_i as u64 ); 
            let trust_increment_ij: I65F63 = I65F63::from_num( stake_i ) / stake_total; 
            for ( index, uid_j ) in weight_uids_i.iter().enumerate() {
                let converted_uid_j: usize = *uid_j as usize;
                let normalize_weights_ij: I65F63 = I65F63::from_num( weight_vals_i[ index ] ) / I65F63::from_num( u32::MAX as f64 );
                let rank_increment_ij: I65F63 = trust_increment_ij * normalize_weights_ij;
                let converted_rank_increment_ij: u64 = rank_increment_ij.to_num::<u64>();
                ranks[ converted_uid_j ] += rank_increment_ij;
                trust[ converted_uid_j ] += trust_increment_ij;
                Bonds::<T>::mutate( uid_i as u64, *uid_j as u64, |el| *el += converted_rank_increment_ij );
                BondTotals::<T>::mutate( *uid_j as u64, |el| *el += converted_rank_increment_ij );
                ranks_total += rank_increment_ij;
                trust_total += trust_increment_ij;
            }
        }

        // Compute Incentive
        // DB Reads/Writes: O( 0 ), Decoding: O( 0 ), Operations: O( n )
        // Inc = R * (exp( T * temperature ) - 1)
        for uid_i in 0..n {
            // Exponentiate normalized Trust
            let normalized_trust: I65F63 = trust [ uid_i ] / stake_total;
            let temperatured_trust: I65F63 = normalized_trust * temperature;
            let exponentiated_trust: I65F63 = exp( temperatured_trust ).expect( "trust is on range(0,1)");
            let incentive_i: I65F63 = ranks[ uid_i ] * ( exponentiated_trust - I65F63::from_num(1.0) );
            incentive[ uid_i ] = incentive_i;
            incentive_total += incentive_i;
        }

        // Compute Inflation
        // DB Reads/Writes: O( 0 ), Decoding: O( 0 ), Operations: O( n )
        // Inf_i = Inc_i * tau
        for uid_i in 0..n {
            let incentive_fraction: I65F63 = incentive[ uid_i ] / incentive_total;
            let inflation_i: I65F63 = incentive_fraction * block_emission;
            inflation[ uid_i ] = inflation_i;
            inflation_total += inflation_i;
        }

        // Compute dividends.
        // DB Reads/Writes: O( n^2 ), Decoding: O( n^2 ), Operations: O( n^2 )
        // d_i = SUM(j) b_ij * Inf_j
        for uid_i in 0..n {
            for uid_j in 0..n {
                let bond_total_j: u64 = BondTotals::<T>::get( uid_j as u64 );
                let bonds_ij: u64 = Bonds::<T>::get( uid_i as u64, uid_j as u64 );
                let bond_fraction_ij: I65F63 = I65F63::from( bonds_ij ) / I65F63::from( bond_total_j );
                let inflation_j: I65F63 = inflation[ uid_i ];
                let dividends_i: I65F63 = bond_fraction_ij * inflation_j;
                dividends[ uid_i ] = dividends_i;
                dividends_total += dividends_i;
            }
        }

        // Distribute Dividends
        // DB Reads/Writes: O( n ), Decoding: O( n ), Operations: O( n )
        // s_i = s_i + d_i
        for uid_i in 0..n {
            let dividends_i: I65F63 = dividends[ uid_i as usize ];
            let converted_dividends_i:u64 = dividends_i.to_num::<u64>();
            // Stake::<T>::mutate( uid_i as u64, |el| *el += converted_dividends_i );
        }

    }

}



