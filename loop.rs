fn main() {
    let mut counter = 0;
    let counter = loop
     {
         println!("Print this line five times");
         counter = counter + 1;
         if(counter == 5)
         {
             break
             counter
         }
};
println!("Loop runs {} times",counter)

}
