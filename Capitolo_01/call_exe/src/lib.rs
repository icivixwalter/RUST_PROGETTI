
//PER APRIRE UN FILE ESTERNO
//----------------------------------------------------------------------------//
//FAQ: @apro.comando.bat; @attivo.cmd  @eseguo.un.bat; @comando.bat
use std::process::Command; //libreria per eseguire i comandi (cmd)
//---------------------------------------------------------------------------//


/*&str = variabile stringa del chiamante che deve contenere la path + file da 
  attivare con il comando cmd:
    es. ----> call_exe::call("c:\\CASA\\PROGRAMMI\\RUST_PROGETTI\\APRI_FILE_{guessing_game}.bat");
*/


/// CHIAMA FILE ESTERNO @chiama.file.esterno
pub fn call(path: &str) {//CHIAMA UN PROCESSO FIGLIO
    //! 
    //!chiama il Bat e a sua volta EXE 
    //!
    Command::new(path)
            .output()
            .expect("failed to execute process");
    println!("\n\n
    ESEMPIO DI CHIAMATA FILE ESTERNO .BAT tramite funzione a parametro
    --------------------------------------------------------------------------------
    FATTO call bat! , chiamato il file .exe individuato nel bat + apertura della
    directory contenente il file .bat 
    Il chiamante trasmette un stringa contenente la path + file.bat con la variabile puntatore
    &str  che viene ricevuto in prestito dalla dal parametro della funzione ricevente che attiva
    il comando cmd mediante la seguente funzione: 
        ----> pub fn call(path: &str) ....
                        Command::new(path)  ...
              valore della variabile path =   {}", path);


    println!("\n\n\
    ---------------------------------------------------------------------------
                    FINE Capitolo_01 - call_exe - \n\n");


}//DONE


