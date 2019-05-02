
use leetcode_rs::p00xx::p15::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t15() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
        Box::new(Solution2) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        // TestCase {
        //     input: vec![-1, 0, 1, 2, -1, -4],
        //     answer: vec![
        //         vec![-1, 0, 1],
        //         vec![-1, 2, -1],
        //     ],
        // },
        TestCase {
            input: vec![],
            answer: vec![],
        },
        TestCase {
            input: vec![-2, 0, 0, 2, 2],
            answer: vec![
                vec![-2, 0, 2],
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.three_sum(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
