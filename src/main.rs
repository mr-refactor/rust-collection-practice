mod departments;
use std::error;
fn main() -> Result<(), Box<dyn error::Error>> {
    departments::run()
}
