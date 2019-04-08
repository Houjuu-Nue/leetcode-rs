
use leetcode_rs::p00xx::p2::*;

#[derive(Debug, Clone)]
struct TestCase {
    input: Input,
    answer: Answer,
}

#[derive(Debug, Clone)]
struct CaseNumber {
    input: (Vec<i32>, Vec<i32>),
    answer: Vec<i32>,
}

impl From<CaseNumber> for TestCase {

    fn from(case: CaseNumber) -> TestCase {
        TestCase {
            input: (Input {
                l1: ListNode::from_vec(case.input.0),
                l2: ListNode::from_vec(case.input.1.into())
            }),
            answer: ListNode::from_vec(case.answer.into()),
        }
    }
}

#[test]
fn t2() {

    let solutions = [
        Box::new(Solution1) as Box<dyn Solution>,
    ];

    let test_cases = [
        CaseNumber {
            input: (vec![2, 4, 3], vec![5, 6, 4]),
            answer: vec![7, 0, 8],
        },
        CaseNumber {
            input: (vec![0], vec![0]),
            answer: vec![0],
        },
        CaseNumber {
            input: (vec![9], vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]),
            answer: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned()
            .map(TestCase::from).enumerate() {

            let test_answer = solution.add_two_numbers(test_case.input.l1, test_case.input.l2);
            
            test_number_equal(&test_answer, &test_case.answer, i, j);
        }
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
                    let help_hint = format!("Test failed on Solution {} TestCase {}\n.Value on nodes are different.\nCorrect answer is {:?}\nTestCase answer is {:?}\n", i, j, list_node_to_vec(t2), list_node_to_vec(t1));
                    assert!(false, help_hint);
                }
            },
            | (None, None) => {
                return // success
            },
            | (Some(_), None)
            | (None, Some(_)) => {
                assert!(false, "Test failed on Solution {} TestCase {}.\nCorrect answer is {:?}\nTestCase answer is {:?}\n", i, j, list_node_to_vec(t2), list_node_to_vec(t1));
            },
        }
    }
    
}
