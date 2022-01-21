// fn main() {
//     println!("Hello, world!");
// }

pub fn run() {

    println!("
    //  ESERCIZIO 6.2.05_Catch-all Patterns and the _ Placeholder_Modelli Catch-all_e_ Segnaposto
    //=========================================================================================//
    //Nota: funzione che attiva la scelta scelta tramite la funzione match (select) con 
      una variabile u8 che comprende il range da 1 a 255 numeri e quindi combinazioni. Le
      combinazioni devono essere tutte coperte altrimenti rust non esegue la compilazione in
      quanto il PATTERN deve coprire tutte le combinazioni degli u8 infatti le combinazioni 
      speciali sono le prime due (3 e 7) mentre l'ultima è il caso generico che copre le 
      restanti combinazioni fino a 255 (perchè u8):
    ");

    attiva(9);
    attiva(7);
    attiva(3);

    println!("\n
    //  *** FINE ***
    //=========================================================================================//\n");

    
}

//faccio FUNZIONE CHE ATTIVA UNA DELLE SCELTE = i numeri u8 sono da 1 a 255 e quindi
// devono essere tutti coperti altrimenti rust non ti fa compilare perche il pattern
// non copre tutto il modello e questo è il potenziale errore:
//  ^^^^^^^^^ patterns `0_u8..=2_u8`, `4_u8..=6_u8` and `8_u8..=u8::MAX` not covered
fn attiva(dice_roll:u8){
    match dice_roll {
        3 => {
            add_fancy_hat(); //primi due casi valori 3, 7
        println!("\n
        combinazione  attiva(7);
        add_fancy_hat();
        metti un bel cappello {}                = ", dice_roll);
        }
        7 => remove_fancy_hat(),
       other => move_player(other), // modello è la variabile
    }
   
} 

fn add_fancy_hat() {}


fn remove_fancy_hat() {
    println!(" \n
    combinazione  attiva(3) add_fancy_hat();
    rimuovi il cappello                             = 7");
}
fn move_player(_num_spaces: u8) {
    println!("\n
    combinazione  attiva(9);
    muovi il personaggio di {} spazi,               =", _num_spaces);
}