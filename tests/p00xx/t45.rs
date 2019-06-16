
use leetcode_rs::p00xx::p45::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t45() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0],
            answer: 3,
        },
        TestCase {
            input: vec![1, 2, 1, 1, 1],
            answer: 3,
        },
        TestCase {
            input: vec![2, 3, 1, 1, 4],
            answer: 2,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.jump(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

