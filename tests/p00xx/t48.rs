
use leetcode_rs::p00xx::p48::*;

#[derive(Debug, Clone)]
struct TestCase {
    input: Input,
    input_modified: Input,
    answer: Output,
}

#[test]
fn t48() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
            ],
            input_modified: vec![
                vec![7, 4, 1],
                vec![8, 5 ,2],
                vec![9, 6, 3],
            ],
            answer: (),
        },
        TestCase {
            input: vec![
                vec![ 5,  1,  9, 11],
                vec![ 2,  4,  8, 10],
                vec![13,  3,  6,  7],
                vec![15, 14, 12, 16]
            ],
            input_modified: vec![
                vec![15, 13,  2,  5],
                vec![14,  3,  4,  1],
                vec![12,  6,  8,  9],
                vec![16,  7, 10, 11]
            ],
            answer: (),
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, mut test_case) in test_cases.iter().cloned().enumerate() {

            solution.rotate(&mut test_case.input);

            assert_eq!(test_case.input, test_case.input_modified,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.input_modified, &test_case.input);
        }
    }
}

