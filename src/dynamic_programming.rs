use crate::retrieve_best_cuts::display_best_cuts;

fn rod_cutting_dp_impl(prices: &[u32], rod_length: u32, cuts: &mut Vec<u32>) -> u32 {
    let mut dp: Vec<u32> = vec![0; (rod_length + 1) as usize];

    for i in 1..=rod_length as usize {
        for j in 1..=i {
            let current = prices[(j - 1) as usize] + dp[i - j];

            if dp[i] < current {
                dp[i] = current;
                cuts[i] = j as u32;
            }
        }
    }

    dp[rod_length as usize]
}

pub fn rod_cutting_dp(prices: &[u32]) -> u32 {
    let rod_length = prices.len();
    let mut cuts = vec![0; rod_length + 1];
    let max_value = rod_cutting_dp_impl(prices, rod_length as u32, &mut cuts);
    display_best_cuts(&cuts, rod_length as u32);
    max_value
}
