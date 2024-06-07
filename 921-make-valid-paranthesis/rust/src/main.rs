fn min_add_to_make_valid(s: String) -> i32 {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        if stack.len() > 0 && stack[stack.len() - 1] == '(' && ch == ')' {
            stack.pop();
            continue;
        }
        stack.push(ch);
    }

    return stack.len() as i32;
}

fn main() {
    println!("Hello, world!");
    let input = "((()))".to_string();
    println!(
        "Opertaions Reqd to Make Paranthesis Valid is {}",
        min_add_to_make_valid(input)
    );
}
