use std::env;

pub fn parse() {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if let Some(arg) = args.get(i) {
            match arg.as_str() {
                
                _ => continue
            }
        } else {
            return;
        }
    }
}

