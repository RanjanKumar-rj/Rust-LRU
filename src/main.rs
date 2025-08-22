use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
struct DoubleLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> DoubleLinkedList<T>
where
    T: std::fmt::Debug + Clone,
{
    fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn insert_from_end(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));
        if self.head.is_none() {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        } else {
            new_node.borrow_mut().prev = Some(Rc::downgrade(self.tail.as_ref().unwrap()));
            self.tail.as_mut().unwrap().borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node);
        }
        self.len += 1;
    }

    fn insert_from_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));
        if self.head.is_none() {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        } else {
            new_node.borrow_mut().next = Some(self.head.as_ref().unwrap().clone());
            self.head.as_mut().unwrap().borrow_mut().prev = Some(Rc::downgrade(&new_node));
            self.head = Some(new_node);
        }
        self.len += 1;
    }

    fn delete_from_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            println!("List is empty");
            return None;
        }
        let old_head = self.head.clone();
        self.head = self.head.clone().as_mut().unwrap().borrow_mut().next.take();

        if self.head.is_none() {
            self.tail = None;
        } else {
            self.head.as_mut().unwrap().borrow_mut().prev = None;
        }
        Some(old_head.as_ref().unwrap().borrow().data.clone())
    }

    fn delete_from_end(&mut self) -> Option<T> {
        if self.head.is_none() {
            println!("List is empty");
            return None;
        }
        let old_tail = self.tail.clone();
        self.tail =
            if let Some(node) = &self.tail.clone().as_mut().unwrap().borrow_mut().prev.take() {
                Some(Weak::upgrade(node)?)
            } else {
                None
            };

        if self.tail.is_none() {
            self.head = None;
        } else {
            self.tail.as_mut().unwrap().borrow_mut().next = None
        }
        Some(old_tail.as_ref().unwrap().borrow().data.clone())
    }

    fn display(&self) {
        let mut cur = self.head.clone();
        while let Some(node) = cur {
            print!("{:?} -> ", node.borrow().data);
            cur = node.borrow().next.clone();
        }
        println!();
    }

    fn reverse_display(&self) {
        let mut cur = self.tail.clone();
        while let Some(node) = cur {
            print!("{:?} <- ", node.borrow().data);
            cur = node.borrow().prev.as_ref().and_then(|w| w.upgrade());
        }
        println!();
    }
}

fn main() {
    let mut my_list = DoubleLinkedList::new();

    my_list.insert_from_end(1);
    my_list.insert_from_end(2);
    my_list.insert_from_end(3);
    my_list.insert_from_front(0);

    my_list.display();
    my_list.reverse_display();

    let _ = my_list.delete_from_front();
    my_list.display();
    

    let _ = my_list.delete_from_end();
    my_list.display();
    
}
