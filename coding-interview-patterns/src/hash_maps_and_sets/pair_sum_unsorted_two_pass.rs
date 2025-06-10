use std::collections::HashMap;

pub fn pair_sum_unsorted_two_pass(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut num_map = HashMap::new();

    // First pass: Populate the hash map with each number and its 
    // index.
    for (i, num) in nums.iter().enumerate() {
        num_map.insert(num, i);
    }
    // Second pass: Check for each number's complement in the hash map.
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(j) = num_map.get(&complement) {
            if i != *j {
                return vec![i, *j];
            }
        }
    }
    vec![]
}
