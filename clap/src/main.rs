extern crate clap; 
use clap::App; 
 
fn main() { 
    App::new("myapp")
       .version("1.0")
       .about("Does great things!")
       .author("Kevin K.")
       .get_matches(); 
}