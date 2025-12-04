fn rod_cutting_dp_impl(prices: &[u32], rod_length: u32) -> u32 {
    let mut dp: Vec<u32> = vec![0; (rod_length + 1) as usize];

    for i in 1..=rod_length as usize {
        for j in 1..=i {
            dp[i] = std::cmp::max(dp[i], dp[i - j] + prices[j - 1]);
        }
    }

    dp[rod_length as usize]
}

pub fn rod_cutting_dp(prices: &[u32]) -> u32 {
    let rod_length = prices.len();
    rod_cutting_dp_impl(prices, rod_length as u32)
}
