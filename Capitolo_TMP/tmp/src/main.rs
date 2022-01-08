
//PER APRIRE UN FILE ESTERNO
//----------------------------------------------------------------------------//
use std::process::Command; //libreria per eseguire i comandi (cmd)
//---------------------------------------------------------------------------//
//const mut NUMERO_CONST: i32 = 10;
pub fn call(path: &str) {//CHIAMA UN PROCESSO FIGLIO
    // chiama il Bat e a sua volta EXE
    Command::new(path)
            .output()
            .expect("failed to execute process");
    println!("FATTO call bat!");
    
}//DONE
