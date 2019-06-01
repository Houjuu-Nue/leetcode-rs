
use leetcode_rs::p00xx::p40::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t40() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                candidates: vec![10, 1, 2, 7, 6, 1, 5],
                target: 8,
            },
            answer: vec![
                vec![1, 1, 6],
                vec![1, 2, 5],
                vec![1, 7],
                vec![2, 6],
            ],
        },
        TestCase {
            input: Input {
                candidates: vec![2, 5, 2, 1, 2],
                target: 5,
            },
            answer: vec![
                vec![1, 2, 2],
                vec![5],
            ],
        },
        TestCase {
            input: Input {
                candidates: vec![2, 2, 2],
                target: 2,
            },
            answer: vec![
                vec![2],
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.combination_sum2(test_case.input.candidates, test_case.input.target);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

