mod two_pointers {
    pub mod largest_container;
}

use two_pointers::largest_container::largest_container;

fn main() {
    let nums = vec![2, 7, 8, 3, 7, 6];
    let result = largest_container(nums);
    println!("Result: {:?}", result);
    assert_eq!(largest_container(vec![2, 7, 8, 3, 7, 6]), 24);
    // Between indices 1 and 5: min(7, 6) * (5 - 1) = 6 * 4 = 24
    assert_eq!(largest_container(vec![1, 1]), 1);
    // Only one possible container: min(1, 1) * 1 = 1

    assert_eq!(largest_container(vec![1, 2]), 1);
    // min(1, 2) * 1 = 1
    assert_eq!(largest_container(vec![1, 2, 3, 4, 5]), 6);
    // Best between indices 1 and 4: min(2, 5) * 3 = 2 * 3 = 6

    assert_eq!(largest_container(vec![5, 4, 3, 2, 1]), 6);
    // Best between indices 0 and 3: min(5, 2) * 3 = 2 * 3 = 6
    assert_eq!(largest_container(vec![5, 5, 5, 5, 5]), 20);
    // Any two farthest lines: 5 * (4 - 0) = 5 * 4 = 20
    assert_eq!(largest_container(vec![3]), 0); // Not enough lines to form a container
}
