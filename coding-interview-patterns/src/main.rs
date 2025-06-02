mod two_pointers {
    pub mod pair_sum_sorted;
}

use two_pointers::pair_sum_sorted::pair_sum_sorted;

fn main() {
    let nums = [-5, -2, 3, 4, 6];
    let target = 7;
    let result = pair_sum_sorted(&nums, target);
    println!("Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::two_pointers::pair_sum_sorted::pair_sum_sorted;

    fn contains_pair(result: &[usize], expected_pairs: &[Vec<usize>]) -> bool {
        expected_pairs.iter().any(|pair| {
            (result.len() == 2) && ((result[0] == pair[0] && result[1] == pair[1]) || (result[0] == pair[1] && result[1] == pair[0]))
        })
    }

    #[test]
    fn test_example_1() {
        let nums = [-5, -2, 3, 4, 6];
        let target = 7;
        let result = pair_sum_sorted(&nums, target);
        assert_eq!(nums[result[0]] + nums[result[1]], target);
    }

    #[test]
    fn test_example_2_multiple_valid_pairs() {
        let nums = [1, 1, 1];
        let target = 2;
        let expected_outputs = vec![
            vec![0, 1], vec![1, 0], vec![0, 2],
            vec![2, 0], vec![1, 2], vec![2, 1],
        ];
        let result = pair_sum_sorted(&nums, target);
        assert!(contains_pair(&result, &expected_outputs));
    }

    #[test]
    fn test_no_pair_found() {
        let nums = [1, 2, 3, 9];
        let target = 8;
        let result = pair_sum_sorted(&nums, target);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = [-10, -4, -2, 0, 3];
        let target = -6;
        let result = pair_sum_sorted(&nums, target);
        assert_eq!(nums[result[0]] + nums[result[1]], target);
    }

    #[test]
    fn test_duplicates() {
        let nums = [2, 2, 2, 2];
        let target = 4;
        let expected_outputs = vec![
            vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0],
            vec![1, 2], vec![2, 1], vec![0, 3], vec![3, 0],
            vec![1, 3], vec![3, 1], vec![2, 3], vec![3, 2],
        ];
        let result = pair_sum_sorted(&nums, target);
        assert!(contains_pair(&result, &expected_outputs));
    }
}
