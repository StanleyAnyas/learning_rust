pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){
            println!("add_to_waitlist");
        }
    }
}

pub fn main() {
    crate::front_of_house::hosting::add_to_waitlist();
}