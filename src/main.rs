pub mod lr; // synthesized by LALRPOP
pub mod util;

fn main() {

    let input_line = String::from("abcdefghijklmnop<^[[1;1Haaaaa");
        
    println!("{:?}", lr::parse_Screen(&input_line.to_string()));
}
