// use core::num;

fn length_of_last_word(s: String) -> i32 {
    let trimed_s = s.trim();
    let parts: Vec<&str> = trimed_s.split_whitespace().collect();
    let len_of_part = parts.len();
    parts[len_of_part - 1].len() as i32
}

fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() == 1 && nums[0] == target && nums[0] != 0 {
        return vec![nums];
    }
    nums.sort();
    let mut new_vec = Vec::new();
    let mut hign = nums.len() - 1;
    let mut low = 0;

    while low < hign {
        let mut left = low + 1;
        let mut right = hign - 1;

        while left < right {
            let sum = nums[low] + nums[hign] + nums[left] + nums[right];

            match sum {
                s if s == target => {
                    let mut quadruplet = vec![nums[hign], nums[low], nums[left], nums[right]];
                    quadruplet.sort();
                    new_vec.push(quadruplet);

                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
                s if s < target => left += 1,
                _ => right -= 1,
            }
        }
        low += 1;
        hign -= 1;

        while low < hign && nums[low] == nums[low - 1] {
            low += 1;
        }

        while low < hign && nums[hign] == nums[hign + 1] {
            hign -= 1;
        }
    }

    new_vec
}

// fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//     if nums.len() == 1 && nums[0] == target {
//         return vec![nums];
//     }
//     nums.sort();
//     let mut new_vec = Vec::new();
//     let n = nums.len();

//     for i in 0..n - 3 {
//         if i > 0 && nums[i] == nums[i - 1] {
//             continue; // Skip duplicates
//         }
//         for j in i + 1..n - 2 {
//             if j > i + 1 && nums[j] == nums[j - 1] {
//                 continue; // Skip duplicates
//             }
//             let mut left = j + 1;
//             let mut right = n - 1;

//             while left < right {
//                 let sum = nums[i] + nums[j] + nums[left] + nums[right];
//                 if sum == target {
//                     new_vec.push(vec![nums[i], nums[j], nums[left], nums[right]]);
//                     while left < right && nums[left] == nums[left + 1] {
//                         left += 1; // Skip duplicates
//                     }
//                     while left < right && nums[right] == nums[right - 1] {
//                         right -= 1; // Skip duplicates
//                     }
//                     left += 1;
//                     right -= 1;
//                 } else if sum < target {
//                     left += 1;
//                 } else {
//                     right -= 1;
//                 }
//             }
//         }
//     }

//     new_vec
// }

fn main() {
    let s = String::from("   fly me   to   the moon  ");
    let result = length_of_last_word(s);
    println!("the length of the last word is: {result}");

    let nums = vec![-3, -1, 0, 2, 4, 5];
    let target = 0;
    let four_sum_result = four_sum(nums, target);
    println!("{:?}", four_sum_result);
}
