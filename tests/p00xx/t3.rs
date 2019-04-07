
use leetcode_rs::p00xx::p3::*;

#[derive(Debug, Clone)]
struct TestCase {
    input: Input,
    answer: Answer,
}

#[test]
fn t3() {

    let solutions = [
        Box::new(Solution1) as Box<dyn Solution>,
    ];

    let test_cases: [TestCase; 3] = [
        TestCase {
            input: String::from("abcabcbb"),
            answer: 3,
        },
        TestCase {
            input: String::from("bbbbb"),
            answer: 1,
        },
        TestCase {
            input: String::from("pwwkew"),
            answer: 3,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.length_of_longest_substring(test_case.input);

            let hint = format!("Test failed on Solution {} Sample {}.\nCorrect answer is {}\nTestCase answer is {}\n", i, j, &test_answer, &test_case.answer);
            assert_eq!(test_answer, test_case.answer, "{}", &hint);
        }
    }
}
