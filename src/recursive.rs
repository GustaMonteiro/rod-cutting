fn rod_cutting_recursive_impl(prices: &[u32], rod_length: u32) -> u32 {
    if rod_length == 0 {
        return 0;
    }

    let mut best: u32 = 0;

    for i in 1..=rod_length {
        best = std::cmp::max(
            best,
            prices[(i - 1) as usize] + rod_cutting_recursive_impl(prices, rod_length - i),
        );
    }

    best
}

pub fn rod_cutting_recursive(prices: &[u32]) -> u32 {
    let rod_length = prices.len();
    rod_cutting_recursive_impl(prices, rod_length as u32)
}
