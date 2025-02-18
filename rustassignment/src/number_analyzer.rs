fn even(n: i32) -> bool{
    return n%2 == 0;
}

fn main(){
    let numbers = [12, 7, 5, 15, 20, 8, 3, 9, 30, 2];

    println!("Number Analysis:");

    for num in numbers{
        if num % 3 == 0 && num % 5 == 0{
            println!("{}: FizzBuzz", num);
        }
        else if num % 3 == 0{
            println!("{}: Fizz", num);
        }
        else if num % 5 == 0{
            println!("{}: Buzz", num);
        }
        else if even(num){
            println!("{}: Even", num);
        }
        else{
            println!("{}: Odd", num);
        }
    }

    let mut sum = 0;
    let mut i = 0;

    while i < numbers.len(){
        sum += numbers[i];
        i += 1;
    }

    println!("\nSum of numbers: {}", sum);

    let mut largest = numbers[0];
    for num in numbers{
        if num > largest{
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
   