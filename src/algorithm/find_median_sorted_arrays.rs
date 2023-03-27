pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    let mut nums = vec![];
    while nums1.len() > 0 && nums2.len() > 0 {
        if nums1[0] < nums2[0] {
            nums.push(nums1.remove(0));
        } else {
            nums.push(nums2.remove(0));
        }
    }
    if nums1.len() > 0 {
        nums.append(&mut nums1);
    }
    if nums2.len() > 0 {
        nums.append(&mut nums2);
    }
    let len = nums.len();
    if len % 2 == 0 {
        return (nums[len / 2 - 1] + nums[len / 2]) as f64 / 2.0;
    } else {
        return nums[len / 2] as f64;
    }
}