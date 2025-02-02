// Rust: Corrected Version - Ownership & Borrowing

fn main() {
    let s = String::from("Hello");
    let s2 = &s; // Borrowing instead of moving ownership
    println!("{}", s2); // Now valid since s is borrowed, not moved
}
