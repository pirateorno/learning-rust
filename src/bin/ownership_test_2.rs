fn main() {
    let mut sentence = String::from("Hello my name is");
    let my_name = String::from("sasha");

    add_name(&mut sentence, &my_name);

    println!("{}", sentence);
}

fn add_name(original_string: &mut String, name: &str) {
    original_string.push(' ');
    original_string.push_str(name);
}