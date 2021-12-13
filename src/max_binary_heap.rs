pub struct MaxBinaryHeap {
    items: Vec<i32>,
}

impl MaxBinaryHeap {
    pub fn new() -> Self {
        MaxBinaryHeap { items: vec![0] }
    }

    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity + 1);
        vec.push(0);
        MaxBinaryHeap { items: vec }
    }

    pub fn push(&mut self, val: i32) {
        self.items.push(val);
        self.move_up(self.items.len() - 1);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let val = self.items[1];
        let last = self.items.len() - 1;
        self.items.swap(1, last);
        self.items.pop();
        self.move_down(1);
        Some(val)
    }

    pub fn peek(&self) -> Option<&i32> {
        if self.is_empty() {
            return None;
        }
        Some(&self.items[1])
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() < 2
    }

    fn move_up(&mut self, mut k: usize) {
        while k > 1 && self.less(k / 2, k) {
            self.items.swap(k / 2, k);
            k /= 2;
        }
    }

    fn move_down(&mut self, mut k: usize) {
        while 2 * k < self.items.len() {
            let mut left = 2 * k;
            if left < self.items.len() - 1 && self.less(left, left + 1) {
                left += 1;
            }
            if self.less(left, k) {
                break;
            }
            self.items.swap(k, left);
            k = left;
        }
    }

    fn less(&self, a: usize, b: usize) -> bool {
        self.items[a] < self.items[b]
    }
}
