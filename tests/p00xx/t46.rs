
use leetcode_rs::p00xx::p46::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t46() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
        Box::new(Solution2) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![1, 2, 3],
            answer: vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.permute(test_case.input);

            assert!(is_match(&test_answer, &test_case.answer),
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

fn is_match(test_answer: &Vec<Vec<i32>>, correct_answer: &Vec<Vec<i32>>) -> bool {

    if test_answer.len() != correct_answer.len() { return false }

    test_answer.iter().all(|ans| 
        correct_answer.contains(ans)
    )
}


