#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let r;

    {
        let x = 5;
        r = x;
    }

    println!("r: {}", r);

    let name = "Stanley";
    let surname = "Anyas";

    let fullname = name_and_surname(name, surname);
    println!("fullname: {}", fullname);

    let novel = String::from("Alfa married Hamza. Nassor Biashara was a good man, after all");
    let sentence = novel.split(',').next().expect("could not find a ','");
    let part_of_novel = ImportantExcerpt { part: sentence };
    println!("part of sentence: {:?}", part_of_novel);
}

fn name_and_surname<'a>(name: &'a str, surname: &'a str) -> String {
    let fullname = format!("{} {}", name, surname);
    fullname
}
