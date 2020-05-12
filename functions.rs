fn main() {
    println!("Hello, world!");
    
    another_func();
    let sum_of_num = add(2,3);
    println!("Sum is {}",sum_of_num);
    let (small,great) = max_min(3,11);
    println!("Maximum number is {}",great);
    println!("Minimum number is {}",small);
}

fn another_func()
{
    println!("Here is another function");
}

// Functions with parameters you must have to specify the datatype for parameters

fn add(x : i32, y : i32) -> i32
{
    println!("Value of x is {}",x);
    println!("Value of y is {}",y);
    let sum_num = x + y;
    sum_num
}

fn max_min(x:i32, y:i32) ->(i32,i32){

    if(x > y)
    {
        (y,x)
    }
    else
    {
        (x,y)
    }
}
