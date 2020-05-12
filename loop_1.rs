fn main() {
   let mut counter = 0;
   let loop_times = loop {
       println!("Loop is runing");
       counter = counter + 1;
       if counter == 3
       {
           break counter //expression
       }
   }; //statement

   println!("Loop run {} times",loop_times);

}
