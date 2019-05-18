
use leetcode_rs::p00xx::p27::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
    nums_modified: Vec<i32>,
}

#[test]
fn t27() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                nums: vec![3, 2, 2, 3],
                val: 3,
            },
            answer: 2,
            nums_modified: vec![2, 2],
        },
        TestCase {
            input: Input {
                nums: vec![0, 1, 2, 2, 3, 0, 4, 2],
                val: 2,
            },
            answer: 5,
            nums_modified: vec![0, 1, 3, 0, 4],
        },
   ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, mut test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.remove_element(&mut test_case.input.nums, test_case.input.val);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  return answer is {}, {:?}\n\
                TestCase return answer is {}, {:?}\n",
                i, j, &test_case.answer, &test_case.nums_modified, &test_answer, &test_case.input);
            assert_eq!(test_case.input.nums, test_case.nums_modified,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  modified input is {:?}\n\
                TestCase modified input is {:?}\n",
                i, j, &test_case.nums_modified, &test_case.input.nums);
        }
    }
}

