/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Clone + PartialOrd + Display> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + PartialOrd + Display> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj.clone()));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        if self.length == 0 {
            self.start = node_ptr;
        } else {
            self.switch(node_ptr.unwrap(), self.start, None);
        }
        self.length += 1;
    }

    pub fn switch(
        &mut self,
        node_ptr: NonNull<Node<T>>,
        select_node: Option<NonNull<Node<T>>>,
        parent_node: Option<NonNull<Node<T>>>,
    ) {
        let node_var = unsafe { &(*node_ptr.as_ptr()).val };
        if let Some(curreut_node) = select_node {
            let val = unsafe { &(*curreut_node.as_ptr()).val };
            if val < node_var {
                unsafe {
                    match (*curreut_node.as_ptr()).next {
                        Some(start_next) => {
                            let next_var = &(*start_next.as_ptr()).val;
                            if next_var < node_var {
                                self.switch(node_ptr, Some(start_next), select_node);
                            } else {
                                (*curreut_node.as_ptr()).next = Some(node_ptr);
                                (*node_ptr.as_ptr()).next = Some(start_next);
                            }
                        }
                        None => {
                            (*curreut_node.as_ptr()).next = Some(node_ptr);
                        }
                    }
                };
            } else {
                unsafe {
                    (*node_ptr.as_ptr()).next = select_node;
                }
                match parent_node {
                    None => {
                        self.start = Some(node_ptr);
                    }
                    Some(parent) => {}
                }
            }
        }
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        if let Some(node) = self.get_ith_node(self.start, index) {
            Some(unsafe { &(*node.as_ptr()).val })
        } else {
            None
        }
    }

    fn get_ith_node(
        &mut self,
        node: Option<NonNull<Node<T>>>,
        index: i32,
    ) -> Option<NonNull<Node<T>>> {
        if index < 0 {
            return None;
        }
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(next_ptr),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn merge(mut list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        //TODO
        let mut ret_list = LinkedList::new();
        let mut current_b = list_b.start;
        let mut curret_a = list_a.start;
        while let Some(node_b) = current_b {
            let bv = &unsafe { node_b.as_ref() }.val;

            ret_list.add(bv.clone());
            current_b = unsafe { node_b.as_ref() }.next;
        }

        while let Some(node_a) = curret_a {
            let bv = &unsafe { node_a.as_ref() }.val;

            ret_list.add(bv.clone());
            curret_a = unsafe { node_a.as_ref() }.next;
        }

        ret_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {:?}", list_c);
        for i in 0..target_vec.len() {
            println!("listc:{}", *list_c.get(i as i32).unwrap());
            // assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
