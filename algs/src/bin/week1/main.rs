use std::env;

mod aplusb;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match &args[0][..] {
        "aplusb" => aplusb::run(),
        "maximumpairwise" => Ok(println!("not implemented yet")),
        _ => Ok(()) 
    }



}