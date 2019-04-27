
use leetcode_rs::p00xx::p13::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t13() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        //Box::new(Solution1) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input: String::from("III"),
            answer : 3,
        },
        TestCase {
            input: String::from("IV"),
            answer : 4,
        },
        TestCase {
            input: String::from("IX"),
            answer : 9,
        },
        TestCase {
            input: String::from("LVIII"),
            answer : 58,
        },
        TestCase {
            input: String::from("MCMXCIV"),
            answer : 1994,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.roman_to_int(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
