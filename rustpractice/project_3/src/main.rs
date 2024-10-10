fn main() {
    let target_number = 42; 
    let mut guesses_taken = 0; 
    let mut correct_guess = false;

    let mut guess = 100; 

    loop {
        

        guesses_taken += 1; 
        let result = check_guess(guess, target_number); 

       
        if result == 0 {
            println!("Congratulations! Your guess is correct.");
            correct_guess = true; 
            break; 
        } else if result == -1 {
            println!("Your guess is too low. Try again!");
            guess += 1;
            
        } else {
            println!("Your guess is too high. Try again!");
            guess -= 1;
            
            
            
        }

        
       
    }

    
    println!("It took you {} guesses.", guesses_taken);
   // println!("{}", guess);
}


fn check_guess(guess: i32, target_number: i32) -> i32 {
    if guess == target_number {
        0
    } else if guess > target_number {
        1
    } else {
        -1
    }
}