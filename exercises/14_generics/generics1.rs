// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
    let s = String::from("123");
    shopping_list.push(s.as_str());
}
