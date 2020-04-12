#![allow(dead_code)]

struct MyStack<T> {
    data: Vec<T>,
}

impl<T> MyStack<T> {
    fn new() -> MyStack<T> {
        MyStack { data: Vec::new() }
    }
}

// I can separate out the interface for the stack functionality here
trait IsStack<T> {
    fn push(&mut self, val: T);
    fn size(self) -> usize;
    fn pop(&mut self) -> Option<T>; //If there's nothing in the stack then None can be returned
    fn is_empty(self) -> bool;
    fn peek(&self) -> Option<&T>;//If there's nothing in the stack then None can be returned. In addition I 
}

impl<T> IsStack<T> for MyStack<T> {
    fn push(&mut self, val: T) {
        self.data.push(val)
    }
    fn size(self) -> usize {
        self.data.len()
    }
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    fn is_empty(self) -> bool {
        self.data.is_empty()
    }
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_push() {
        let mut uut = MyStack::new();
        uut.push(1);
        assert_eq!(uut.size(), 1);
    }
    #[test]
    fn initialised_empty() {
        let uut: MyStack<i32> = MyStack::new();
        assert_eq!(uut.size(), 0);
    }
    #[test]
    fn can_pop() {
        let mut uut = MyStack::new();
        uut.push(5);
        let v = uut.pop();
        assert!(uut.is_empty());
        assert_eq!(v.unwrap(), 5);
    }
    #[test]
    fn can_peek() {
        let mut uut = MyStack::new();
        uut.push(5);
        let v = uut.peek();
        assert_eq!(5, *v.unwrap());
        assert_eq!(uut.size(), 1);
    }
    #[test]
    fn can_contain_other_data_types() {
        let mut uut: MyStack<f32> = MyStack::new();
        uut.push(30.3);
        assert_eq!(*uut.peek().unwrap(), 30.3);
    }
}
