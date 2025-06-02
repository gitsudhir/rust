pub fn pair_sum_sorted(nums: &[i32], target: i32) -> Vec<usize> {
    let mut first_pointer = 0;
    let mut last_pointer = nums.len() - 1;
    while first_pointer < last_pointer {
        let first_value = nums[first_pointer];
        let last_value = nums[last_pointer];
        let cur_sum = first_value + last_value;
        // current sum is lower than target , increment the first pointer
        if cur_sum < target {
            first_pointer += 1;
        } else if cur_sum > target {
            // current sum is heigher than target , decreament the last pointer
            last_pointer -= 1;
        } else {
            // current sum is equal to target , return the first pointer and last pointer
            return vec![first_pointer, last_pointer];
        }
    }
    vec![]
}
