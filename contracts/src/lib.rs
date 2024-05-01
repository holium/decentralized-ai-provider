// load the abi code for calling the contract from ./out/contracts.json

use std::io::Read;
use std::{error::Error, fs::File}; // Import the Read trait

pub fn load_abi() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("./out/AppRegistry/AppRegistry.json")?;
    let mut abi = String::new();
    file.read_to_string(&mut abi)?;
    Ok(abi)
}
