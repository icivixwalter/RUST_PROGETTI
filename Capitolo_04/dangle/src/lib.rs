
// use std::io::stdin;

/*CAPITOLO 4 - 4.2.02_Dangling References_Riferimenti_pendenti
//____________________________________________________________________________//
*NOTE: Gestione dei riferimenti a puntatori non validi che rust controlla in 
*anticipo non permettendo di creare questi puntatori errati es:
*- creare erroneamente un puntatore PENDENTE,
*- un puntatore che fa riferimento a una posizione nella memoria che potrebbe essere stata assegnata a qualcun altro,
*- liberando un po' di memoria preservando un puntatore che punta ancora a quella memoria liberata e non piu valida.
*
*/
pub fn run() {

    println!("
    /*CAPITOLO 4 - 4.2.02_Dangling References_Riferimenti_pendenti
    //____________________________________________________________________________//
    *NOTE: Riferimenti pendenti errati ed il controllo di rust.
     
    \n   
        ");
         
            let reference_to_nothing = dangle(); 

            println!("proga con il riferimento
            [let reference_to_nothing = dangle(); ]  il valore è questo: {}",reference_to_nothing);
               

// //              RITARDO DELLA SCHELL
// //---------------------------------------------------------------------------//
// // utilizzare questa libreria:
// //'use::std::io;'
// //use std::io::stdin;

//     let mut s = String::new();
//     println!("\n\n RITARDO DELLA SCHELL: premi invio per uscire!");
//     stdin()
//     .read_line(&mut s)
//     .expect("Did not enter a correct string");

// //---------------------------------------------------------------------------//

            
            
} /*FINE MAIN  */
        

//*******************************************************************************************/
//                          funzioni esterne  INIZIO
// fuori dal main la funzione
//*******************************************************************************************/

fn dangle() -> String { 
    let s = String::from("hello"); 
    s  //con &s riferimento

} 


//                     fine
//---------------------------------------------------------------------------------//   
    
      


