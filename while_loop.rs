fn main() {
    
let mut counter = 0;
let arr_of_nums = [1,2,3,4,55,66,88,4,11,1];

while counter < arr_of_nums.len()
{
    println!("No at index {} is {}",counter,arr_of_nums[counter]);
    counter = counter + 1;
}

println!("loop runs {} times",counter);


}
