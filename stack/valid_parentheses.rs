// 20. Valid Parentheses

use std::collections::VecDeque;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = VecDeque::new();
        for c in s.chars() {
            match c {
                '(' => stack.push_front(')'),
                '{' => stack.push_front('}'),
                '[' => stack.push_front(']'),
                ')' | '}' | ']' => {
                    if stack.len() > 0 && stack[0] == c {
                        stack.pop_front();
                    } else {
                        return false;
                    }
                },
                _ => return false,
            }
        }
        return stack.len() == 0;
    }
}
