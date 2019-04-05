
use leetcode_rs::p0xx::p002::*;

#[derive(Debug, Clone)]
struct TestCase {
    input: Input,
    answer: Answer,
}

#[test]
fn t002() {

    let solutions = [
        Box::new(Solution1) as Box<dyn Solution>,
    ];

    let test_samples: [TestCase; 1] = [
        test_case1(),
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, sample) in test_samples.iter().enumerate() {

            let test_answer = solution.add_two_numbers(&sample.input.l1, &sample.input.l2);
            
            test_number_equal(&test_answer, &sample.answer, i, j);
        }
    }
}

fn test_case1() -> TestCase {

    TestCase {
        input: Input {
            l1: ListNode::new(2, ListNode::new(4, ListNode::new(3, None))),
            l2: ListNode::new(5, ListNode::new(6, ListNode::new(4, None))),
        },
        answer: ListNode::new(7, ListNode::new(0, ListNode::new(8, None))),
    }
}

fn test_number_equal(t1: &Option<Box<ListNode>>, t2: &Option<Box<ListNode>>, i: usize, j: usize) {

    let mut n1 = t1;
    let mut n2 = t2;

    loop {
        match (n1, n2) {
            | (Some(node1), Some(node2)) => {
                if node1.val == node2.val {
                    n1 = &node1.next;
                    n2 = &node2.next;
                } else {
                    let help_hint = format!("Test failed on Solution {} Sample {}\n.Value on nodes are different.", i, j);
                    assert!(false, help_hint);
                }
            },
            | (None, None) => {
                return // success
            },
            | (Some(_), None)
            | (None, Some(_)) => {
                assert!(false, "Test failed on Solution {} Sample {}.\nOne node is Some(_), another is None.", i, j);
            },
        }
    }
    
}
