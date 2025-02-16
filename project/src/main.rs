// // Assignment 1: Temperature Converter

// const FREEZING_POINT: f64 = 32.0;

// fn fahrenheit_to_celsius(f: f64) -> f64 {
//     (f-32.0) * (5.0/9.0)
// }

// fn celsius_to_fahrenheit(c: f64) -> f64 { 
//     (c*1.8) + 32.0
// }

// fn main() {
//     let mut f_temp = FREEZING_POINT;
//     let c_temp = fahrenheit_to_celsius(f_temp);
//     println!("Farenheit: {}, Celsius: {}", f_temp, c_temp);

//     for n in 1..6 {
//         let new_temp = fahrenheit_to_celsius(f_temp + n as f64);
//         println!("Farenheit: {}, Celsius: {}", (f_temp + n as f64), new_temp);
//     }
// }


// Assignment 2: Number Analyzer
// const XS: [i32; 10] = [1, 2, 3, 4, 20, 23, 6, 9, 15, 10];

// fn is_even(n: i32) -> bool {
//     n % 2 == 0
// }

// fn main() {
//     let mut sum = 0;
//     let mut j = 0;
//     let mut max = 0;

//     for n in XS {
//         if is_even(n) {
//             println!("{} is even", n);
//         } else {
//             println!("{} is odd", n);
//         }

//         if (n % 3 == 0) && (n % 5 == 0) {
//             println!("FizzBuzz");
//         } else if n % 3 == 0 {
//             println!("Fizz");
//         } else if n % 5 == 0 {
//             println!("Buzz");
//         }
//     }

//     // Find and print the sum of all the numbers in the array
//     let mut i = 0;
//     while i < XS.len() {
//         sum = sum + XS[i];
//         i += 1;
//     }
//     println!("Sum of the array: {}", sum);

//     // Find and print the largest number in the array

//     while j < (XS.len() - 1) {
//         if XS[j] > XS[j + 1] {
//             max = XS[j];
//         } else {
//             max = XS[j + 1];
//         }
//         j += 1;
//     }

//     println!("Largest number in the array: {}", max);

// }

// Assignment 3: Guessing Game
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let mut secret = 4;
    let mut guess = 7;
    let mut attempts = 0;

    while attempts < 5 {
        let confirm = check_guess(guess, secret);
        attempts += 1;

        if confirm == 0 {
            println!("Guess was correct!");
            break;
        } else if confirm == 1 {
            println!("Guess was too high!");
        } else {
            println!("Guess was too low!");
        }

        guess -= 1;
    }

    println!("Number of guesses: {}", attempts);


}