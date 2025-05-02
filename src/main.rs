mod evaluation;
mod search;
mod interface;
use interface::uci::uci;
use std::env;
fn main() {
    env::set_var("RAYON_NUM_THREADS", "16");
    uci();
}
//need to add testing 

//Make the thread count equal to the number of cpu cores present or slightly less(ideally)
//After running and going through the code, if any suggestion are present, make a pull request
//Leave some of the comments in the code, they're to explain future ideas
//Urls are to be kept there, they either explain whats to be done... or explain a concept