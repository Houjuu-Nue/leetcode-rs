
use leetcode_rs::p00xx::p21::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t21() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    let test_cases = [
        TestCase {
            input: Input {
                l1: ListNode::from_list(&[1, 2, 4]),
                l2: ListNode::from_list(&[1, 3, 4]),
            },
            answer: ListNode::from_list(&[1, 1, 2, 3, 4, 4]),
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.merge_two_lists(test_case.input.l1, test_case.input.l2);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, list_node_to_vec(&test_case.answer), list_node_to_vec(&test_answer));
        }
    }
}

