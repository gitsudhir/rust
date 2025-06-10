use std::collections::HashMap;

pub fn pair_sum_unsorted(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut hashmap = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(j) = hashmap.get(&complement) {
            if i != *j {
                return vec![i, *j];
            }
        }
        hashmap.insert(num,i);
    }
    vec![]
}
