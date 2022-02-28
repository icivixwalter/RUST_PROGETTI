pub fn run() {

 

let ni:u8= value_in_cents(Coin::Nickel);


 println!("
 \n\n
 6.2.00_The match Control Flow Operator_L'operatore del flusso di controllo della partita

 I° ESEMPIO = 4 bracci con una sola espressione per caso
 //---------------------------------------------------------------------------------//
 CHIAMO value_in_cents(Coin::Penny);
 restituisce:      {} 
 CHIAMO value_in_cents(Coin::Quarter)
 restituisce:      {} 
 CHIAMO value_in_cents(Coin::Nickel)
 restituisce:      {} 


  ", value_in_cents(Coin::Penny),  value_in_cents(Coin::Quarter),ni   
);

println!("\n
 II° ESEMPIO = 4 bracci con  1 caso multi espressione diviso per graffe
 //---------------------------------------------------------------------------------//
 chiamo value_in_cents_2
 restituisce:      {} 

", value_in_cents_2(Coin::Penny));


//FUNZIONI ESTERNE
//===================================================================================================//


    println!("\n\n\
    ---------------------------------------------------------------------------
                    FINE Capitolo_06 - control_flow - \n\n");



}

enum Coin {
    Penny,
    Nickel,
    #[allow(dead_code)]
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}



//FUNZIONE MULTILINEA
fn value_in_cents_2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { //PIU LIBEE sullo stesso braccio utilizzare le graffe ma viene
                         //restituito solo un valore
            println!("
            braccio Penny con stampa - ma restituisco sempre un solo valore:
            Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
