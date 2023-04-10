pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut res = nums[0] + nums[1] + nums[2];
    for i in 0..nums.len() {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if (sum - target).abs() < (res - target).abs() {
                res = sum;
            }
            if sum > target {
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                return res;
            }
        }
    }
    res
}