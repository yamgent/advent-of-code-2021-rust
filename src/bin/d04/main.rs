mod original;
mod shortcircuit;

fn main() {
    println!("Original:");
    original::main_original();

    println!();
    println!("Short-circuit:");
    shortcircuit::main_shortcircuit();
}
