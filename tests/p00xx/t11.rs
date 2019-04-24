
use leetcode_rs::p00xx::p11::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t11() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input : vec![1,8,6,2,5,4,8,3,7],
            answer: 49,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.max_area(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
