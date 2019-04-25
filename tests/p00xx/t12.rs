
use leetcode_rs::p00xx::p12::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t12() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input : 3,
            answer: String::from("III"),
        },
        TestCase {
            input : 4,
            answer: String::from("IV"),
        },
        TestCase {
            input : 9,
            answer: String::from("IX"),
        },
        TestCase {
            input : 58,
            answer: String::from("LVIII"),
        },
        TestCase {
            input : 1994,
            answer: String::from("MCMXCIV"),
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.int_to_roman(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
