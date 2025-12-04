struct TestCase {
    input: Vec<u32>,
    expected: u32,
}

pub fn test_rod_cutting_variant(name: &str, algorithm: fn(&[u32]) -> u32) {
    let test_cases: Vec<TestCase> = vec![
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
    ];

    let mut tests_failed = 0;

    println!("{}", "-".repeat(15));
    println!("Testing: {name}\n");
    for test_case in test_cases {
        let received = algorithm(&test_case.input);
        println!(
            "Input: {:?}, expected: {}, received: {}",
            test_case.input, test_case.expected, received
        );

        if received != test_case.expected {
            tests_failed += 1;
        }
    }

    println!("");

    if tests_failed > 0 {
        println!("{tests_failed} tests failed");
    } else {
        println!("All tests passed!");
    }

    println!("\n");
}
