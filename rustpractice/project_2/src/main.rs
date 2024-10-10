fn main() {
    
    let numbers = [10, 15, 23, 42, 55, 60, 73, 84, 90, 101];

    
    for number in numbers.iter() {
        if number % 3 == 0 && number % 5 == 0 {
            println!("{}: FizzBuzz", number);
        } else if number % 3 == 0 {
            println!("{}: Fizz", number);
        } else if number % 5 == 0 {
            println!("{}: Buzz", number);
        } else {
            let even_status = is_even(*number);
            if even_status {
                println!("{}: Even", number);
            } else {
                println!("{}: Odd", number);
            }
        }
    }

    
    let mut sum = 0;
    let mut index = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    let mut largest = numbers[0];

    for &number in &numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number: {}", largest);
}


fn is_even(n: i32) -> bool {
    n % 2 == 0
}