use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum_old(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = h.get(&(target - num)) {
                return vec![i as i32, j as i32];
            }
            h.insert(num, i);
        }
        vec![]
    }
}

fn main() {
    println!("{:?}", Solution::two_sum_old(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum_old(vec![3, 2, 4], 6));
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
}
