
use leetcode_rs::p00xx::p30::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t30() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                s: String::from("abababab"),
                words: vec![
                    String::from("a"),
                    String::from("b"),
                    String::from("a"),
                ],
            },
            answer: vec![0, 2, 4],
        },
        TestCase {
            input: Input {
                s: String::from("barfoothefoobarman"),
                words: vec![
                    String::from("foo"),
                    String::from("bar"),
                ],
            },
            answer: vec![0, 9],
        },
        TestCase {
            input: Input {
                s: String::from("wordgoodgoodgoodbestword"),
                words: vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("word"),
                ],
            },
            answer: vec![],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.find_substring(test_case.input.s, test_case.input.words);

            assert!(compare(&test_answer, &test_case.answer),
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

fn compare<T: Eq>(v1: &Vec<T>, v2: &Vec<T>) -> bool {

    if v1.len() != v2.len() { return false }
    v2.iter().all(|v| v1.contains(v))
}

