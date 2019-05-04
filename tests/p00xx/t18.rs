
use leetcode_rs::p00xx::p18::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t18() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                nums: vec![1, 0, -1, 0, -2, 2],
                target: 0,
            },
            answer: vec![
                vec![-1,  0, 0, 1],
                vec![-2, -1, 1, 2],
                vec![-2,  0, 0, 2],
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.four_sum(test_case.input.nums, test_case.input.target);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

