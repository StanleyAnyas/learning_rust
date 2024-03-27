use std::cell::RefCell;

fn main() {
    let name = RefCell::new("test_name");
    {
        *name.borrow_mut() = "test_name2";
        println!("name1: {:?}", name);
    }
    let another_name = name.borrow();
    println!("name2: {:?}", another_name);
}
