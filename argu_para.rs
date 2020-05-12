fn main() {
    
let x = 12;
let y = 14;

let (s,p) = cal(2.0,0.3);
println!("product of two numbers are {}",p);
println!("Sum of two numbers are {}",s);


}


fn cal(x : f32, y : f32)->(f32, f32){

    let z = x + y;
    let c = x * y;
    (z,c)

}