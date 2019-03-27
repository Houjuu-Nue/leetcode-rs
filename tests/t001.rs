
use leetcode_rs::p001::{Input, Solution, Answer};

#[test]
fn t001_test1() {

    let input = Input {
        nums: vec![2, 7, 11, 15],
        target: 9,
    };

    let answer: Answer = vec![0, 1];

    let test = Solution::two_sum(input.nums, input.target);

    assert_eq!(test, answer);
}
