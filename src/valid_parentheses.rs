// Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

//  1. Open brackets must be closed by the same type of brackets.
//  2. Open brackets must be closed in the correct order.

// Note that an empty string is also considered valid.

pub struct Solution1;

pub trait Solution {
    fn is_valid(s: String) -> bool;
}

fn is_open(c: char) -> bool {
    c == '{' || c == '[' || c == '('
}

fn is_match(open: char, close: char) -> bool {
    match open {
        '{' => close == '}',
        '[' => close == ']',
        '(' => close == ')',
        _ => false,
    }
}

impl Solution for Solution1 {
    fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        if s.len() % 2 != 0 {
            return false;
        }
        let mut stack = vec![];
        for c in s.chars() {
            if is_open(c) {
                stack.push(c)
            } else {
                let last = stack.pop();
                match last {
                    Some(l) => {
                        if !is_match(l, c) {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert_eq!(true, Solution1::is_valid(String::from("()")));
        assert_eq!(true, Solution1::is_valid(String::from("()[]{}")));
        assert_eq!(false, Solution1::is_valid(String::from("{]")));
        assert_eq!(false, Solution1::is_valid(String::from("([)]")));
        assert_eq!(true, Solution1::is_valid(String::from("{[]}")));
        assert_eq!(false, Solution1::is_valid(String::from("{{)}")));
    }
}
