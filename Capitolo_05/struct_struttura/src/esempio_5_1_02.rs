
use crate::esempio_struttura::User;


//la funzione è pubblica con pub (altrimenti è di base private), inoltre per utilizzarla
// occorre applicare questo codice : use crate::esempio_4::esempio_5_1_2_pfunct;
pub fn esempio_5_1_2_pfunct() {
    println!("
    // II° ESEMPIO - UTILIZZO DELLA STRUTTURA 
    //_________________________________________________________________________________________________//
    //NOTE :
    Creazione di una nuova istanza di una struttura che utilizza la maggior parte dei valori
    di una vecchia istanza ma ne modifica alcuni. La sintassi di aggiornamento della struttura.
    Innanzitutto, il Listato 5-6 mostra come creiamo una nuova istanza User in user2 senza la sintassi di aggiornamento. Impostiamo un nuovo valore per l'e-mail, ma per il resto usa gli stessi valori di user1 che abbiamo creato nel Listato 5-2. Riferimento ad una parte di una stringa: 
        Allo stesso modo, se la tua slice (pezzo di stringa ) include l'ultimo byte della stringa, 
        puoi eliminare il numero finale. Ciò significa che sono uguali: (l’esempio si base sul comando [3….len] o [3..] )
    "
    );
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),//HO MODIFICATO L'EMAIL
        ..user1
    };


    println!("\n stampo l'istanza user2: {} ", user2);
}
