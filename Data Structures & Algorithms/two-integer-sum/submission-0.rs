use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        map.insert(nums[0], 0);

        for i in 1..nums.len() {
            let k = target - nums[i];
            if map.contains_key(&k) {
                return vec![*map.get(&k).unwrap(), i as i32];
            }
            map.insert(nums[i], i as i32); 
        }
        vec!()       
    }
}
