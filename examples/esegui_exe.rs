use std::process::Command; //libreria per eseguire i comandi (cmd)
//---------------------------------------------------------------------------//
// Esegue l'exe main.exe
fn main() {
    // per eseguire usa cargo run --example esegui_exe
    // chiama direttamente l'exe
    Command::new("Capitolo_01/hello_world/main.exe") //crea il Command per eseguire l'exe
            .spawn() // esegue il comando spawnando un processo figlio
            .expect("failed to execute process"); // se fallisce stampa questo
   
}//DONE
