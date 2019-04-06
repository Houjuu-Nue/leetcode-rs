
use leetcode_rs::p0xx::p001::*;

#[derive(Debug, Clone)]
struct TestCase {
    input: Input,
    answer: Answer,
}

#[test]
fn t001() {

    let solutions = [
        Box::new(Solution1) as Box<dyn Solution>,
        Box::new(Solution2) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input: Input {
                nums: vec![2, 7, 11, 15],
                target: 9,
            },
            answer: vec![0, 1],
        },
        TestCase {
            input: Input {
                nums: vec![3, 3],
                target: 6,
            },
            answer: vec![0, 1]
        },
        TestCase {
            input: Input {
                nums: vec![3, 2, 4],
                target: 6,
            },
            answer: vec![1, 2],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, case) in test_cases.iter().cloned().enumerate() {

            let input = case.input;
            let answer = case.answer;
            let test_answer = solution.two_sum(input.nums, input.target);
            
            assert_eq!(test_answer, answer,
                "Test failed on Solution {} Sample {}", i, j);
        }
    }
}
