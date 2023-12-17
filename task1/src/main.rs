// create a function which takes two string as arguments and returns a string
fn concatenate_strings(a: &str, b: &str) -> String {
    let mut result = String::new(); // mutable result variable
    result.push_str(a); // pushing a to result
    result.push_str(b); // pushing b to result
    result // return result
}

// implement the main function
fn main() {
    let string1 = String::from("string one "); // create string1 variable
    let string2 = String::from("string two "); // create string2 variable
    let concatenated_string = concatenate_strings(&string1[..], &string2[..]); // create a concatenated string variable and assign concatenate_strings function to it
    println!("Concatenated string: {}", concatenated_string); // print the concatenated string
}