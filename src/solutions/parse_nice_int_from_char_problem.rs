//You ask a small girl,"How old are you?" She always says, "x years old", where x is a random number between 0 and 9.
//Write a program that returns the girl's age (0-9) as an integer.
//Assume the test input string is always a valid string. For example, the test input may be "1 year old" or "5 years old". The first character in the string is always a number.

pub fn get_age(age: &str) -> u32 {
    let first_char: String = age.get(0..1).expect("invalid data").to_string();
    let first_char_int: u32 = first_char.trim().parse::<u32>().expect("unable to parse");
    first_char_int
}

pub fn another_soultion(age: &str) -> u32 {
    age[..1].parse().unwrap()
}
