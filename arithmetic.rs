fn main() {

    /*Rust is static typed language so it must know each variable type before compiling while other langauges are
    dynamically type so they set variables accordinly */

    let num_1 = 8;
    let num_2 = 2;

    //if I convert any of these to floating point number error arises as in rust we cannot add int and float 

    println!("{}",num_1 + num_2);
    println!("{}",num_1 - num_2);
    println!("{}",num_1 / num_2);
    println!("{}",num_1 * num_2);
    println!("{}",num_1 % num_2);

}
