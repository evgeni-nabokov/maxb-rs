pub struct MinStack {
    container: Vec<(i32, i32)>
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            container: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        if self.container.is_empty() {
            self.container.push((x, x));
        } else {
            let curr_min = self.container.last().unwrap().1;
            self.container.push((x, std::cmp::min(x, curr_min)));
        }
    }

    pub fn pop(&mut self) {
        self.container.pop();
    }

    pub fn top(&self) -> i32 {
        self.container.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.container.last().unwrap().1
    }
}
