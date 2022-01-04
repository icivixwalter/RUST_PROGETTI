
use std::io::stdin;
struct User {
    username: String,       //corretta con String al posto di &str che dava errore
    email: String,
    sign_in_count: u64,
    active: bool,
}



//*******************************************************************************************/
//                          ESEMPIO PROPRIETA DELLA STRUCT
// 
//*******************************************************************************************/

// region: main
fn main() {
    println!("
    //      Ownership of Struct Data
    ----------------------------------------------------------------------------------------//
        Proprietà dei dati Struct
        strutture memorizzino riferimenti a dati di proprietà di qualcos'altro,


    ");
        //corretta per evitare l'errore   da username: &str,+  email: &str, a String::from...
  
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
  
    
        
    


//              RITARDO DELLA SCHELL
//---------------------------------------------------------------------------//
// utilizzare questa libreria:
//'use::std::io;'
//use std::io::stdin;

let mut s = String::new();
println!("\n\n RITARDO DELLA SCHELL: premi invio per uscire!");
stdin()
.read_line(&mut s)
.expect("Did not enter a correct string");

//---------------------------------------------------------------------------//

}
// endregion: main


// region: funzioni esterne
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

// endregion: funzioni esterne    
      


