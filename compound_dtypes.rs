fn main() {
 //Two types of compound data types are in rust i.e array and tuple
 //In rust tuples contains heterogenous data while arrays contains homogenous data

//Tuples

 let student = ("waqar",25,85.6);

 println!("{}",student.0);
 println!("{}",student.1);
 println!("{}",student.2);

 //access tuples by destructuring

let (x,y,z) = student;

println!("{}",x);
println!("{}",y);
println!("{}",z);

//Arrays

let nums:[u32;6] = [1,52,32,344,5,16];
println!("{}",nums[0]);
println!("{}",nums[1]);
println!("{}",nums[2]);

let nums_five_ten_times =[5;10];
println!("{:#?}",nums_five_ten_times);






}
