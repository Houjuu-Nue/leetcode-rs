
use leetcode_rs::p00xx::p8::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t8() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input : String::from("42"),
            answer: 42,
        },
        TestCase {
            input : String::from("   -42"),
            answer: -42,
        },
        TestCase {
            input : String::from("4193 with words"),
            answer: 4193,
        },
        TestCase {
            input : String::from("words and 987"),
            answer: 0,
        },
        TestCase {
            input : String::from("-91283472332"),
            answer: -2147483648,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.my_atoi(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
