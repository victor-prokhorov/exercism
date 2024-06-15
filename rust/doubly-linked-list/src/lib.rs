use std::marker::PhantomData;
use std::ptr::NonNull;

mod pre_implemented;

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    elt: T,
}

impl<T> Node<T> {
    fn new(elt: T) -> Self {
        Node {
            next: None,
            prev: None,
            elt,
        }
    }
}

impl<T> AsMut<T> for Node<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.elt
    }
}

pub struct Cursor<'a, T> {
    current: Option<NonNull<Node<T>>>,
    list: &'a mut LinkedList<T>,
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.current
            .map(|mut node| unsafe { node.as_mut().as_mut() })
    }

    pub fn next(&mut self) -> Option<&mut T> {
        self.current
            .and_then(|node| {
                let node_ref = unsafe { node.as_ref() };
                node_ref.next
            })
            .and_then(|node| {
                self.current = Some(node);
                self.peek_mut()
            })
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        self.current
            .and_then(|node| {
                let node_ref = unsafe { node.as_ref() };
                node_ref.prev
            })
            .and_then(|node| {
                self.current = Some(node);
                self.peek_mut()
            })
    }

    pub fn take(&mut self) -> Option<T> {
        let node = self.current.take()?;
        unsafe {
            let node = Box::from_raw(node.as_ptr());

            match (node.prev, node.next) {
                (Some(mut node_prev), Some(mut node_next)) => {
                    node_prev.as_mut().next = Some(node_next);
                    node_next.as_mut().prev = Some(node_prev);
                    self.current = Some(node_next);
                }
                (Some(mut node_prev), None) => {
                    self.list.tail = Some(node_prev);
                    node_prev.as_mut().next = None;
                    self.current = Some(node_prev);
                }
                (None, Some(mut node_next)) => {
                    self.list.head = Some(node_next);
                    node_next.as_mut().prev = None;
                    self.current = Some(node_next);
                }
                (None, None) => {
                    self.list.head = None;
                    self.list.tail = None;
                }
            }
            self.list.len -= 1;

            Some(node.elt)
        }
    }

    pub fn insert_after(&mut self, elt: T) {
        let node = Box::new(Node::new(elt));
        let mut node_ptr = NonNull::from(Box::leak(node));

        if let Some(mut node) = self.current {
            unsafe {
                if let Some(mut node_next) = node.as_ref().next {
                    node.as_mut().next = Some(node_ptr);
                    node_ptr.as_mut().prev = Some(node);
                    node_ptr.as_mut().next = Some(node_next);
                    node_next.as_mut().prev = Some(node_ptr);
                } else {
                    self.list.tail = Some(node_ptr);
                    node.as_mut().next = Some(node_ptr);
                    node_ptr.as_mut().prev = Some(node);
                }
            }
        } else {
            self.list.head = Some(node_ptr);
            self.list.tail = Some(node_ptr);
        }

        self.list.len += 1;
    }

    pub fn insert_before(&mut self, elt: T) {
        let node = Box::new(Node::new(elt));
        let mut node_ptr = NonNull::from(Box::leak(node));

        if let Some(mut node) = self.current {
            unsafe {
                if let Some(mut node_prev) = node.as_ref().prev {
                    node_prev.as_mut().next = Some(node_ptr);
                    node_ptr.as_mut().prev = Some(node_prev);
                    node_ptr.as_mut().next = Some(node);
                    node.as_mut().prev = Some(node_ptr);
                } else {
                    self.list.head = Some(node_ptr);
                    node_ptr.as_mut().next = Some(node);
                    node.as_mut().prev = Some(node_ptr);
                }
            }
        } else {
            self.list.head = Some(node_ptr);
            self.list.tail = Some(node_ptr);
        }

        self.list.len += 1;
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a, T> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let node = self.head?;
        let node = unsafe { node.as_ref() };
        self.len -= 1;
        self.head = node.next;
        Some(&node.elt)
    }
}

#[derive(Default)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.head,
            list: self,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.tail,
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() {}
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}
