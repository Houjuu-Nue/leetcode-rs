
use leetcode_rs::p00xx::p55::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t55() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
        Box::new(Solution2) as Box<dyn Solution>,
        Box::new(Solution3) as Box<dyn Solution>,
        Box::new(Solution4) as Box<dyn Solution>,
        Box::new(Solution5) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![3, 2, 1, 0, 4],
            answer: false,
        },
        TestCase {
            input: vec![1, 2, 3],
            answer: true,
        },
        TestCase {
            input: vec![2, 3, 1, 1, 4],
            answer: true,
        },
        TestCase {
            input: vec![0, 2, 3],
            answer: false,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.can_jump(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

