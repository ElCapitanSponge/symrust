use std::env;
use symlink;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        match symlink::symlink_auto(&args[1], &args[2]) {
            Ok(result) => { result },
            Err(error) => {return Err(error.into())}
        };
    } else {
        return Err("Too few, or too many arguments provided! Source location and desitination are required".into());
    }
    Ok(())
}

