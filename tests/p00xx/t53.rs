
use leetcode_rs::p00xx::p53::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t53() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
        Box::new(Solution2) as Box<dyn Solution>,
        Box::new(Solution3) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
            answer: 6,
        },
        TestCase {
            input: vec![8, -19, 5, -4, 20],
            answer: 21,
        },
        TestCase {
            input: vec![-2, -1],
            answer: -1,
        },
        TestCase {
            input: vec![-2, 1],
            answer: 1,
        },
        TestCase {
            input: vec![1, 2],
            answer: 3,
        },
        TestCase {
            input: vec![1, 2, -1, -2, 2, 1, -2, 1, 4, -5, 4],
            answer: 6,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.max_sub_array(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

