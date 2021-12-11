use ethplorer_rs::{get_token_info};

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!(
        "{:?}",
        get_token_info("0xdf9d4674a430bdcc096a3a403128357ab36844ba", "")
    );
}
