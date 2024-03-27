#[allow(unused_assignments)]
#[allow(unused_mut)]
fn main() {
    // println!("max u32: {}", std::u32::MAX);
    // println!("max u64: {}", std::u64::MAX);
    // println!("max usize: {}", std::usize::MAX);
    // println!("max u128: {}", std::u128::MAX);
    // println!("max f32: {}", std::f32::MAX);
    // println!("max f64: {}", std::f64::MAX);

    let _is_true: bool = true; // if you have any unsed variable, you can prefix it with an underscore to let the compiler ignore it

    let mut num = 1;
    num += 3;
    assert_eq!(num, 4);
    // println!("num = {}", num);

    let mut s = String::from("");
    s.push_str("Hello, world!");
    let _copy = s.clone();
    // println!("s = {}", s);
    let mut name = String::from("John");
    let len = calculate_length(&name);
    println!("len = {}", len);
    println!("name = {}", name);
    let full_name = change(&mut name);
    println!("full_name = {}", full_name);

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    fn change(name: &mut String) -> String {
        name.push_str(" Doe");
        name.to_string()
    }

    let mut zen = String::from("zen");
    // zen = "zed".to_string();
    println!("{}", zen);
    let r1 = &zen;
    let r2 = &zen;

    println!("r1 = {}, r2 = {}, zen = {}", r1, r2, zen);

    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.into_bytes();
    _s.iter().for_each(|x| print!("{} ", x));
}