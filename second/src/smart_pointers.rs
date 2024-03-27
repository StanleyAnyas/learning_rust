use std::ops::Deref;
use std::rc::Rc;
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

enum List2<T> {
    Cons2(T, Rc<List2<T>>),
    Nil2,
}

#[derive(Debug)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            val: value,
            right: None,
            left: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value <= self.val {
            match &mut self.left {
                Some(node) => node.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        } else {
            match &mut self.right {
                Some(node) => node.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        }
    }

    fn inorder_traversal(&self) {
        if let Some(node) = &self.left {
            node.inorder_traversal();
        }
        if let Some(node) = &self.right {
            node.inorder_traversal();
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> MyBox<T> {
        MyBox(val)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::List::{Cons, Nil}; // this is using the Box smart pointer
use crate::List2::{Cons2, Nil2}; // this is using the Rc smart pointer
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let _my_cons_list = Cons(1, Box::new(Cons(32, Box::new(Cons(11, Box::new(Nil))))));
    // let points_to_my_cons_list = Cons(1, Box::new(_my_cons_list));
    // let again_points_to_my_cons_list = Cons(4, Box::new(_my_cons_list)); // i can't do this because the borrow checker will give me an error

    let my_cons_list2 = Rc::new(Cons2(
        1,
        Rc::new(Cons2(32, Rc::new(Cons2(11, Rc::new(Nil2))))),
    ));
    let _points_to_my_cons_list2 = Cons2(45, Rc::clone(&my_cons_list2));
    let _again_points_to_my_cons_list2 = Cons2(1, Rc::clone(&my_cons_list2)); // now this won't give any error beacause i'm using the Rc smart pointer

    let x;
    {
        let y = Rc::new(3);
        x = Rc::clone(&y);
    }

    println!("{}", x);

    println!("{}", Rc::strong_count(&x));

    println!("recursive: {}", recursion_count(32, 3));

    let mut root = Node::new(71);
    root.insert(61);
    root.insert(21);
    root.insert(1);
    root.inorder_traversal();

    println!("root: {:?}", root);

    let to_be_dropped = |x: i32, y: &str| {
        // this is a closure
        println!("this is a closure and has {x} and {y}");
    };

    to_be_dropped(12, "hello");

    let mut my_map = vec![21, 22, 23, 24, 25, 26, 27, 28, 29];
    let added: Vec<i32> = my_map
        .iter()
        .filter(|x| **x != 21)
        .map(|&x| if x == 21 { x } else { x - 1 })
        .collect();
    println!("the vector: {:?}", added);
    println!("the vector length: {:?}", my_map.len());
    my_map.push(78);
}

fn recursion_count(x: i32, num_to_increment: i32) -> i32 {
    if x <= 0 {
        0
    } else {
        num_to_increment + recursion_count(x - 1, num_to_increment)
    }
}
