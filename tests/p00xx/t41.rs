
use leetcode_rs::p00xx::p41::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t41() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![-1, 4, 2, 1, 9, 10],
            answer: 3,
        },
        TestCase {
            input: vec![1, 1],
            answer: 2,
        },
        TestCase {
            input: vec![1, 2, 0],
            answer: 3,
        },
        TestCase {
            input: vec![3, 4, -1, 1],
            answer: 2,
        },
        TestCase {
            input: vec![7, 8, 9, 11, 12],
            answer: 1,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.first_missing_positive(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

