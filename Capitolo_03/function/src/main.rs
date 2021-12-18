
use::std::io;
use std::io::stdin;

/**
 * //CAPITOLO 3 LE FUNZIONI
 * //____________________________________________________________________________//
 * 3.3.02_Function Bodies Contain Statements and Expressions
 * I corpi delle funzioni contengono istruzioni ed espressioni                                      
 */ 
fn main() {

    let y0 = 6;	//ISTRUZIONE perche crei ed assegni una variabile con ; NON RITORNA VALORE
    // DA ERRORE: let x = (let y = 6);    //errore NON SI PUO ASSEGNARE UNA ISTRUZIONE AD UN’ALTRA LA Y NON RITORNA IL VALORE

    /*ESPRESSIONE CORRETTA
     questa è una espressione corretta perche crea PRIMA UNA ISTRUZION con x=3;. Il punto e virgola (;)
     la definisce come ISTRUZIONE.
    CREA UNA ESPRESSIONE con x+1, 
        incrementando la variabile x e mancando il punto e virgola (;) permette IL RETURN DEL VALORE
    */
   
   let y = {        //ESPRESSIONE DI Y CON LE {}
        let x = 3;
        x + 1
    };
    //infine stampo il valore dell'espressione
    println!("\n
    STAMPA L'ESPRESSIONE CREATA CON LET Y ED IL SUO VALORE DI RITORNO 
    espressione creata all'interno di un blocco di parentesi graffe con l'ultima variabile
    x incrementata senza punto e virgola (;) per ottenere il valore di ritorno
    tipdo di espressione: let x = 3; x + 1 

    The value of y is: Y = {} \n\n", y);

    //              LA FUNZIONE CHE RITORNA IL VALORE FIVE I° ESEMPIO
    //===============================================================================================//
    
    //FIRMA DELLA FUNZIONE con il IL TIPO DI VALORE DA RESTITUIRE I32
    fn five() -> i32 { //definizione del parametro della funzione i32
            5 // con il valore da restituire (SENZA IL ;)
            }
            //Espressione con restituzione del valore ed assegnagmento alla variabile
            let x = five();// CHIAMATA DELLA FUNZIONE five per recuperare il valore di ritorno ed assegnarlo a x 
                           // come se avessi assegnato direttamente: let x = 5; Inoltre la funzine five ()
                           // Questa funzione non ha parametri MA E' UNA ESPRESSIONE con restituzione di un valore di ritorno.

         // II° ESEMPIO DI FUNZIONE PLUS_ONE       
        //===============================================================================================//
           {
            //chiamo la funzione passando un numero intero ma chiamando la definizione per il suo incremento
            let x2 = plus_one(5);
            println!("
            LA NUOVA FUNZIONE PLUS_ONCE CHE RITORNA IL VALORE  II° ESEMPIO
            ----------------------------------------------------------------------------
            la funzione chiamata restituisce il valore: - The value of x is: {} \n", x2);
           }

        //firma della funzione una volta chiamata e passando come parametro un intero 5
        

        fn plus_one(x2: i32) -> i32 {
            x2 + 1  // prima di restituirlo viene incrementato ATTENZIONE SENZA ; PER IL RETURN
    
    
    
    
        }
    //===============================================================================================//

    // PERMETTE DI RIMANERE VISIBILE LA SHELL 
//---------------------------------------------------------------------------//
// utilizzare questa libreria: 
//use::std::io;
//use std::io::stdin;

let mut s= String::new();
println!("\n\n premi invio per uscire!");
stdin().read_line(&mut s).expect("Did not enter a correct string");  

//---------------------------------------------------------------------------//


}






//CAPITOLO 3 LE FUNZIONI            *** FINE ***
//____________________________________________________________________________//

