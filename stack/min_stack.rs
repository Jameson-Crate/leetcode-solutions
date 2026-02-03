// 155. Min Stack

use std::cmp;

struct MinStack {
    vals: Vec<(i32, i32)>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self { vals: Vec::new() }
    }
    
    fn push(&mut self, val: i32) {
        if let Some((_, prev)) = self.vals.last() {
            self.vals.push((val, cmp::min(val, *prev)));
        } else {
            self.vals.push((val, val));
        }   
    }
    
    fn pop(&mut self) {
        self.vals.pop();
    }
    
    fn top(&self) -> i32 {
        if let Some((val, _)) = self.vals.last() {
            *val
        } else {
            0
        }
    }
    
    fn get_min(&self) -> i32 {
        if let Some((_, min)) = self.vals.last() {
            *min
        } else {
            0
        }
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
