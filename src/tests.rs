pub struct TestCase {
    pub input: Vec<u32>,
    pub expected: u32,
}

pub fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: vec![],
            expected: 0,
        },
        TestCase {
            input: vec![1, 5, 8, 9, 10, 17, 17, 20],
            expected: 22,
        },
        TestCase {
            input: vec![3, 5, 8, 9, 10, 17, 17, 20],
            expected: 24,
        },
        TestCase {
            input: vec![1, 5, 8, 9],
            expected: 10,
        },
        TestCase {
            input: vec![1, 5, 8, 9, 10],
            expected: 13,
        },
        TestCase {
            input: vec![1, 1, 1, 1, 100],
            expected: 100,
        },
        TestCase {
            input: vec![10],
            expected: 10,
        },
        TestCase {
            input: vec![3, 4, 4, 5],
            expected: 12,
        },
        TestCase {
            input: vec![2, 5, 9, 9],
            expected: 11,
        },
        // LARGE CASE 1: n = 30
        // Logic: Price equals length (e.g., length 1 = 1, length 2 = 2).
        // Optimal strategy: Keep the rod as is (or any combination, sum remains 30).
        // Purpose: Benchmarking naive recursion vs. DP. Naive recursion struggles significantly at n >= 30.
        TestCase {
            input: (1..=30).collect(),
            expected: 30,
        },
        // LARGE CASE 2: n = 2000
        // Logic: Length 1 has a high value (5), all other lengths have value 1.
        // Input generation: [5, 1, 1, 1, ..., 1]
        // Optimal strategy: Cut the entire rod into 2000 pieces of length 1.
        // Calculation: 2000 * 5 = 10,000.
        // Purpose: Forces the algorithm to evaluate deep recursion or many iterations favoring small cuts.
        TestCase {
            input: (0..2000).map(|i| if i == 0 { 5 } else { 1 }).collect(),
            expected: 10_000,
        },
        // LARGE CASE 3: n = 5000
        // Logic: Price is exactly double the length.
        // Input generation: [2, 4, 6, ..., 10000]
        // Optimal strategy: Any cut combination works, but keeping the rod whole is the simplest optimal.
        // Calculation: 5000 * 2 = 10,000.
        // Purpose: Stress test for memory and stack depth in recursive DP solutions.
        TestCase {
            input: (1..=3000).map(|x| x * 2).collect(),
            expected: 6_000,
        },
    ]
}

pub fn test_rod_cutting_variant(
    name: &str,
    algorithm: fn(&[u32]) -> u32,
    test_cases: &Vec<TestCase>,
) {
    let mut tests_failed = 0;

    let header_text = format!("Rod Cutting - {name} approach tests");

    println!("{}", "-".repeat(header_text.len()));
    println!("{}", header_text);
    println!("{}", "-".repeat(header_text.len()));

    for test_case in test_cases {
        if name == "Recursive" && test_case.input.len() > 30 {
            println!(
                "[SKIPPED] Input size {} too large for Recursive approach",
                test_case.input.len()
            );
            continue;
        }

        let timer = std::time::Instant::now();
        let received = algorithm(&test_case.input);
        let elapsed = timer.elapsed();

        let input_display_len = if test_case.input.len() > 30 {
            println!(
                "(Displaying first 30 elements of input with length {})",
                test_case.input.len()
            );
            30
        } else {
            test_case.input.len()
        };

        if received != test_case.expected {
            println!(
                "[FAIL] Input: {:?}, expected: {}, received: {}, elapsed: {}ns",
                test_case
                    .input
                    .clone()
                    .into_iter()
                    .take(input_display_len)
                    .collect::<Vec<_>>(),
                test_case.expected,
                received,
                elapsed.as_nanos()
            );
            tests_failed += 1;
            continue;
        }

        println!(
            "Input: {:?}, output: {}, elapsed: {}ns",
            test_case
                .input
                .clone()
                .into_iter()
                .take(input_display_len)
                .collect::<Vec<_>>(),
            received,
            elapsed.as_nanos()
        );
    }

    println!("");

    if tests_failed > 0 {
        println!("{tests_failed} tests failed");
    } else {
        println!("All tests passed!");
    }

    println!("\n");
}
