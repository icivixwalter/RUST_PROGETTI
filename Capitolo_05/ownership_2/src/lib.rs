
//use std::io::stdin;
struct User {
    username: String,       //corretta con String al posto di &str che dava errore
    email: String,
    sign_in_count: u64,
    #[allow(dead_code)]
    active: bool,
}



//*******************************************************************************************/
//                          ESEMPIO PROPRIETA DELLA STRUCT
// 
//*******************************************************************************************/

// region: main
pub fn run() {

    
        //corretta per evitare l'errore   da username: &str,+  email: &str, a String::from...
        //questa è una struttura??
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        
        
        println!("
        //      Ownership_2 of Struct Data --- ESEMPIO DANGLE --
        ----------------------------------------------------------------------------------------//
            Proprietà dei dati Struct
            strutture memorizzino riferimenti a dati di proprietà di qualcos'altro,
            1° operazione chiamo DANGLE che restituisce una stringa, questo è il codice:
                    fn dangle() -> String {{ 
                        let s = String::from(''hello''); 
                        s  //con &s riferimento
                    
                    }} 

            qui chiamo dangle() ----> restituisce : {} \n ",dangle());
    
            println!("
            //      Ownership_2 of Struct Data --- ESEMPIO STRUTTURA USER --
            ----------------------------------------------------------------------------------------//
            Note: in questo esempio, viene costruita una struttura ed assegnata alla variabile user1,
            è possibile richiamare i componenti della struttura qualificando la variabile oggetto.elemento
            come in questo esempio:
             
            La struttura costruita --->         let user1 = User {{
                                                email: String::from('someone@example.com'),
                                                username: String::from('someusername123'),
                                                active: true,
                                                sign_in_count: 1,
                                                }};
            Richiamo i singoli elementi:
            user1.email         :                {}
            user1.username      :                {}
            user.sign_in_count  :                {}",user1.email, user1.username, user1.sign_in_count);
           
    


//              RITARDO DELLA SCHELL
//---------------------------------------------------------------------------//
// utilizzare questa libreria:
//'use::std::io;'
//use std::io::stdin;

// let mut s = String::new();
// println!("\n\n RITARDO DELLA SCHELL: premi invio per uscire!");
// stdin()
// .read_line(&mut s)
// .expect("Did not enter a correct string");

//---------------------------------------------------------------------------//


    println!("\n\n\
    ---------------------------------------------------------------------------
                    FINE Capitolo_05 - ownership_2 - \n\n");


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
      


