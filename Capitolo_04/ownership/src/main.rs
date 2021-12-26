//use::std::io;
use std::io::stdin;

/*CAPITOLO 4 - 4.1.00 What Is Ownership -Che cos'è la proprietà
//____________________________________________________________________________//
*NOTE: La caratteristica principale di Rust è la proprietà ( ownership ).
*c) Rust utilizza un terzo approccio: la memoria è gestita attraverso un sistema di
*proprietà con un insieme di regole che il compilatore controlla in fase di compilazione.
*Nessuna delle funzionalità di proprietà rallenta il tuo programma mentre è in esecuzione.
*
*/
fn main() { //main
    /*4.1.07_Ways Variables and Data Interact: Clone_Modi di interazione delle variabili e dei dati: clone
     * //____________________________________________________________________________//
     */

    println!(
        "\n\n
    /*4.1.07_Ways Variables and Data Interact: Clone_Modi di interazione delle 
        variabili e dei dati: clone
    //____________________________________________________________________________//
    \n
  
    "
    );
    let s1 = String::from(" Utilizzo di ::from per: hello "); //copia s1 in s2
    let s2 = s1.clone(); // metodo costoso con clone
                         //stampa due volte il messaggio
    println!(
        " Metodo clone CLONAZIONE PROFONDA, metodo costosto copia  
               il valore dell'heap e quelli dello stack es:
               s1 = s2 : 
              valore di s1 = {} 
              valore di s2 = {} ",
        s1, s2
    );

    //____________________________________________________________________________//

    {
        let s = String::from("hello"); // s comes into scope 	- entra nel campo

        takes_ownership(s); // s's value moves into the function...
                            // ... and	so is no longer valid  - il valore di  s si sposta
                            //nella funzione è quindi non è piu valido.

        let x = 5; // x comes into scope - entra nel campo di validita
        makes_copy(x); // x would move into the function, x si sposta nella funzione makes_copy
        // but i32 is Copy, so it's okay to still (ma i32 è Copy, quindi va bene ancora)
        // use x afterward

        // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens
        // Qui, x esce dall'ambito, quindi s. Ma poiché il valore di s è stato spostato, niente di speciale succede.
    
    
    
    }


    {
     // II° GRUPPO DI FUNZIONI  DELL'ESERCIZIO 4.1.01.07_Return_Values_and Scope
     //---------------------------------------------------------------------------//
  
     
     println!("\n
     II° GRUPPO DI FUNZIONI  DELL'ESERCIZIO 4.1.01.07_Return_Values_and Scope
     //---------------------------------------------------------------------------//
     NOte: con questo II° esercizio si vuole dimostrare come 
     La restituzione di valori può anche trasferire la proprietà.   \n ");                                       
        //codice--->a)_gives_s1 = chiamo la funzione gives_ownership
        let gives_s1 = gives_ownership();       // gives_ownership moves its return 
                                                // value into s1 let gives_ownership sposta il valore 

         println!("
                    a) in gives_ownership  inserisco una stringa in s1- 
                    comando---->  let gives_s1 = gives_ownership(); 
                    \n");                                       
        
         //codice--->gives_s2.b) = assegno a gives_s2 una stringa nell'heap
        let gives_s2 = String::from("hello");         // s2 comes into scope – s2 entra nell’ambito di validita
        
        println!("
                b) gives_s2 = assegno una stringa as s2;
                comando---->  let gives_s1 = gives_ownership(); 
                \n            
                ");

        //codice--->gives_s3.c) = assegno a gives_s3 una stringa da gives_s2
        let gives_s3 = takes_and_gives_back(gives_s2); // s2 is moved into – s2 entra into
                                  // takes_and_gives_back, which also // moves its return value into s3 – 
                                  //takes_and_gives_back gli prende il valore di ritorno da s2 e lo sposta in s3

        println!("
                 b) gives_s3 = assegno una stringa a s3  recuperandola dalla funzione;
                        comando----> let gives_s3 = takes_and_gives_back(gives_s2);\n");
                  
            println!("\n
                **** fine ****
                II° GRUPPO DI FUNZIONI  DELL'ESERCIZIO 4.1.01.07_Return_Values_and Scope
            //---------------------------------------------------------------------------//\n");

     /*FINE */   
     // II° GRUPPO DI FUNZIONI  DELL'ESERCIZIO 4.1.01.07_Return_Values_and Scope
     //---------------------------------------------------------------------------//


    }

    {
        // II°.B LA TUPLA (GRUPPO DI FUNZIONI  DELL'ESERCIZIO 4.1.01.07_Return_Values_and Scope)
        // Esempio di tupla
        //---------------------------------------------------------------------------//
        //codice------>Tupla.01_AssegnoLeVariabili

        println!("\n
        II°.B LA TUPLA (GRUPPO DI FUNZIONI  DELL'ESERCIZIO 4.1.01.07_Return_Values_and Scope)
        //---------------------------------------------------------------------------//
        Note: esempio di costruzione di una TUPLA per la restituzione di piu valori:           \n ");                                       
   

        let tupla_s1 = String::from("TUPLA: hello"); // assegno la stringa a s1
        let (tupla_s2, len) = calculate_length(tupla_s1); // assegno a s2 la lunghezza di s1 quindi i32
	    println!("
        Lunghezza della TUPLA: The length of : s2= '{}' len= is {}.
        sono stati restituiti 2 tipi, una stringa ed un i32 con la tupla.
        //---------------------------------------------------------------------------//
        ", tupla_s2, len); //stampo la lunghezza
//	

        
        /*FINE */   
        // II°.B LA TUPLA (GRUPPO DI FUNZIONI  DELL'ESERCIZIO 4.1.01.07_Return_Values_and Scope)
        //---------------------------------------------------------------------------//
        
    }    
  
    
    
    //              RITARDO DELLA SCHELL
    //---------------------------------------------------------------------------//
    // utilizzare questa libreria:
    //'use::std::io;'
    //'use std::io::stdin;'

    let mut s = String::new();
    println!("\n\n RITARDO DELLA SCHELL: premi invio per uscire!");
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    //---------------------------------------------------------------------------//




}//main *** fine

//funzioni esterne  INIZIO
//---------------------------------------------------------------------------//

        //((**-------------------------------------------**))

//*******************************************************************************************/
//                          funzioni esterne  INIZIO
// fuori dal main la funzione
//*******************************************************************************************/


 
    // II° GRUPPO FUNZIONI FUNZIONI DELL'ESERCIZIO 4.1.01.07_Return_Values_and Scope
    //-------------------------------------------------------------------------------------------//

            //01) FUNZIONE takes_and_gives_back
             // This function takes a String and returns one 
            //Questa funzione accetta una stringa e ne restituisce una 
            
            
            fn takes_and_gives_back(a_string: String) -> String { // a_string comes into a_string
                                                                 // ambito di validia
                                                                  // scopo 	
                a_string    // a_string is returned and moves out to the calling function 
                            //a_string viene restituito e si sposta alla funzione chiamante 
            } 


            //codice--->a)_gives_s1.Function_gives_ownership = chiamata
            fn gives_ownership() -> String { // gives_ownership will move its return value into the function  that calls it
                                            // gives_ownership sposterà il suo valore nella	funzione che chiama
            let some_string = String::from("yours"); // some_string comes into scope 

            some_string             // some_string is returned and moves out to the calling function
                                 
            //viene restituita some_string e passa alla funzione chiamata 
            }


    //*** FINE  */
    // II° GRUPPO FUNZIONI FUNZIONI DELL'ESERCIZIO 4.1.01.07_Return_Values_and Scope
    //-------------------------------------------------------------------------------------------//

    // II°.B LA TUPLA  LA FUNZIONE PER LA TUPLA
    //-------------------------------------------------------------------------------------------//
    //funzione calcolo lunghezza:
    //codice------>Tupla.01_CalcoloLunghezza
        fn calculate_length(s: String) -> (String, usize) { 
            let length = s.len(); // len() 
            (s, length)           //returns the length of a String – len() restituisce la lunghezza di una stringa 
        } 

    /*Fine */
    // II°.B LA TUPLA  LA FUNZIONE PER LA TUPLA
    //-------------------------------------------------------------------------------------------//
    



    // I° GRUPPO FUNZIONI FUNZIONI DELL'ESERCIZIO 4.1.07
    //LE FUNZIONI DELL'ESERCIZIO 4.1.07_Ways Variables and Data Interact: 
    //Clone_Modi di interazione delle variabili e dei dati: clone
    //---------------------------------------------------------------------------//
    fn takes_ownership(some_string: String) {  //clonazione
        // some_string comes into scope 	- entra nell'ambito

        println!("{}", some_string);
        // Here, some_string goes out of scope and `drop` is called. The backing
        // memory is freed.
        // Qui, some_string esce dallo scope e viene chiamato `drop`. Il supporto
        //e la memoria viene liberata.
    }

    fn makes_copy(some_integer: i32) {  //copia nello stack
        // some_integer comes into scope

        println!("{}", some_integer);
        //Here, some_integer goes out of scope. Nothing special happens.
        //Qui, some_integer esce dall'ambito. Non succede niente di speciale.
    }

//*******************************************************************************************/
//                          funzioni esterne  FINE
//*******************************************************************************************/


//---------------------------------------------------------------------------//

//---------------------------------------------------------------------------//
//---------------------------------------------------------------------------//