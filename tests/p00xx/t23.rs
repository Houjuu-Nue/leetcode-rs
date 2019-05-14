
use leetcode_rs::p00xx::p23::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t23() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
        Box::new(Solution2) as Box<dyn Solution>,
    ];
    let test_cases = [
        TestCase {
            input: vec![
                ListNode::from_list(&[1, 4, 5]),
                ListNode::from_list(&[1, 3, 4]),
                ListNode::from_list(&[2, 6]),
            ],
            answer: ListNode::from_list(&[1, 1, 2, 3, 4, 4, 5, 6]),
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.merge_k_lists(test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, list_node_to_vec(&test_case.answer), list_node_to_vec(&test_answer));
        }
    }
}

