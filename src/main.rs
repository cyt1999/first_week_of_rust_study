mod layer_one;

fn main() {
    // 97 - 90
    println!("Printing from 'a' to 'Z':");
    layer_one::print_a_to_z();

    // 65 - 122
    println!("\nPrinting from 'A' to 'z':");
    layer_one::layer_two::print_a_to_z();
}
