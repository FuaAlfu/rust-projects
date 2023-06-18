fn max_subarray(nums: &[i32]) -> i32 {
    let mut current_max = nums[0];
    let mut global_max = nums[0];
    
    for i in 1..nums.len() {
        current_max = current_max.max(nums[i]);
        global_max = global_max.max(current_max);
    }
    
    global_max
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray(&nums);
    println!("Maximum subarray sum: {}", max_sum);
}
