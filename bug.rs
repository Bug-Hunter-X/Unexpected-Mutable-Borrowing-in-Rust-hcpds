fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; 
    *y = 10;
    println!("{}", *z); // This will print 10
}