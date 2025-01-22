use dirs;
use cmd_lib::run_cmd;

use super::account;
use super::json;

// Finds where fmp's vault is
//
// USAGE
//
// let var: String = get_fmp_vault_location();
pub fn get_fmp_vault_location() -> String{
    // Gets users home directory
    let home_dir = dirs::home_dir().expect("Could not find home directory!");
    // Appends directory name to end of home directory
    let fmp_vault_location = home_dir.join(".fmpVault");

    return fmp_vault_location.display().to_string();
}

// Encrypts the .fmpVault file to .fmpVault.tar.gz.gpg
//
// USAGE
//
// encrypt_fmp_vault();
pub fn encrypt_fmp_vault() {
    // Gets locations
    let fmp_vault_location = get_fmp_vault_location();
    let fmp_vault_as_encrypted_tar = format!("{}.tar.gz.gpg", fmp_vault_location);

    println!("Encrypting fmp vault...\n");

    // Encrypts .fmpVault
    run_cmd!(tar -cz $fmp_vault_location | gpg -c -o $fmp_vault_as_encrypted_tar).expect("Failed to execute command");

    println!("\nEncrypted!")
}

// Decrypts the .fmpVault.tar.gz.gpg file to .fmpVault
//
// USAGE
//
// decrypt_fmp_vault();
pub fn decrypt_fmp_vault() {

    let fmp_vault_location = get_fmp_vault_location();
    let fmp_vault_as_encrypted_tar = format!("{}.tar.gz.gpg", fmp_vault_location);

    println!("Decrypting fmp vault...\n");

    run_cmd!(gpg -d $fmp_vault_as_encrypted_tar | tar xz).expect("Failed to execute command");

    println!("\nDecrypted");
}

// Reads all json files and prints to screen
//
// USAGE
//
// read_vault() 
pub fn read_vault(){
    // Gets list of accounts
    let accounts_list: Vec<String> = account::read_account(account::get_account_location());
        // Loop for each entry in accounts_list
        for i in 0..accounts_list.len() {
            // Find corrosponding json file and read
            let service = accounts_list[i].clone();
            let json = json::read_json(get_fmp_vault_location(), service);
            // Output
            println!("{}: Username: {} Password: {}", accounts_list[i], json.username, json.password)
        }
}