fn main() {

    let arr = [1,2,3,4,5,6,7];


    for element in arr.iter()
    {
        println!("Element is {}",element);
    }

    for num in 0..10{
        println!("number is {}",num)
    }

    for num2 in (0..10).rev()
    {
        println!("number is {}",num2)
    }



}
