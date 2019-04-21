
use leetcode_rs::p00xx::p10::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t10() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input : Input {
                s: String::from("aa"),
                p: String::from("a"),
            },
            answer: false,
        },
        TestCase {
            input : Input {
                s: String::from("aa"),
                p: String::from("a*"),
            },
            answer: true,
        },
        TestCase {
            input : Input {
                s: String::from("ab"),
                p: String::from(".*"),
            },
            answer: true,
        },
        TestCase {
            input : Input {
                s: String::from("aab"),
                p: String::from("c*a*b"),
            },
            answer: true,
        },
        TestCase {
            input : Input {
                s: String::from("mississippi"),
                p: String::from("mis*is*p*."),
            },
            answer: false,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.is_match(test_case.input.s, test_case.input.p);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
