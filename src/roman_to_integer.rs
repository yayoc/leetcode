// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
// 
// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// 
// For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.
// 
// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
// 
//     I can be placed before V (5) and X (10) to make 4 and 9. 
//     X can be placed before L (50) and C (100) to make 40 and 90. 
//     C can be placed before D (500) and M (1000) to make 400 and 900.
// 
// Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
// 
// Example 1:
// 
// Input: "III"
// Output: 3
// 
// Example 2:
// 
// Input: "IV"
// Output: 4
// 
// Example 3:
// 
// Input: "IX"
// Output: 9
// 
// Example 4:
// 
// Input: "LVIII"
// Output: 58
// Explanation: L = 50, V= 5, III = 3.
// 
// Example 5:
// 
// Input: "MCMXCIV"
// Output: 1994
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.

pub struct Solution1;

pub trait Solution {
    fn roman_to_int(s: String) -> i32;
}

impl Solution for Solution1 {
    fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        for (i, c) in s.chars().enumerate() {
            if i > 0 {
                if c == 'V' || c == 'X' {
                    match s.chars().nth(i - 1) {
                        Some(t) => {
                            if t == 'I' {
                                total -= 1 * 2;
                            }
                        },
                        None => {}
                    };
                }
                if c == 'L' || c == 'C' {
                    match s.chars().nth(i - 1) {
                        Some(t) => {
                            if t == 'X' {
                                total -= 10 * 2;
                            }
                        },
                        None => {}
                    };
                }
                if c == 'D' || c == 'M' {
                    match s.chars().nth(i - 1) {
                        Some(t) => {
                            if t == 'C' {
                                total -= 100 * 2;
                            }
                        },
                        None => {}
                    }; 
                }
            }
            total += match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                 _  => 0
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_roman_to_integer() {
        assert_eq!(3, Solution1::roman_to_int(String::from("III")));
        assert_eq!(4, Solution1::roman_to_int(String::from("IV")));
        assert_eq!(58, Solution1::roman_to_int(String::from("LVIII")));
        assert_eq!(1994, Solution1::roman_to_int(String::from("MCMXCIV")));
    }
}

