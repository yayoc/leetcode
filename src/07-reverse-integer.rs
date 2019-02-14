// Given a 32-bit signed integer, reverse digits of an integer.

// Example 1:

// Input: 123
// Output: 321

// Example 2:

// Input: -123
// Output: -321

// Example 3:

// Input: 120
// Output: 21

// Note:
// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer
// range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed
// integer overflows.

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut y = x;
        let mut rev = 0;
        while y != 0 {
            let pop = y % 10;
            y /= 10;
            if rev > i32::max_value() / 10 {
                return 0;
            }
            if rev < i32::min_value() / 10 {
                return 0;
            }
            if rev == i32::max_value() / 10 && pop > 7 {
                return 0;
            }
            rev = rev * 10 + pop;
        }
        return rev;
    }
}

// An another solution with converting to string
pub fn reverse(x: i32) -> i32 {
    x.signum()
        * x.abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
}
