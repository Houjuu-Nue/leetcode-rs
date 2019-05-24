
use leetcode_rs::p00xx::p31::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    input_modifed: Input,
}

#[test]
fn t31() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![1, 2, 3],
            input_modifed: vec![1, 3, 2],
        },
        TestCase {
            input: vec![3, 2, 1],
            input_modifed: vec![1, 2, 3],
        },
        TestCase {
            input: vec![1, 3, 2],
            input_modifed: vec![2, 1, 3],
        },
        TestCase {
            input: vec![2, 3, 1],
            input_modifed: vec![3, 1, 2],
        },
        TestCase {
            input: vec![4, 2, 0, 2, 3, 2, 0],
            input_modifed: vec![4, 2, 0, 3, 0, 2, 2],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, mut test_case) in test_cases.iter().cloned().enumerate() {

            solution.next_permutation(&mut test_case.input);

            assert_eq!(test_case.input, test_case.input_modifed,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.input_modifed, &test_case.input);
        }
    }
}

