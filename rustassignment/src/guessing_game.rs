fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        return 0;
    }
    else if guess > secret{
        return 1;
    }
    else{
        return -1;
    }
}

fn main(){
    let secret = 7;
    let mut guess = 0;
    let mut attempts = 0;

    println!("Guess number");

    loop{
        guess +=1;
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0{
            println!("{} is correct! You guessed it in {} attempts.", guess, attempts);
            break;
        }
        else if result == 1{
            println!("{}is too high", guess);
        }
        else{
            println!("{} is too low", guess);
        }
    }
}