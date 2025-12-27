fn main() {
    println!("testing shit");

    let mut s = String::from("hello");
    testing_shit(&mut s);

    println!("{}", s);
}

fn testing_shit(some_string: &mut String){
    some_string.push_str("_banan");
}