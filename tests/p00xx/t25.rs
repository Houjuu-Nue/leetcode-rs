
use leetcode_rs::p00xx::p25::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t25() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    let test_cases = [
        TestCase {
            input: Input {
                head: ListNode::from_list(&[1, 2, 3, 4, 5]),
                k: 2,
            },
            answer: ListNode::from_list(&[2, 1, 4, 3, 5]),
        },
        TestCase {
            input: Input {
                head: ListNode::from_list(&[1, 2, 3, 4, 5]),
                k: 3,
            },
            answer: ListNode::from_list(&[3, 2, 1, 4, 5]),
        },
        TestCase {
            input: Input {
                head: ListNode::from_list(&[1, 2]),
                k: 2,
            },
            answer: ListNode::from_list(&[2, 1]),
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.reverse_k_group(test_case.input.head, test_case.input.k);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, list_node_to_vec(&test_case.answer), list_node_to_vec(&test_answer));
        }
    }
}

