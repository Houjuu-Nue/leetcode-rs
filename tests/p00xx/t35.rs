
use leetcode_rs::p00xx::p35::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t35() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                nums: vec![1, 3, 5, 6],
                target: 5,
            },
            answer: 2,
        },
        TestCase {
            input: Input {
                nums: vec![1, 3, 5, 6],
                target: 2,
            },
            answer: 1,
        },
        TestCase {
            input: Input {
                nums: vec![1, 3, 5, 6],
                target: 7,
            },
            answer: 4,
        },
        TestCase {
            input: Input {
                nums: vec![1, 3, 5, 6],
                target: 0,
            },
            answer: 0,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.search_insert(test_case.input.nums, test_case.input.target);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {}\n\
                TestCase answer is {}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

