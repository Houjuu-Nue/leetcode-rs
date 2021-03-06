
use leetcode_rs::p00xx::p7::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t7() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input : 123,
            answer: 321,
        },
        TestCase {
            input : -123,
            answer: -321,
        },
        TestCase {
            input : 120,
            answer: 21,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.reverse(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
