
use leetcode_rs::p00xx::p17::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t17() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: String::from("23"),
            answer: vec![
                String::from("ad"),
                String::from("ae"),
                String::from("af"),
                String::from("bd"),
                String::from("be"),
                String::from("bf"),
                String::from("cd"),
                String::from("ce"),
                String::from("cf"),
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.letter_combinations(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}
