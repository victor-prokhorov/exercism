use std::boxed::Box;
use std::collections::VecDeque;
use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Default)]
pub struct SimpleLinkedList<T: Copy> {
    head: Link<T>,
}

pub struct Node<T: Copy> {
    data: T,
    next: Link<T>,
}

impl<T: Copy> SimpleLinkedList<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { head: None }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut head = self.head.as_ref();
        while head.is_some() {
            head = head.and_then(|node| node.next.as_ref());
            len += 1;
        }
        len
    }

    pub fn push(&mut self, data: T) {
        self.head = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take()?;
        self.head = head.next;
        Some(head.data)
    }

    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        Some(&self.head.as_ref()?.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        let mut head = self.head;
        while let Some(node) = head {
            list.push(node.data);
            head = node.next;
        }
        list
    }
}

impl<T: Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for i in iter {
            list.push(i);
        }
        list
    }
}

impl<T: Copy> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut deque = VecDeque::new();
        while let Some(node) = linked_list.pop() {
            deque.push_front(node);
        }
        deque.into()
    }
}
