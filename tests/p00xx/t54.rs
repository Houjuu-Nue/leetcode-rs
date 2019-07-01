
use leetcode_rs::p00xx::p54::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t54() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![
		vec![1, 2, 3],
	        vec![4, 5, 6],
	        vec![7, 8, 9],
	    ],
            answer: vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
        },
        TestCase {
            input: vec![
		vec![1, 2, 3, 4],
	        vec![5, 6, 7, 8],
	        vec![9, 10, 11, 12],
	    ],
            answer: vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.spiral_order(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

