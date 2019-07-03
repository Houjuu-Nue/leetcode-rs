
use leetcode_rs::p00xx::p56::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t56() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![
                vec![2, 3],
                vec![5, 5],
                vec![2, 2],
                vec![3, 4],
                vec![3, 4],
            ],
            answer: vec![
                vec![2, 4],
                vec![5, 5],
            ],
        },
        TestCase {
            input: vec![
                vec![1, 3],
                vec![2, 6],
                vec![8, 10],
                vec![15, 18],
            ],
            answer: vec![
                vec![1, 6],
                vec![8, 10],
                vec![15, 18],
            ],
        },
        TestCase {
            input: vec![
                vec![1, 4],
                vec![2, 3],
            ],
            answer: vec![
                vec![1, 4],
            ],
        },
        TestCase {
            input: vec![
                vec![1, 4],
                vec![4, 5],
            ],
            answer: vec![
                vec![1, 5],
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.merge(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

