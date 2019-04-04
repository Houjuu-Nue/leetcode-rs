
use leetcode_rs::p0xx::p001::*;

#[test]
fn t001_test1() {

    let solutions = [
        Box::new(Solution1),
    ];

    for solution in solutions.into_iter() {

        let input = Input {
            nums: vec![2, 7, 11, 15],
            target: 9,
        };

        let answer: Answer = vec![0, 1];

        let test_answer = solution.two_sum(input.nums, input.target);

        assert_eq!(test_answer, answer);
    }
}