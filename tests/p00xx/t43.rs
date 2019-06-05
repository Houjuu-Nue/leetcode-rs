
use leetcode_rs::p00xx::p43::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t43() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                num1: String::from("123456789"),
                num2: String::from("987654321"),
            },
            answer: String::from("121932631112635269"),
        },
        TestCase {
            input: Input {
                num1: String::from("2"),
                num2: String::from("3"),
            },
            answer: String::from("6"),
        },
        TestCase {
            input: Input {
                num1: String::from("123"),
                num2: String::from("456"),
            },
            answer: String::from("56088"),
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.multiply(test_case.input.num1, test_case.input.num2);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

