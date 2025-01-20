fn main() {
    let mut x = 5;
    {
        let y = &mut x; 
        *y = 10;
    }
    let z = &mut x;
    println!("{}", *z); // This will print 10
} 