pub fn get_best_cuts(cuts: &[u32], rod_length: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut length = rod_length;

    while length > 0 {
        result.push(cuts[length as usize]);
        length -= cuts[length as usize];
    }

    result
}

pub fn display_best_cuts(cuts: &[u32], rod_length: u32) {
    let best_cuts = get_best_cuts(cuts, rod_length);

    if best_cuts.len() > 30 {
        println!(
            "(Displaying first 30 cuts of total {} cuts)",
            best_cuts.len()
        );

        println!("Best cuts: {:?}", &best_cuts[..30]);
        return;
    }

    println!("Best cuts: {:?}", best_cuts);
}
