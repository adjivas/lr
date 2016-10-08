pub mod lr; // synthesized by LALRPOP
pub mod util;

fn main() {

    let input_line = String::from("abcdefghijklmnop<^[[1;1Haaaaa");
        
    println!("{:?}", lr::parse_Screen(&input_line.to_string()) );

    let input_line = String::from("aAbcdefghijklmnop<^[[1;1H1A~1aaaa1");
        
    println!("{:?}", lr::parse_Screen(&input_line.to_string()) );
}
