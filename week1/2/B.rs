struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            in_stack: vec![],
            out_stack: vec![],
        }
    }

    fn push(&mut self) -> i32 {
        if self.out_stack.is_empty(){
            self.fill.out();
        }

        self.out_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32{
        if self.out_stack.is_empty(){
            self.fill_out();
        }

        *self.out_stack.last().unwrap()
    }


    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }

    fn fill_out(&mut self) {
        let MyQueue { in_stack, out_stack } = self;
        if out_stack.is_empty() {
            while !in_stack.is_empty() {
                out_stack.push(in_stack.pop().unwrap());
            }
        }
    }
}