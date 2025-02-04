fn main() {
    let mut x = 5;
    { // Limiting the scope of mutable references prevents data races
        let y = &mut x; 
        *y += 1; 
    }
    { // Limiting the scope of mutable references prevents data races
        let z = &mut x;
        *z += 1;
    } 
    println!("x = {}", x);
}
