impl Solution {
    pub fn has_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();
        for i in 1..nums.len() {
            if nums[i-1] == nums[i] {
                return true;
            }
        }
        return false;
    }
}
