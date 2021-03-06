
use leetcode_rs::p00xx::p6::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t6() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
        Box::new(Solution2) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input: Input {
                s: String::from("LEETCODEISHIRING"),
                num_rows: 3,
            },
            answer: String::from("LCIRETOESIIGEDHN"),
        },
        TestCase {
            input: Input {
                s: String::from("LEETCODEISHIRING"),
                num_rows: 4,
            },
            answer: String::from("LDREOEIIECIHNTSG"),
        },
        TestCase {
            input: Input {
                s: String::from("PAYPALISHIRING"),
                num_rows: 4,
            },
            answer: String::from("PINALSIGYAHRPI"),
        }
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.convert(test_case.input.s, test_case.input.num_rows);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
