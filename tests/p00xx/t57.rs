
use leetcode_rs::p00xx::p57::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t57() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input: Input {
                intervals: vec![
                    vec![1, 3],
                    vec![6, 9],
                ],
                new_interval: vec![2, 5]
            },
            answer: vec![
                vec![1, 5],
                vec![6, 9],
            ],
        },
        TestCase {
            input: Input {
                intervals: vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16],
                ],
                new_interval: vec![4, 8],
            },
            answer: vec![
                vec![1, 2],
                vec![3, 10],
                vec![12, 16],
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.insert(test_case.input.intervals, test_case.input.new_interval);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
