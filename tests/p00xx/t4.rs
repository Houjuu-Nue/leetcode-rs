
use leetcode_rs::p00xx::p4::*;

#[derive(Debug, Clone)]
struct TestCase {
    input: Input,
    answer: Answer,
}

#[test]
fn t4() {

    let solutions = [
        Box::new(Solution1) as Box<dyn Solution>,
    ];

    let test_cases = [
        TestCase {
            input: Input {
                nums1: vec![1, 3],
                nums2: vec![2],
            },
            answer: 2.0,
        },
        TestCase {
            input: Input {
                nums1: vec![1, 2],
                nums2: vec![3, 4],
            },
            answer: 2.5,
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.find_median_sorted_arrays(test_case.input.nums1, test_case.input.nums2);

            let hint = format!("Test failed on Solution {} TestCase {}.\nCorrect  answer is {}\nTestCase answer is {}\n", i, j, &test_case.answer, &test_answer);
            assert_eq!(test_answer, test_case.answer, "{}", &hint);
        }
    }
}
