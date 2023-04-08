pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let len = nums.len();
    if len < 3 {
        return ans;
    }
    let mut nums = nums;
    nums.sort();
    let mut left = 0;
    let mut right = 0;
    for k in 0..len {
        if nums[k] > 0 {
            return ans;
        }
        if (k > 0) && (nums[k] == nums[k-1]) {
            continue;
        }
        left = k + 1;
        right = len - 1;
        while left < right {
            match nums[k] + nums[left] + nums[right] {
                0 => {
                    ans.push(vec![nums[k], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                    while left < right && nums[left] == nums[left-1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right+1] {
                        right -= 1;
                    }
                },
                a if a > 0 => {right -= 1},
                _ => {left += 1},
            }
        }
    }
    ans
}