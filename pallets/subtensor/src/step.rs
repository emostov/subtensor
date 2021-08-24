use super::*;
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


        // Pull state.
        // DB Reads/Writes: O(1), Decoding: O(1), Operations: O(1)
        let n: usize = Self::get_neuron_count() as usize;
        let temperature: I65F63 = I65F63::from_num(75.0);
        let block_emission: I65F63 = I65F63::from_num(1000000000); 
        let stake_total: I65F63 = I65F63::from_num( Self::get_total_stake() );

        // The network is initialized by the first staking operation. 
        // Otherwise there is no token inflation.TotalStake
        if Self::get_total_stake() == 0 {
            return 
        }
        
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
        let mut ranks_u64: Vec<u64> = vec![0; n]; 
        let mut trust_u64: Vec<u64> = vec![0; n]; 
        let mut incentive_u64: Vec<u64> = vec![0; n];
        let mut inflation_u64: Vec<u64> = vec![0; n];
        let mut dividends_u64: Vec<u64> = vec![0; n];
        for _ in 0..n {
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
        // // Optional Debug.
        // for i in 0..n {
        //     println!("i:{:} r:{:} t:{:} inc:{:} inf:{:} div:{:}", i, Self::pr(ranks[i]), Self::pr(trust[i]), Self::pr(incentive[i]), Self::pr(inflation[i]), Self::pr(dividends[i]));
        // }

        // Compute Rank + Trust scores and Bond updates.
        // DB Reads/Writes: O( n^2 ), Decoding: O( n^2 ), Operations: O( n^2 )
        // r_i = SUM(j) s_j * w_ji
        // t_i = SUM(j) s_j if w_ji != 0
        // b_ij = w_ij * s_i
        for uid_i in 0..n {
            let stake_i: u64 = Stake::<T>::get( uid_i as u64 );
            let weight_uids_i: Vec<u64> = WeightUids::<T>::get( uid_i as u64 ); 
            let weight_vals_i: Vec<u32> = WeightVals::<T>::get( uid_i as u64 ); 
            let trust_increment_ij: I65F63 = I65F63::from_num( stake_i ); 
            for ( index, uid_j ) in weight_uids_i.iter().enumerate() {
                let converted_uid_j: usize = *uid_j as usize;
                let normalize_weights_ij: I65F63 = I65F63::from_num( weight_vals_i[ index ] ) / I65F63::from_num( u32::MAX as f64 );
                let rank_increment_ij: I65F63 = trust_increment_ij * normalize_weights_ij;
                let converted_rank_increment_ij: u64 = rank_increment_ij.to_num::<u64>();
                ranks[ converted_uid_j ] += rank_increment_ij;
                trust[ converted_uid_j ] += trust_increment_ij;
                ranks_u64[ converted_uid_j ] += rank_increment_ij.to_num::<u64>();
                trust_u64[ converted_uid_j ] += trust_increment_ij.to_num::<u64>();
                Bonds::<T>::mutate( uid_i as u64, *uid_j as u64, |el| *el += converted_rank_increment_ij );
                BondTotals::<T>::mutate( *uid_j as u64, |el| *el += converted_rank_increment_ij );
                ranks_total += rank_increment_ij;
                trust_total += trust_increment_ij;
                // println!("uid_i:{:} uid_j:{:} stake:{:} trust_increment_ij:{:} normalize_weights_ij:{:} converted_rank_increment_ij:{:}", uid_i, converted_uid_j, stake_i, Self::pr(trust_increment_ij), Self::pr(normalize_weights_ij), converted_rank_increment_ij);
            }
        }
        // // Optional Debug.
        // for i in 0..n {
        //     println!("i:{:} r:{:} t:{:} inc:{:} inf:{:} div:{:}", i, Self::pr(ranks[i]), Self::pr(trust[i]), Self::pr(incentive[i]), Self::pr(inflation[i]), Self::pr(dividends[i]));
        // }

        // Compute Incentive
        // DB Reads/Writes: O( 0 ), Decoding: O( 0 ), Operations: O( n )
        // Inc = R * (exp( T * temperature ) - 1)
        for uid_i in 0..n {
            // Exponentiate normalized Trust
            let normalized_trust: I65F63 = trust [ uid_i ] / (temperature * stake_total);
            let temperatured_trust: I65F63 = normalized_trust * temperature;
            let exponentiated_trust: I65F63 = exp( temperatured_trust ).expect( "temperatured_trust is on range(0, temperature)");
            let incentive_i: I65F63 = ranks[ uid_i ] * ( exponentiated_trust - I65F63::from_num(1.0) );
            // println!("uid_i:{:} trust:{:} stake_total:{:} normalized_trust:{:} temperatured_trust:{:} exponentiated_trust:{:}, incentive_i:{:}", uid_i, Self::pr(trust[uid_i]), Self::pr(stake_total), Self::pr(normalized_trust * 1000), Self::pr(temperatured_trust*1000), Self::pr(exponentiated_trust), Self::pr(incentive_i));
            incentive[ uid_i ] = incentive_i;
            incentive_u64[ uid_i ] += incentive_i.to_num::<u64>();
            incentive_total += incentive_i;
        }
        // // Optional Debug.
        // for i in 0..n {
        //     println!("i:{:} r:{:} t:{:} inc:{:} inf:{:} div:{:}", i, Self::pr(ranks[i]), Self::pr(trust[i]), Self::pr(incentive[i]), Self::pr(inflation[i]), Self::pr(dividends[i]));
        // }

        // Compute Inflation
        // DB Reads/Writes: O( 0 ), Decoding: O( 0 ), Operations: O( n )
        // Inf_i = Inc_i * tau
        for uid_i in 0..n {
            let incentive_fraction: I65F63 = incentive[ uid_i ] / incentive_total;
            let inflation_i: I65F63 = incentive_fraction * block_emission;
            // println!("uid_i:{:} incentive:{:} incentive_total:{:} incentive_fraction:{:} inflation_i:{:}", uid_i, Self::pr(incentive[ uid_i ]), Self::pr(incentive_total), Self::pr(incentive_fraction), Self::pr(inflation_i));
            inflation[ uid_i ] = inflation_i;
            inflation_u64[ uid_i ] += inflation_i.to_num::<u64>();
            inflation_total += inflation_i;
        }
        // // Optional Debug.
        // for i in 0..n {
        //     println!("i:{:} r:{:} t:{:} inc:{:} inf:{:} div:{:}", i, Self::pr(ranks[i]), Self::pr(trust[i]), Self::pr(incentive[i]), Self::pr(inflation[i]), Self::pr(dividends[i]));
        // }

        // Compute dividends.
        // DB Reads/Writes: O( n^2 ), Decoding: O( n^2 ), Operations: O( n^2 )
        // d_i = SUM(j) b_ij * Inf_j
        for uid_i in 0..n {
            for uid_j in 0..n {
                let mut bond_fraction_ij: I65F63 = I65F63::from_num( 0.0 );
                let bond_total_j: u64 = BondTotals::<T>::get( uid_j as u64 );
                let bonds_ij: u64 = Bonds::<T>::get( uid_i as u64, uid_j as u64 );
                if bond_total_j != 0 {
                    bond_fraction_ij = ( I65F63::from_num( bonds_ij ) )/ I65F63::from_num( bond_total_j );
                }
                let inflation_j: I65F63 = inflation[ uid_j ];
                let mut dividends_i: I65F63 = bond_fraction_ij * ( inflation_j / 2 );
                if uid_i == uid_j {
                    dividends_i += inflation_j / 2;
                }
                dividends[ uid_i ] += dividends_i;
                dividends_u64[ uid_i ] += dividends_i.to_num::<u64>();
                dividends_total += dividends_i;
                // println!("uid_i:{:} uid_j:{:} bond_total_j:{:} bonds_ij:{:} bond_fraction_ij:{:} inflation_j:{:} dividends_i:{:}", uid_i, uid_j, bond_total_j, bonds_ij,  Self::pr(bond_fraction_ij),  Self::pr(inflation_j),  Self::pr(dividends_i));
            }
        }
        // // Optional Debug.
        // for i in 0..n {
        //     println!("i:{:} r:{:} t:{:} inc:{:} inf:{:} div:{:}", i, Self::pr(ranks[i]), Self::pr(trust[i]), Self::pr(incentive[i]), Self::pr(inflation[i]), Self::pr(dividends[i]));
        // }

        // Distribute Dividends
        // DB Reads/Writes: O( n ), Decoding: O( n ), Operations: O( n )
        // s_i = s_i + d_i
        for uid_i in 0..n {
            let dividends_i: I65F63 = dividends[ uid_i as usize ];
            let converted_dividends_i:u64 = dividends_i.to_num::<u64>();
            Stake::<T>::mutate( uid_i as u64, |el| *el += converted_dividends_i );
            // println!("uid_i:{:} dividends_i:{:}", uid_i,  Self::pr(dividends_i));
        }

        // Update new total.
        TotalStake::<T>::mutate( |el| *el += dividends_total.to_num::<u64>() );

        // Set vectors.
        Ranks::<T>::set( ranks_u64 );
        Trust::<T>::set( trust_u64 );
        Incentive::<T>::set( incentive_u64 );
        Inflation::<T>::set( inflation_u64 );
        Dividends::<T>::set( dividends_u64 );

    }

}



