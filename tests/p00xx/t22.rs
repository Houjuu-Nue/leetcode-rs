
use leetcode_rs::p00xx::p22::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t22() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
        Box::new(Solution2) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: 3,
            answer: vec![
                String::from("((()))"),
                String::from("(()())"),
                String::from("(())()"),
                String::from("()(())"),
                String::from("()()()"),
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.generate_parenthesis(test_case.input);

            assert!(compare(&test_answer, &test_case.answer),
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

fn compare(v1: &Vec<String>, v2: &Vec<String>) -> bool {

    use std::collections::HashSet;
    let set1: HashSet<String> = v1.iter().cloned().collect();
    let set2: HashSet<String> = v2.iter().cloned().collect();
    set1 == set2

    // v1.iter()
    //    .all(|s1| v2.iter().find(|s2| s2 == &s1).is_some())
}

