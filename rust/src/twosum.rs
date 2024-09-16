use std::collections::HashMap;

pub fn brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, left) in nums.iter().enumerate() {
        for (j, right) in nums.iter().enumerate() {
            if i != j && left + right == target {
                return vec![
                    i.try_into()
                        .expect("The problem won't use arrays longer than i32 max"),
                    j.try_into()
                        .expect("The problem won't use arrays longer than i32 max"),
                ];
            }
        }
    }

    unreachable!("The problem states there should always be a solution")
}

pub fn hashtable(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut previous = HashMap::new();
    for (j, num) in nums.iter().enumerate() {
        if let Some(i) = previous.get(&(target - num)) {
            return vec![
                *i,
                j.try_into()
                    .expect("The problem won't use arrays longer than i32 max"),
            ];
        } else {
            previous.insert(
                nums[j],
                j.try_into()
                    .expect("The problem won't use arrays longer than i32 max"),
            );
        }
    }

    unreachable!("The problem states there should always be a solution")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![2,7,11,15], 9, vec![0, 1])]
    #[case(vec![3,2,4], 6, vec![1, 2])]
    #[case(vec![3, 3], 6, vec![0, 1])]
    fn two_sum_test(#[case] input: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        assert_eq!(brute_force(input.clone(), target), expected);
        assert_eq!(hashtable(input, target), expected);
    }
}
