
use leetcode_rs::p00xx::p26::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
    input_modified: Input,
}

#[test]
fn t26() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![1, 1, 2],
            answer: 2,
            input_modified: vec![1, 2],
        },
        TestCase {
            input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
            answer: 5,
            input_modified: vec![0, 1, 2, 3, 4],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, mut test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.remove_duplicates(&mut test_case.input);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  return answer is {}, {:?}\n\
                TestCase return answer is {}, {:?}\n",
                i, j, &test_case.answer, &test_case.input_modified, &test_answer, &test_case.input);
            assert_eq!(test_case.input, test_case.input_modified,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  modified input is {:?}\n\
                TestCase modified input is {:?}\n",
                i, j, &test_case.input_modified, &test_case.input);
        }
    }
}

