/**
 * Remove Duplicates From Sorted Array 2
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
 */

#[allow(dead_code)]

fn reverse_words(mut s: String) -> String {
    // Reversing String
    // Not Reqd but need to learn
    let mut i = 0;
    let mut j = s.len() - 1;
    let mut temp: String = s.chars().nth(i).unwrap().to_string();
    let mut temp2: String = s.chars().nth(j).unwrap().to_string();
    while i < s.len() / 2 && j > s.len() / 2 {
        s.replace_range(i..(i + 1), &temp2);
        s.replace_range(j..j + 1, &temp);

        i += 1;
        j -= 1;

        temp = s.chars().nth(i).unwrap().to_string();
        // let temp = s.get(i..i + 1).unwrap();
        temp2 = s.chars().nth(j).unwrap().to_string();
        // let temp2 = s.get(j - 1..j).unwrap();
    }

    // Using 2 pointer Method
    let mut index = 0;
    let mut jndex = 1;
    while jndex < s.len() {
        if s.chars().nth(jndex).unwrap() == ' ' && s.chars().nth(index) != s.chars().nth(jndex) {
            let sub_string = s.get(i..j).unwrap();
            // Reverse String from i..j
            s.replace_range(i..j, sub_string.chars().rev().collect());
        }
        j += 1;
    }

    return s;
}

fn main() {
    let s: String = "the sky is blue".into();
    println!("Hello, world!");
    println!("String = {}", s);

    println!("Result {:?}", reverse_words(s));
}
