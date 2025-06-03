mod two_pointers {
    pub mod triplet_sum;
}

use two_pointers::triplet_sum::triplet_sum;

fn main() {
    let nums = vec![0, -1, 2, -3, 1];
    let result = triplet_sum(nums);
    println!("Result: {:?}", result);
}
