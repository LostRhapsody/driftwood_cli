pub mod cli;
pub mod lib;
pub mod netlify;

pub mod crypto;
use dotenv::dotenv;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    cli::draw_menu();

    // When initiating OAuth flow:
    //let (private_key, public_key) = generate_key_pair();
    //let public_key_pem = get_public_key_pem(&public_key);
    
    // When receiving the encrypted token:
    //let decrypted_token = decrypt_token(&encrypted_token, &private_key);

    // Send public_key_pem to server along with OAuth request
    Ok(())
}