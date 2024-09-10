fn main() {
    let firstname = String::from("Merve");
    let lastname = String::from("Keser");

    let test = concatenate_strings(&firstname, &lastname);
    println!("Concatenate Strings {}", test);
}

fn concatenate_strings(first_text: &str, second_text: &str) -> String {
    let mut result = String::new();
    result.push_str(first_text);
    result.push_str(second_text);
    result
}
