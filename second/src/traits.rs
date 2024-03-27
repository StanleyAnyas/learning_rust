// use std::fmt::format;

fn main() {
    let first_tweets = Tweet {
        username: "John Smith".to_string(),
        content: "Hey algorithm I want to connect with software developer and rich people"
            .to_string(),
        reply: false,
        retweet: false,
    };

    notify(&first_tweets);

    println!("{}", first_tweets.summarize());

    let user1 = User {
        name: String::from("Stanley"),
        last_name: String::from("Anyas"),
        age: 22,
        account_id: 234987,
        is_owing: false,
        how_much_owing: 0.0,
    };

    println!("the user details is {}", user1.user_details());
    println!("is the user owning: {}", user1.is_owning());
    println!("how much is the user owning: {}", user1.how_much_owned());
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "the following tweet has been retweeted by {}",
            self.username
        )
    }
}

pub fn notify<T: Summary>(item: &T) {
    // instead of usinf the trait bound notify<T: Summary>(item: &T) you can also us the impl which is notify(item: &impl Summary)
    println!("Breaking news! {}", item.summarize());
}

struct User {
    name: String,
    last_name: String,
    age: i32,
    account_id: i32,
    is_owing: bool,
    how_much_owing: f32,
}

pub trait Details {
    fn user_details(&self) -> String;

    fn is_owning(&self) -> bool;

    fn how_much_owned(&self) -> String;
}

impl Details for User {
    fn user_details(&self) -> String {
        format!(
            "{} {}, age {}, account id {}",
            self.name, self.last_name, self.age, self.account_id
        )
    }

    fn is_owning(&self) -> bool {
        self.is_owing
    }

    fn how_much_owned(&self) -> String {
        self.how_much_owing.to_string()
    }
}
