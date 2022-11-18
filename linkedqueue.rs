use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::ptr;

type  NodePtr<T> = AtomicPtr<Node<T>>;
struct Node<T> {
    pub item: Option<T>,
    pub next: NodePtr<T>,
}
impl<T> Node<T> {
    pub fn new(x: T) -> Self {
        Self {
            item: Some(x), 
            next: AtomicPtr::from(std::ptr::null_mut()),
         }
    }

    pub fn new_empty() -> Self {
        Self {
            item: None, 
            next: AtomicPtr::from(std::ptr::null_mut()),
         }
    }
}

pub struct LinkedQueue<T> {
    len: AtomicUsize,
    head: NodePtr<T>,
    tail: NodePtr<T>,
}
impl<T> LinkedQueue<T> {

    pub fn new() -> Self {
        let empty_node = Box::new(Node::new_empty());
        let node_ptr = Box::into_raw(empty_node);
        let head = AtomicPtr::from(node_ptr);
        let tail = AtomicPtr::from(node_ptr);
        Self {
            len: AtomicUsize::new(0),
            head,
            tail
        }
    }

    pub fn push(&self, item: T) {
        let new_node = Box::new(Node::new(item));
        let node_ptr = Box::into_raw(new_node);

        let old_tail = self.tail.load(Ordering::Acquire);
        unsafe {
            let mut tail_next = &(*old_tail).next;
            while tail_next.compare_exchange(ptr::null_mut(), node_ptr, 
                Ordering::Release, Ordering::Relaxed).is_err() {
                let mut tail = tail_next.load(Ordering::Acquire);
                loop {
                    let next = (*tail).next.load(Ordering::Acquire);
                    if next.is_null() {
                        break;
                    }
                    tail = next;
                }
                tail_next = &(*tail).next;
            }
        }
        let _ = self.tail.compare_exchange(old_tail, 
            node_ptr, Ordering::Release, Ordering::Relaxed);
        self.len.fetch_add(1, Ordering::SeqCst);
    }

    pub fn pop(&self) -> Option<T> {
        let mut data = None;
        if self.is_empty() {
            return data;
        }
        unsafe {
            let mut head: *mut Node<T>; 
            loop {
                head= self.head.load(Ordering::Acquire);
                let next = (*head).next.load(Ordering::Acquire);
                if next.is_null() {
                    return None;
                }
                if self.head.compare_exchange(head, 
                    next, Ordering::Release, Ordering::Relaxed).is_ok() {
                    data = (*next).item.take();
                    break;
                }
            }
            let _ = Box::from_raw(head);
        }
        self.len.fetch_sub(1, Ordering::SeqCst);
        return data;
    }

    pub fn size(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    pub fn is_empty(&self) -> bool {
        0 == self.len.load(Ordering::SeqCst)
    }
}

    // impl<T> Drop for LinkedQueue<T> {
    //     fn drop(&mut self) {

    //         while self.pop().is_some() { }
    //         unsafe {
    //             let _ = Box::from_raw(self.head.get_mut());
    //         }
    //     }
    // }
  
