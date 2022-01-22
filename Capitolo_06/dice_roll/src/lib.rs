pub fn run() {
println!(
"\n
//CASO IN CUI VENGONO IGNORATI TUTTI GLI ALTRI VALORI
//-----------------------------------------------------------------------------//
Nota: Con questo esempio viene conformato il principio di esaustivita 
con un ciclo while vengo esaminati i 3 valori con l'accortezza che se la 
variabile non assume il valore 3 o 7 viene eseguito l'ultimo caso \n"
);

    //

    let mut number = 3;
    //finche è diverso da zero continua la stampa
    while number != 0 {
        if number % 4 == 3 {
            let _dice_roll = 3;
            esegui(_dice_roll)
        }

        if number % 4 == 2 {
            let _dice_roll = 7;
            esegui(_dice_roll)
        }

        if number % 4 == 1 {
            let _dice_roll = 9;
            esegui(_dice_roll)
        }

        println!("CONTEGGIO ALL'INDIETRO {}!\n", number);
        number -= 1; //=-1 per conteggio all'indietro
    }
println!(
"\n
//      FINE
//--------------------------------------------------------------------------------------//
    \n\n"
);

    //modifica con un ciclo while per eseguire le 3 condizioni utilizzato la funzione esegui

    fn esegui(_dice_roll: u8) {
        match _dice_roll {
            3 => add_fancy_hat(_dice_roll),
            7 => remove_fancy_hat(_dice_roll),
            _ => reroll(_dice_roll),
        }
    }

    fn add_fancy_hat(_dice_roll: u8) {
        println!("\n add_fancy_hat è uguale a : {}", _dice_roll);
    }
    fn remove_fancy_hat(_dice_roll: u8) {
        println!("\n remove_fancy_hat è uguale a : {}", _dice_roll);
    }
    fn reroll(_dice_roll: u8) {
        println!(
            "\n con il valore {} vengono ignorati gli altri valori 3 e 7 ",
            _dice_roll
        );
    }
}
