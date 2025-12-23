impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();
        let mut result: Vec<i32> = vec![0; n*2];
        for i in 0..n{
            result[i] = nums[i];
            result[i + n] = nums[i];
        }
        return result
    }
}