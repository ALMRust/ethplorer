use ethplorer_rs::get_top_token_holders;

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!(
        "{:?}",
        get_top_token_holders("0xdf9d4674a430bdcc096a3a403128357ab36844ba", 1000, "")
    );
}
