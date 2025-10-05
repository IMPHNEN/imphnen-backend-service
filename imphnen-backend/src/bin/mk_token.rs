use imphnen_libs::jsonwebtoken::encode_access_token;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: mk_token <email_or_sub>");
        std::process::exit(1);
    }
    let sub = args[1].clone();
    // Use sub as both sub and user_id
    match encode_access_token(sub.clone(), sub.clone()) {
        Ok(token) => println!("{}", token),
        Err(e) => {
            eprintln!("Failed to generate token: {:?}", e);
            std::process::exit(2);
        }
    }
}
