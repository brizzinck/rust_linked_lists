type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    value: T,
    next: Link<T>,
}
struct List<T> {
    head: Link<T>,
}
impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }
    fn push(&mut self, value: T) {
        let new_head = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_head);
    }
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;
            old_head.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            cur_link = node.next;
        }
    }
}

struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

fn main() {
    println!("Hello, i am ok stack!");
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
    }
    #[test]
    fn peek_works() {
        let mut list = List::new();
        list.push(6);
        assert_eq!(*list.peek().unwrap(), 6);
        list.peek_mut().map(|value| *value += 2);
        assert_eq!(*list.peek().unwrap(), 8);
    }
    #[test]
    fn into_iter_works() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_works() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(*iter.next().unwrap(), 3);
        assert_eq!(*iter.next().unwrap(), 2);
        assert_eq!(*iter.next().unwrap(), 1);
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut_works() {
        let mut list = List::new();
        list.push(1);
        let mut iter = list.iter_mut();
        assert_eq!(
            *iter
                .next()
                .map(|value| {
                    *value = 1234;
                    value
                })
                .unwrap(),
            1234
        );
        assert_eq!(iter.next(), None);
    }
}
