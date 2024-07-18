impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..nums.len() {
            if nums[i] <= nums[i-1] {
                let change = nums[i-1] - nums[i] + 1;
                ans += change;
                nums[i] += change;
            }
        }
        ans
    }
}

