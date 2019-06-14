
use leetcode_rs::p00xx::p44::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t44() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                s: String::from("aaaababbbaaabaabbbbabaababaabbabbaabababbaaaaaaabba"),
                p: String::from("baaaaba*****b***ab******"),
            },
            answer: false,
        },
        TestCase {
            input: Input {
                s: String::from("aa"),
                p: String::from("a"),
            },
            answer: false,
        },
        TestCase {
            input: Input {
                s: String::from("aa"),
                p: String::from("*"),
            },
            answer: true,
        },
        TestCase {
            input: Input {
                s: String::from("cb"),
                p: String::from("?a"),
            },
            answer: false,
        },
        TestCase {
            input: Input {
                s: String::from("adceb"),
                p: String::from("*a*b"),
            },
            answer: true,
        },
        TestCase {
            input: Input {
                s: String::from("acdcb"),
                p: String::from("a*c?b"),
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

