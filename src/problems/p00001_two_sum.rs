/// Runs the `two_sum` function with the following inputs:
///
/// - `[2,7,11,15]`, `9`
/// - `[2,5,5,11]`, `10`
pub fn run() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]); // nums[0] + nums[1] == 9
    assert_eq!(two_sum(vec![2, 5, 5, 11], 10), vec![1, 2]); // nums[0] + nums[2] == 10
}

/// Solves the Two Sum problem by finding indices of two numbers in an array that add up to a target value.
///
/// Given an array of integers `nums` and an integer `target`, this function returns the indices of the two
/// numbers such that their sum equals `target`. Each input is guaranteed to have exactly one solution,
/// and the same element cannot be used twice. The answer can be returned in any order.
///
/// ## Examples
///
/// ```rust
/// use leetcode::problems::p00001_two_sum::two_sum;
///
/// let result = two_sum(vec![2, 7, 11, 15], 9);
/// assert_eq!(result, vec![0, 1]);
///
/// let result = two_sum(vec![3, 2, 4], 6);
/// assert_eq!(result, vec![1, 2]);
///
/// let result = two_sum(vec![3, 3], 6);
/// assert_eq!(result, vec![0, 1]);
/// ```
///
/// ## Constraints
///
/// - `2 <= nums.length <= 10^4`
/// - `-10^9 <= nums[i] <= 10^9`
/// - `-10^9 <= target <= 10^9`
/// - Only one valid answer exists.
///
/// ## Notes
///
/// This implementation assumes that each input has exactly one solution, as per the problem constraints.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 {
        panic!("nums must have at least two elements")
    }

    for i in 0..nums.len() {
        let left = nums[i];

        for j in 1..nums.len() {
            let right = nums[j];
            if i == j {
                continue;
            }

            if left + right == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    panic!("no solution found")
}
