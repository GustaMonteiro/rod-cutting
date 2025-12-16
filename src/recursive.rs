use crate::retrieve_best_cuts::display_best_cuts;

fn rod_cutting_recursive_impl(prices: &[u32], rod_length: u32, cuts: &mut Vec<u32>) -> u32 {
    if rod_length == 0 {
        return 0;
    }

    let mut best: u32 = 0;

    for i in 1..=rod_length {
        let current =
            prices[(i - 1) as usize] + rod_cutting_recursive_impl(prices, rod_length - i, cuts);

        if current > best {
            best = current;
            cuts[rod_length as usize] = i;
        }
    }

    best
}

pub fn rod_cutting_recursive(prices: &[u32]) -> u32 {
    let rod_length = prices.len();
    let mut cuts = vec![0; rod_length + 1];
    let max_value = rod_cutting_recursive_impl(prices, rod_length as u32, &mut cuts);
    display_best_cuts(&cuts, rod_length as u32);
    max_value
}
