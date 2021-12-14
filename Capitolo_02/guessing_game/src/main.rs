use std::io;
//aggiornare il file .toml   con le nuove dipendenze prima di utilizzare rand
use rand::Rng;
use std::cmp::Ordering; // libreria per gli

fn main() {
    //scrivi il tuo numero
    println!("prego scrivi il tuo numero: Please input your guess.");

    /* GENERA UN NUMERO CASUALE
        viene creato il numero casuale con il rand:: .... nel range tra 1 e 100
        utilizzando la libreria da includere :   use rand::Rng;
    */
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        /*INSERISCI IL TUO NUMERO SEGRETO
            funzione chre restituisce una stringa vuota (non nulla perche in rust non esiste null)
            che viene assegnata alla variabile mut guess
        */
        //println!("Prego Inserisci il tuo numero segreto");
        let mut guess = String::new(); //qui viene ASSEGNATO

        /* poteva essere scritta su una linea
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            ma è meno comprensibile e si puo dividere in linea con alla fine
            il ; lo standard io ha bisogno della libreria:
            use std::io;
        */
        io::stdin()
            //lo stdin ha la funzione leggi la linea e recupero per riferimento la variabile guess
            .read_line(&mut guess)
            //controllo errori gestito obbligatorio
            .expect("Failed to read line"); //; è la fine comando

        /* shadowing = oscuramento della variabile guess: riutilizzo del nome di una
            variabile oscurando il valore precedente e sostituendo uno nuovo
            parse: CONVERTENDO da un tipo stringa a U32 (intero unsigned a 32 bit)
            trim:  ELIMINA SPAZI all'inizio E alla fine dell'input da tastiera ed elimina anche l'invio new line (\n)
            CONVERSIONE STRINGA A NUMERO IN INPUT + CONTROLLO ERRORE
        */
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess = match guess.trim().parse::<u32>(){
            Ok(num) => num,
            Err(_) => {
                println!("Scrivi un numero");
                continue; // Riparte da loop!!!!
            }
        };

        /* PER IL CONFRONTO CON IL NUMERO CASUALE
            a. use std::cmp::Ordering; = questa libreria è necessaria per l'ordinamento
        */
        // costrutto match simile allo switch di Java, ma è molto più potente (permette di fare pattern-matching).
        // se usi "=> funzione()", se si verifica quella condizione si esegue una funzione con 1 istruzione
        // se vuoi funzioni con più istruzioni racchiudile tra graffe "=> {istruzione1(); ... istruzioneN();} "
        // l'operatore => è simile alle funzioni lambda di Java. Rappresenta una funzione anonima.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small - Troppo piccolo!"),
            Ordering::Greater => println!("Too big - Troppo grande!"),
            Ordering::Equal => {
                println!("You win - Hai vinto!");
                break; // ESCE DAL loop!!!
            }
        }
    }
}
