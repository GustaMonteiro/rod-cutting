fn rod_cutting_memoization_impl(
    prices: &[u32],
    rod_length: u32,
    memo: &mut Vec<Option<u32>>,
) -> u32 {
    if let Some(value) = memo[rod_length as usize] {
        return value;
    }

    let mut best: u32 = 0;

    for i in 1..=rod_length {
        best = std::cmp::max(
            best,
            prices[(i - 1) as usize] + rod_cutting_memoization_impl(prices, rod_length - i, memo),
        );
    }

    memo[rod_length as usize] = Some(best);

    best
}

pub fn rod_cutting_memoization(prices: &[u32]) -> u32 {
    let rod_length = prices.len();
    let mut memo: Vec<Option<u32>> = vec![None; rod_length + 1];
    memo[0] = Some(0);
    rod_cutting_memoization_impl(prices, rod_length as u32, &mut memo)
}
