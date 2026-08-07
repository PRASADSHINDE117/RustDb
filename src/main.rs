pub mod catalog;
pub mod concurrency;
pub mod execution;
pub mod parser;
pub mod server;
pub mod storage;
pub mod resources;
fn main() {
    println!("Starting rustdb_ps...");
    resources::variable::variable_docs(10);
}
