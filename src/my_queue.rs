pub struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.reverse();
        self.s2.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        self.reverse();
        *self.s2.last().unwrap()
    }

    pub fn empty(&mut self) -> bool {
        self.reverse();
        self.s2.is_empty()
    }

    #[inline]
    fn reverse(&mut self) {
        if self.s2.is_empty() {
            while !self.s1.is_empty() {
                self.s2.push(self.s1.pop().unwrap());
            }
        }
    }
}