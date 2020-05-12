fn main() {
    let number = 8;
    let fact = factorial(number);
    println!("Factorial of {} is {}",number,fact);
}


fn factorial(mut x:i32)->i32{
    let mut num = 1;
    while(x != 1){
        num = num * x;
        x = x - 1;

    }
    num
}
