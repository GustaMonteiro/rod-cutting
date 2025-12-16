use crate::retrieve_best_cuts::display_best_cuts;

fn rod_cutting_memoization_impl(
    prices: &[u32],
    rod_length: u32,
    memo: &mut Vec<Option<u32>>,
    cuts: &mut Vec<u32>,
) -> u32 {
    if let Some(value) = memo[rod_length as usize] {
        return value;
    }

    let mut best: u32 = 0;

    for i in 1..=rod_length {
        let current = prices[(i - 1) as usize]
            + rod_cutting_memoization_impl(prices, rod_length - i, memo, cuts);

        if best < current {
            best = current;
            cuts[rod_length as usize] = i;
        }
    }

    memo[rod_length as usize] = Some(best);

    best
}

pub fn rod_cutting_memoization(prices: &[u32]) -> u32 {
    let rod_length = prices.len();
    let mut memo: Vec<Option<u32>> = vec![None; rod_length + 1];
    memo[0] = Some(0);

    let mut cuts = vec![0; rod_length + 1];

    let max_value = rod_cutting_memoization_impl(prices, rod_length as u32, &mut memo, &mut cuts);

    display_best_cuts(&cuts, rod_length as u32);

    max_value
}
