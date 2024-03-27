// use std::os::windows::io::HandleOrInvalid;


#[derive(Clone)]
pub struct LinkedList<T> {
    value: Option<T>,
    prev: Option<Box<LinkedList<T>>>,
    next: Option<Box<LinkedList<T>>>,
    length: i32,
    head: Option<Box<LinkedList<T>>>,
    tail: Option<Box<LinkedList<T>>>,
}

impl LinkedList<i32> {
    fn new() -> Self {
        LinkedList {
            value: None,
            prev: None,
            next: None,
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, value: i32) {
        if self.length == 0 { 
            let new_node = Box::new(LinkedList {
                value: Some(value),
                prev: None,
                next: None,
                length: self.length,
                head: None,
                tail: None,
            });
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            self.length += 1
        }else {
            let mut new_node = Box::new(LinkedList {
                value: Some(value),
                prev: None,
                next: None,
                length: self.length,
                head: None,
                tail: None,
            });
            if let Some(mut tail) = self.tail.take() {
                tail.next = Some(new_node.clone());
                new_node.prev = Some(tail);
                self.tail = Some(new_node);
                self.length += 1;
            }
        }
    }

    fn prepend(&mut self, value: i32){
        if self.length == 0 {
            let new_node = Box::new(LinkedList {
                value: Some(value),
                prev: None,
                next: None,
                length: self.length,
                head: None,
                tail: None,
            });
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
            self.length += 1
        }else {
            let mut new_node = Box::new(LinkedList{
                value: Some(value),
                prev: None,
                next: None,
                length: self.length,
                head: None,
                tail: None,
            });

            if let Some(mut head) = self.head.take() {
                head.prev = Some(new_node.clone());
                new_node.next = Some(head);
                self.head = Some(new_node);
                self.length += 1;
            }
        }
    }

    fn get_first(&self) -> i32 {
        if let Some(head) = &self.head {
            if let Some(value) = &head.value {
                return *value;
            }
        }
        panic!("List is empty");
    }

    fn get_last(&self) -> i32 {
        if let Some(tail) = &self.tail {
            if let Some(value) = &tail.value {
                return *value;
            }
        }
        panic!("List is empty")
    }
}

fn main() {
    let value = 3;
    let value2 = 16;
    let mut my_linked_list = LinkedList::new();
    my_linked_list.append(value);
    my_linked_list.prepend(value2);

    println!("{}", my_linked_list.get_last());
    println!("{}", my_linked_list.get_first());

}