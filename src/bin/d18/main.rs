mod naive;
mod tree;

fn main() {
    println!("Naive");
    naive::solve_naive();
    println!();

    println!("Tree");
    tree::solve_tree();
}
