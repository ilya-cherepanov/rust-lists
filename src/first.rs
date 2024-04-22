use std::mem;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug)]
enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, new_element: T) {
        let node = Box::new(Node {
            element: new_element,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                let result = node.element;
                self.head = node.next;

                Some(result)
            },
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list: List<i32> = List::new();

        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }
}
