
use leetcode_rs::p00xx::p34::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t34() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
        Box::new(Solution2) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                nums: vec![5, 7, 7, 8, 8, 10],
                target: 8,
            },
            answer: vec![3, 4],
        },
        TestCase {
            input: Input {
                nums: vec![5, 7, 7, 8, 8, 10],
                target: 6,
            },
            answer: vec![-1, -1],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.search_range(test_case.input.nums, test_case.input.target);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

