
use leetcode_rs::p00xx::p49::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t49() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![
                String::from("eat"),
                String::from("tea"),
                String::from("tan"),
                String::from("ate"),
                String::from("nat"),
                String::from("bat")
            ],
            answer: vec![
                vec![String::from("ate"), String::from("eat"), String::from("tea")],
                vec![String::from("nat"), String::from("tan")],
                vec![String::from("bat"), ]
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.group_anagrams(test_case.input);

            assert!(is_match(&test_answer, &test_case.answer),
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

fn is_match(test_answer: &[Vec<String>], correct_answer: &[Vec<String>]) -> bool {
    
    if test_answer.len() != correct_answer.len() { return false }

    test_answer.iter().all(|ans| {
        
        let key = to_sort_bytes(&ans[0]);
        
        if let Some(strs) = correct_answer.iter().find(|strs| to_sort_bytes(&strs[0]) == key) {
            if ans.len() != strs.len() { return false }

            for s in ans {
                if strs.contains(s) == false {
                    return false
                }
            }

            true
        } else {
            false
        }
    })
}

fn to_sort_bytes(s: &String) -> Vec<u8> {

    let mut s = s.clone().into_bytes();
    s.sort_unstable();
    s
}

