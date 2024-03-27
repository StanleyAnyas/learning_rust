use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
use std::fs::Permissions;
use std::fs;

fn main(){
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
        },
    };
    if let Err(error) = f.write_all(b"Hello, world!") {
        panic!("Problem writing to the file: {:?}", error);
    }
    // f.write_all(b"Hello, world!");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         match error.kind() {
    //             ErrorKind::PermissionDenied => {
    //                 match fs::set_permissions("hello.txt", Permissions::from_mode(0o111)) {
    //                     Ok(_) => match File::open("hello.txt") {
    //                         Ok(fc) => fc,
    //                         Err(e) => panic!("Problem opening the file: {:?}", e),
    //                     },
    //                     Err(e) => panic!("Problem setting the file permission: {:?}", e),
    //                 }
    //             }
    //         }
    //     }
    // };

    let mut vec = vec![2, 4, 6, 8];
    let result = multiple_num(&mut vec);
    println!("{:?}", result);
}

fn multiple_num(vec: &mut Vec<u8>) -> Vec<u8> {
    vec.iter_mut().for_each(|x| *x *= 2);
    vec.to_vec()
}

fn small_fn(words: &str) -> String {
    for i in words{
        if i == ' '{
            return words[..i].to_string();
        }
    }
    words.to_string()
}