
//PER APRIRE UN FILE ESTERNO
//----------------------------------------------------------------------------//
use std::process::Command; //libreria per eseguire i comandi (cmd)
//---------------------------------------------------------------------------//

pub fn call(path: &str) {
    // chiama il Bat e a sua volta EXE
    Command::new(path)
            .output()
            .expect("failed to execute process");
    println!("FATTO call bat!");
    
}//DONE


