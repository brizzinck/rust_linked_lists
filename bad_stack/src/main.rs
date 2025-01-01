use std::mem;
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}
#[derive(Debug)]
struct List {
    head: Option<Box<Node>>,
}

impl List {
    fn new() -> List {
        List { head: None }
    }
    fn push(&mut self, x: i32) {
        let new_head = Some(Box::new(Node {
            value: x,
            next: mem::replace(&mut self.head, None),
        }));
        self.head = new_head;
    }
    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

fn main() {
    println!("Hello, i am bad stack!");
}

#[cfg(test)]
mod tests {
    use crate::List;

    #[test]
    fn push_pop_works() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
    }
}
