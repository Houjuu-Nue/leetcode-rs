
use leetcode_rs::p00xx::p50::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t50() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                x: 2.00000,
                n: 10,
            },
            answer: 1024.00000,
        },
        TestCase {
            input: Input {
                x: 2.10000,
                n: 3,
            },
            answer: 9.26100,
        },
        TestCase {
            input: Input {
                x: 2.00000,
                n: -2,
            },
            answer: 0.25000,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.my_pow(test_case.input.x, test_case.input.n);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

