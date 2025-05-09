pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] + nums[j] == target && i != j {
                return [i as i32, j as i32].to_vec();
            }
        }
    }

    [].to_vec()
}
