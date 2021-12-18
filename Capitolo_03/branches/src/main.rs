use::std::io;
use std::io::stdin;
 /*CAPITOLO 3 - 3.5.0 Control Flow - Controllo del flusso
    //____________________________________________________________________________//
    3.5.01 if Expressions
    Un'espressione if consente di ramificare il codice in base alle condizioni. 
    Fornisci una condizione e poi dichiari: "Se questa condizione è soddisfatta, 
    esegui questo blocco di codice. Se la condizione non è soddisfatta,
    non eseguire questo blocco di codice".
    
 */


fn main() {

    let number = 3; 
    //IF - ESPRESSIONE 
	if number < 5 { 
			println!("CONDIZONE VERA - condition was true"); 
		} 
		else { 
			println!("condition was false"); 
		} 

        //CONDIZIONE BOOL
        let number2 = 3;

        //RUST si attende un bool QUINDI NON PUO ESSER LASCIATO VUOTO
        if number2!=0 {     //condizione vero o falsa NO INTERO perche non viene convertito
            println!("CONDZIONE BOOL SODDISFATTAT number was three \n\n");
        }
        

        //3.5.03 Using if in a let Statement - Utilizzo di if in un'istruzione let
        //----------------------------------------------------------------------------//
        {
            let condition = true; 
            let number = if condition { 5 } else {6}; //I DUE BRACCI DEVONO AVERE LO STESSO TIPI i32
            println!(" 3.5.03 Using if in a let Statement - Utilizzo di if in un'istruzione let 

            Poiché if è un'espressione, possiamo usarla sul lato destro di un'istruzione let, come nel Listato 3-2.
            
            The value of number is: {}", number); 


        }
        //fine
        //----------------------------------------------------------------------------//


// PERMETTE DI RIMANERE VISIBILE LA SHELL 
//---------------------------------------------------------------------------//
// utilizzare questa libreria: 
//use::std::io;
//use std::io::stdin;

let mut s= String::new();
println!("premi invio per uscire!");
stdin().read_line(&mut s).expect("Did not enter a correct string");  

//---------------------------------------------------------------------------//

}
