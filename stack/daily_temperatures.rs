// 739. Daily Temperatures

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut ans = vec![0; n];
        let mut stack = Vec::new();
        for (i, &temp) in temperatures.iter().enumerate() {
            while let Some(&last_index) = stack.last() {
                if temperatures[last_index] < temp {
                    stack.pop();
                    ans[last_index] = (i - last_index) as i32;
                } else {
                    break;
                }
            }
            stack.push(i)
        }
        ans
    }
}
