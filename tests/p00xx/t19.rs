
use leetcode_rs::p00xx::p19::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t19() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                head: ListNode::from_list(&[1, 2, 3, 4, 5]),
                n: 2,
            },
            answer: ListNode::from_list(&[1, 2, 3, 5]),
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.remove_nth_from_end(test_case.input.head, test_case.input.n);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, list_node_to_vec(&test_case.answer), list_node_to_vec(&test_answer));
        }
    }
}

