//use std::io::stdin;

/*CAPITOLO 4 - 4.2.00_References_and_Borrowing_Referenze_e_Prestito
//____________________________________________________________________________//
*NOTE: Oggetto con reference
*
*/
pub fn run() {
    // run
    println!(
        "
    /*CAPITOLO 4 - 4.2.00_References_and_Borrowing_Referenze_e_Prestito
    //____________________________________________________________________________//
    *NOTE: Oggetto con reference
    \n   
        "
    );
    {
        //al modifica
        let reference_s1 = String::from("hello in s1");
        let reference_len = calculate_length(&reference_s1);
        println!(
            "Il riferimento: The length of - len di s1: '{}' lunghezza di s1 è - is: {}.",
            reference_s1, reference_len
        );
    }

    {
        //assegno ad una variabile mut la stringa
        let mut new_s1 = String::from("new_s1: hello");

        println!(
            "
            assegno alla variabile mut un valore stringa ma lo inserisco nell'heap con from
                    -----> let mut new_s1 = String::from (hello in new_s1); 
            "
        );

        //aggiungo il tentativo di modificare il riferimento da hello a word ma da errore.
        //chiamata della funzine modificata con &mut per rendere il RIFERIMENTO MUTEVOLE.
        change(&mut new_s1);

        println!(
            "RIFERIMENTO ALLA STRINGA MUTATO
           nuovo valore DELL VARIABILE  new_s1 cambiato con la funzine change
           che ha integrato la stringa iniziale hello in: -----> {}
                    ",
            new_s1
        );
    }

    {
        /* //INIZIO E FINE DEL RIFERIMENTO
        IL riferimento ha inizio dove viene introdotto e finisce dove viene utilizato l'ultima
        volta. es:
        */

        println!(
            "
            /*CAPITOLO 4 - 4.2.00_References_and_Borrowing_Referenze_e_Prestito
            //____________________________________________________________________________//
            *NOTE: II° ESEMPIO dove ha inizio e fine il riferimento e come puo essere utiizzato
                unitamente a mut senza conflitto. 
            \n    "
        );

        //utilizzo le variabili con mutamento
        let mut s = String::from("hello"); //codice funzionante.
        let r1 = &s; // no problem
        let r2 = &s; // no problem

        println!(
            "
    
            Le variabili r1 e r2 possono essere utilizate con mut s in quanto vengono
            utilizzate prima di un nuovo caso di mut:
            assegno a s il valore stringa con mut e stampo tutto:
                mut s = {}
                r1 = %s {}
                r2 = %s {}
            \n    ",
            s, r1, r2
        );

        //nuovo cas di mut dopo il vecchio utilizzo
        let r3 = &mut s; // no problem
        println!(
            "
            NUOVO CASO DI MUT DOPO L'USCITA DI AMBITO DI R1+R2:
            dopo l'uscita d'ambito di r1 e r2 perchè utilizate per ultime 
            posso assgnare  a r3 la variabile s mutata e stampo:
                    -----> let r3 = &mut s;
            
            variabile r3 mutata : {}",
            r3
        ); //qui viene inserito il riferimento mutabile
    }

    //              RITARDO DELLA SCHELL
    //---------------------------------------------------------------------------//
    // utilizzare questa libreria:
    //'use::std::io;'
    //'use std::io::stdin;'

    let mut s = String::from("provo la stringa mutabile: ");
    walter(&mut s);  // se metto riferimento &mut = presto il riferimento mutabile
    println!("stampo la stringa restituita dalla funzione walter() {}",s);

    // println!("\n\n RITARDO DELLA SCHELL: premi invio per uscire!");
    // stdin()
    //     .read_line(&mut s)
    //     .expect("Did not enter a correct string");

    //---------------------------------------------------------------------------//



    println!("\n\n\
    ---------------------------------------------------------------------------
                    FINE Capitolo_04 - references_and_borrowing - \n\n");


} // *** fine *** run

//*******************************************************************************************/
//                          funzioni esterne  INIZIO
// fuori dal run la funzione
//*******************************************************************************************/
// III° GRUPPO DI FUNZIONI ESERCIZIO 4.2.00_References_and_Borrowing_Referenze_e_Prestito
//-------------------------------------------------------------------------------------------//

fn calculate_length(s: &String) -> usize {
    s.len() // s is a reference to a String
}
// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens- Qui, s esce dal campo di applicazione.
//Ma poiché non ha la proprietà di ciò a cui
// si riferisce, non succede nulla..

//funzione che tenta di modificare un riferimento  ma da errore - viend modificata
// in modo da accettare un RIFERIMENTO MUTEVOLE con  fn change(some_string: &mut String)
fn change(some_string: &mut String) {
    //firma della funzione.
    some_string.push_str(", : world");
}

//*******************************************************************************************/
//                          funzioni esterne  Inizio
// fuori dal run la funzione
//*******************************************************************************************/
//-------------------------------------------------------------------------------------------//

fn walter(stringa: &mut String) {
    stringa.push_str("walter");  //push aggiunge alla fine
}

//*******************************************************************************************/
//                          funzioni esterne  FINE
// fuori dal run la funzione
//*******************************************************************************************/
