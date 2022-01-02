use ethplorer::get_last_block;

fn main() {
    let res = get_last_block("");
    match res {
        Ok(t) => println!("{:?}", t),
        Err(e) => println!("{:?}", e),
    }
}
