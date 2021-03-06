
    pub fn run() {
         //funziona solo con gli altri
        println!("
        ESERCIZIO CAPITOLO 6 bind_values
        Listing 6-4: A Coin enum in which the Quarter variant also holds a UsState value
        ESEMPIO DI ASSOCIAZIONE DI DUE STRUTTURE
        richiama una struttura associata ad un'altra struttura:

        II° STRUTTURA
        enum UsState {{ 
            Alabama, 
            Alaska, // --snip taglia-- 
        }} 

        I STRUTTURA ASSOCIATA ALLA II°
        enum Coin {{
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),   // per richiamarla occorre --> l'itruzione value_in_cents(Coin::Quarter(UsState::Alaska))
        }}

        risultato l'ultima con Quarter chiamo lo stato associrato e restituisce il valore:

        se inserisco Nichel (Coin::Nickel)
            esce questo risultato                   : {}

        se inserisco Penny (Coin::Penny)
        esce questo risultato                       : {}

        se inserisco Dime (Coin::Dime)
        esce questo risultato                       : {}

        se inserisco QUARTER Coin::Quarter(UsState::Alaska)
                esce questo risultato               : {}

        se inserisco QUARTER Coin::Quarter(UsState::Alabama)
            esce questo risultato                   : {}


        "
        , value_in_cents(Coin::Nickel)
        , value_in_cents(Coin::Penny)
        , value_in_cents(Coin::Dime)

         , value_in_cents(Coin::Quarter(UsState::Alaska))
         , value_in_cents(Coin::Quarter(UsState::Alabama))
            );



        println!("\n\n\
    ---------------------------------------------------------------------------
                    FINE Capitolo_06 - bind_values - \n\n");

    }



//derive deve stare qui altrimenti errore
#[derive(Debug)] // so we can inspect the state in a minute 
			    //così possiamo ispezionare lo stato in un minuto


//struttura DATI associata: MONETE + STATI

//struttura STATI
enum UsState { 
	Alabama, 
	Alaska, // --snip taglia-- 
} 

//struttura monete
enum Coin { 
	Penny, 
	Nickel, 
	Dime, 
	Quarter(UsState),       //associazione all'altra struttura
    } 

    
fn value_in_cents(coin: Coin) -> u8 { 
    match coin 
    { Coin::Penny => 1, 
    Coin::Nickel => 5, 
    Coin::Dime => 10, 
    Coin::Quarter(state) => { 
        println!("State quarter from {:?}!", state); 25 } 
    } 
} 
    