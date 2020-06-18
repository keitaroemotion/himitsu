use std::fs;

const CREDENTIAL_FILE_PATH: &str = "/usr/local/etc/him/.credential";

fn main() {
    let cipher = read_file(CREDENTIAL_FILE_PATH);
    println!("{}", cipher);
}

//
// get encrypted file path and read the ciphered text
//
fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
               .expect("Something went wrong reading the file");
}

fn read_credentials() {
    // action 1: read credentials
    //  - decrypt credentials - asks for user input (pass) to decrypt
    //  - parse and get required credentials
    //  - set on the clipboard
}

fn write_credentials() {
    // action 2: write credentials
    // - enrypt credentials
}
