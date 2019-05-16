
use leetcode_rs::p00xx::p24::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t24() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
    ];
    let test_cases = [
        TestCase {
            input : ListNode::from_list(&[1, 2, 3, 4]),
            answer: ListNode::from_list(&[2, 1, 4, 3]),
        },
        TestCase {
            input : ListNode::from_list(&[1]),
            answer: ListNode::from_list(&[1]),
        }
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.swap_pairs(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, list_node_to_vec(&test_case.answer), list_node_to_vec(&test_answer));
        }
    }
}

