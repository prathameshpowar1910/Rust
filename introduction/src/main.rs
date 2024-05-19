fn main() {
    println!("Hello, world!");

    // calling the add function
    let sum: i32 = add(6, 11);
    println!("The sum is: {}", sum);

    //String data type
    let greeting: String = String::from("Hello world");
    println!("{}", greeting);
    // let char1 = greeting.chars().nth(1000);
    let char1: Option<char> = greeting.chars().nth(1);
    // print!("{}",char1.unwrap());
    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character found"),
    }

    //conditionals
    let is_even: bool = sum % 2 == 0;

    if is_even {
        println!("The number is even");
    } else if !is_even {
        println!("The number is odd");
    } else {
        println!("The number is not a number");
    }

    //loops
    for tp in 1..11 {
        println!("The number is: {}", tp);
    }

    let num: i32 = 10;
    for _i in 0..num {
        println!("The number is:");
    }

    //iterating over a string
    let sentence: String = String::from("Hellow its peabody");
    // let first_word = first_word(&sentence);
    let first_word = get_first_word(sentence);
    println!("The first word is: {}", first_word);
}

// function that takes two numbers and returns the sum
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn get_first_word(sentence: String) -> String {
    // let answer: String = String::new();
    let mut answer: String = String::from("");
    for char in sentence.chars() {
        answer.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return answer;
}

//how to initialise a rust project steps
//1. cargo new project_name or cargo init i your project file
//2. cd project_name
//3. cargo build
//4. cargo run
//5. cargo check
//6. cargo test
//7. cargo doc --open
//8. cargo update
//9. cargo clean
//10. cargo fmt

//the different data types in rust are

//1. i8, i16, i32, i64, i128, isize
//2. u8, u16, u32, u64, u128, usize
//3. f32, f64
//4. bool
//5. char
//6. arrays
//7. tuples
//8. slices
//9. strings
//10. vectors
//11. hashmaps
//12. enums
//13. structs
// and many more
