mod dynamic_programming;
mod memoization;
mod recursive;
mod tests;

use crate::dynamic_programming::rod_cutting_dp;
use crate::memoization::rod_cutting_memoization;
use crate::recursive::rod_cutting_recursive;
use crate::tests::test_rod_cutting_variant;

fn main() {
    test_rod_cutting_variant("Recursive", rod_cutting_recursive);
    test_rod_cutting_variant("Memoization", rod_cutting_memoization);
    test_rod_cutting_variant("Dynamic Programming", rod_cutting_dp);
}
