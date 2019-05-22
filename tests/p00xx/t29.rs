
use leetcode_rs::p00xx::p29::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t29() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                dividend: 10,
                divisor: 3,
            },
            answer: 3,
        },
        TestCase {
            input: Input {
                dividend: 7,
                divisor: -3,
            },
            answer: -2,
        },
        TestCase {
            input: Input {
                dividend: -10,
                divisor: 3,
            },
            answer: -3,
        },
        TestCase {
            input: Input {
                dividend: -17,
                divisor: -3,
            },
            answer: 5,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.divide(test_case.input.dividend, test_case.input.divisor);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

