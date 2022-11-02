use better_file_maker;
mod questions;

fn main() {
    better_file_maker::make_folders("output").unwrap_or(());
    println!("output file created!");
    
}
