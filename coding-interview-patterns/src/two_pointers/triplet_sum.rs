pub fn triplet_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // sort the vector
    nums.sort();
    // a + b + c = 0
    // use the pair sum sorted , b + c = -a (target);
    let len = nums.len();
    let mut result: Vec<Vec<i32>> = vec![];

    for index in 0..len - 1 {
        // skip the duplicate pair
        if index > 0 && nums[index] == nums[index - 1] {
            continue;
        }
        // find the triplet ,pair_sum_sorted will give [a,b,c]
        let triplet = pair_sum_sorted(&nums[index..], -nums[index]);
        if triplet.len() != 0 {
            result.push(triplet);
        }
    }

    result
}

fn pair_sum_sorted(remain_nums: &[i32], target: i32) -> Vec<i32> {
    let mut first_pointer = 0;
    let mut last_pointer = remain_nums.len() - 1;
    
    while first_pointer < last_pointer {
        let sum = remain_nums[first_pointer] + remain_nums[last_pointer];
        // if sum is equal to target return the  triplet [a,b,c]
        if sum == target {
            return vec![
                -target,
                remain_nums[first_pointer],
                remain_nums[last_pointer],
            ];
        } else if sum < target {
            first_pointer += 1;
        } else {
            last_pointer -= 1;
        }
    }
    // if there is no triplet return empty vector
    vec![]
}
