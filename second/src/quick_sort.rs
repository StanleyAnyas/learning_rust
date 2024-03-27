fn quick_sort(arr: &Vec<i32>) -> Vec<i32> {
    if arr.is_empty() || arr.len() == 1 {
        return arr.to_vec();
    }

    let middle_point = arr.len() / 2;
    let pivot = arr[middle_point];

    let mut remainat1: Vec<i32> = arr[..middle_point].to_vec();
    let mut remainat2: Vec<i32> = arr[middle_point + 1..].to_vec();
    remainat1.append(&mut remainat2);
    let whole_remainat = remainat1;
    let mut left = Vec::new();
    let mut right = Vec::new();

    for num in whole_remainat {
        if num < pivot {
            left.push(num);
        } else {
            right.push(num);
        }
    }

    quick_sort(&left)
        .into_iter()
        .chain(Some(pivot))
        .chain(quick_sort(&right))
        .collect()
}

fn binary_search(arr: &Vec<i32>, target: i32) -> bool {
    if arr.is_empty() {
        return false;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let middle_point = (left + right) / 2;
        match arr[middle_point] {
            n if n == target => {
                return true;
            }
            n if n < target => {
                left = middle_point + 1;
            }
            _ => {
                right = middle_point - 1;
            }
        }
    }
    false
}

fn main() {
    let my_vec = vec![2, 1, 54, 7, 11, 6, 9, 3];
    let result = quick_sort(&my_vec);
    println!("{:?}", result);

    let search = binary_search(&result, 11);
    println!("the search was {search}")
}
