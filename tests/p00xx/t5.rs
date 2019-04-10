
use leetcode_rs::p00xx::p5::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t4() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        //Box::new(Solution1) as Box<dyn Solution>,
        //Box::new(Solution2) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input : String::from("babad"),
            answer: String::from("bad"),
        },
        TestCase {
            input : String::from("cbbd"),
            answer: String::from("bb"),
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.longest_palindrome(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n
                Correct  answer is {}\n
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
