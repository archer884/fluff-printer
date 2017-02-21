#![feature(conservative_impl_trait)]

extern crate rand;

mod format;
mod model;

use format::ModelFormatter;
use model::{ModelGenerator, ModelIterator};

fn main() {
    let data = generate();
    let out = std::io::stdout();
    
    let mut formatter = ModelFormatter::new(data, out);
    formatter.write().expect("Unable to write to stdout");
}

fn generate() -> impl ModelIterator {
    ModelGenerator::default().take(10)
}
