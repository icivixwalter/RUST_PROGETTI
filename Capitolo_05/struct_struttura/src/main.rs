use std::io::stdin;
mod esempio_5_1_02;
mod esempio_struttura;
use crate::{esempio_5_1_02::esempio_5_1_2_pfunct, esempio_struttura::User};

fn main() {
    //modificato con let mut - per usare user occorre la definizione del file::User
    let mut user1 = esempio_struttura::User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // user1.email = String::from("anotheremail@example.com"); //per modificare l'email è stata inserito mut
    //ho chiamato il meto setEmail per cambiare il nome dell'email per riferimento &str
    user1.setEmail("anotheremail@example.com");
    println!(
        "
        //      ESEMPIO I CREATA UNA STRUTTURA  - USER 1
        //-------------------------------------------------------------------------------//
        creata una struttura USER 1
            per il suo utilizzo occore utilizzare la variabile 
            oggetto che punta ad un suo campo:
            - user1.email            : {}
            - user1.username         : {}
            - user1.active           : {}
            - user1.sign_in_count    : {}
            //-------------------------------------------------------------------------------//

            ",
        &user1.email, &user1.username, &user1.active, &user1.sign_in_count
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!(
        "\n
            //      ESEMPIO II MODIFICATA UNA STRUTTURA 
            //-------------------------------------------------------------------------------//
            II ESEMPIO DI USER 2 - creata una struttura 
            per il suo utilizzo occore utilizzare la variabile 
            oggetto che punta ad un suo campo:
            user2            : {}
            .. MODIFICATO L'EMAIL ..OLD.. someone@example.com 
            //-------------------------------------------------------------------------------//
            ",
        &user2
    );

    //5.1.02_Creating Instances From Other Instances With Struct Update Syntax_Creazione
    //di istanze da altre istanze con la sintassi di aggiornamento della struttura
    //--------------------------------------------------------------------------------------------//

    esempio_5_1_2_pfunct();
    //--------------------------------------------------------------------------------------------//

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
} //main *** fine ***


