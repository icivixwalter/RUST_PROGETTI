//




//use std::io::stdin;
mod esempio_4;
use crate::esempio_4::stampa_4_pfunct;
mod esempio_5;
use crate::esempio_5::stampa_5_pfunct;

mod esempio_6;
use crate::esempio_6::stampa_6_pfunct;

mod esempio_7_slices;
use crate::esempio_7_slices::stampa_7_slices_pfunct;




/*CAPITOLO 4.3.00_The Slice Type_Il_tipo_DiSezione
//____________________________________________________________________________//
*NOTE: Un altro tipo di dati che non ha la proprietà è la sezione.
*Le sezioni ti consentono di fare riferimento alla sequenza contigua di elementi
*in una raccolta piuttosto che nell'intera raccolta.
*
*/
pub fn run() {
    // 0° ESEMPIO BASE DI GIACOMO PER IL CONTROLLO DIRETTO
    println!(
        "
    CAPITOLO 4.3.00_The Slice Type_Il_tipo_DiSezione
    ____________________________________________________________________________\n
    *NOTE: ESEMPIO BASE DI GIACOMO
            controllo diretto della stringa con il comando e restituzione dell'indice 
            let i = first_word(& 'PosizioneDelloSpazio ciao'.to_string())

    \n"
    );

    let i = first_word(&"PosizioneDelloSpazio ciao".to_string());
    println!("lunghezza della prima parola indice i: {}", i);

    println!("____________________________________________________________________________\n");

    // I° ESEMPIO ESERCIZIO - RICERCA LO SPAZIO CON LA FUNZIONE ?
    //_________________________________________________________________________________________________//
    {
        println!(
            "\n
        I° ESEMPIO RICERCA LO SPAZIO CON LA FUNZIONE ?
        _______________________________________________________________________________________
        NOTA:     *NOTE: la sezione di una stringa che ti consente di fare riferimento alla
        sequenza contigua di elementi. Utilizzando la funzione fn first_word(s: &String) -> ?
        si ha come parametro un ?  ma va bene perchè non si è alla ricerca di una proprieta.
        La funzione non ha modo di definire una parte della parola ma si limita a restituire 
        un indice della stringa con il passaggio del parametro a RIFERIMENTO &String

        \n"
        );

        println!(
            "
            01) passo la stringa mutevole lo assegno ad s:
                    codice----> let mut s = String::from('hello world')
            
            "
        );
        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5 - restiusce il valore 5

        println!(
            "
            02) ottengo il valore dalla stringa passata con riferimento 
                    codice----> let word = first_word(&s); 
                    valore di word: {}
            
            ",
            word
        );

        s.clear(); // this empties the String, making it equal to ""
                   // word still has the value 5 here, but there's no more string that
                   // we could meaningfully use the value 5 with. word is now totally invalid!
                   // questo svuota la stringa, rendendola uguale a ""
                   // la parola ha ancora il valore 5 qui, ma non ci sono più stringhe che
                   // potremmo usare significativamente il valore 5 con. la parola ora è totalmente invalida!

        println!(
            "
            03) svuoto la stringa s rendendola a vuota: ' '
                        codice----> s.clear();       
            "
        );
    }
    // I° ESEMPIO ESERCIZIO - RICERCA LO SPAZIO CON LA FUNZIONE ? *** fine ***
    //_________________________________________________________________________________________________//

    {
        // II° ESEMPIO ESERCIZIO - RICERCA LO SPAZIO CON LA FUNZIONE ? *** fine ***
        //_________________________________________________________________________________________________//
        //NOTE: monitoraggio della parola con un indici inziale e finale.
        //let mut s_second = String::from("hello world");
        let s_second = String::from("hello world");
        let word = first_word(&s_second); // word will get the value 5

        println!("
            // II° ESEMPIO ESERCIZIO - RICERCA LO SPAZIO CON LA FUNZIONE ? *** fine ***
            //_________________________________________________________________________________________________//
            //NOTE: monitoraggio della parola con un indici inziale e finale.

            01) passo la stringa mutevole lo assegno ad s_second  :           {}
                    codice---->  let mut s_second = String::from('hello world');

            02) passo per riferimento la variabile s_second alla funzione ed ottengo come
                risultato restituito nella variabile word è  :           {}
                codice---->  let word = first_word(&s_second);

            
            ",s_second, word);

        println!("
             Conclusione, il programma funziona ma il difetto è che il valore ricercato non è
             collegato alla stringa per cui la posizione restituita potrebbe nel futuro non essere
             valida se viene utilizzata un'altra stringa. Per cui è necessario trovare il modo di 
             collegare l'indice restituito con la parola passata come parametro. Quindi è necessario
             costruire una funzione con la restituzione di 2 valori tipo questa:
                        fn second_word(s: &String) -> (usize, usize) ... 
            Con questo tipo di funzione puoi ricavare dai dati l'indice iniziale e finale ma che comunque 
            devono essere collegate allo stato della stringa, per cui 3 variabili devono essere sincronizzate
            tra di loro. Rust trova una soluzione chiamata parti della stringa....
            // *** fine ***
            //_________________________________________________________________________________________________//");

        // II° ESEMPIO ESERCIZIO - RICERCA LO SPAZIO CON LA FUNZIONE ? *** fine ***
        //_________________________________________________________________________________________________//
    }

    // III° ESEMPIO - RICERCA LO SPAZIO CON IL CONTROLLO DELL'INDICI INIZIALE E FINALE
    //_________________________________________________________________________________________________//
    //NOTE: Sezioni stringa monitoraggio della parola con un indici iniziale e finale.
    {
        println!("
        // III° ESEMPIO - RICERCA LO SPAZIO CON IL CONTROLLO DELL'INDICI INIZIALE E FINALE
        //_________________________________________________________________________________________________//
            Riferimento ad una parte di una stringa: "
        );
        let s_tre = String::from("hello world");
        let hello = &s_tre[0..5]; //riferimento all'inizio da  1 a 5 mentre quella successiva da 6 a 11
        let world = &s_tre[6..11];

        println!(
            "
            1) variabile contiene la stringa completa           - s_tre:    {}
            2) ''        contiene la posizione iniziale         - hello:    {}
                codice---> &s_tre[0..5]
            3) ''        contiene la posizione finale           - world:    {}
                codice---> &s_tre[6..11]
 
           ",
            s_tre, hello, world
        );

        //indice 0 senza l'indice iniziale - il primo numero indice iniziale e l'ultimo indice finale -1 (escluso)
        let s_stringa = String::from("hello");
        let slice_inizio_fine = &s_stringa[0..2]; //indice iniziale e finale
        let slice_solo_inizio = &s_stringa[..2]; //inidice solo finale
        println!("\n\n
           ------------------------------------------------------------------------------------------------------
           Con la sintassi dell'intervallo di Rust, se si desidera iniziare dall'indice zero,
            è possibile eliminare il valore prima dei due punti. In altre parole, questi sono uguali,
            questo esempio prende 2 byte perche viene indicato nel comando [0..2] oppure :.. [..2]
            0) variabile con la stringa di base                 -  let s_stringa                                        :{}
            1) variabile contiene la stringa completa           -  let slice_inizio_fine    = &s_stringa[0..2];         :{} 
            2) variabile contiene la stringa  iniziale e finale - let slice_solo_inizio     = &s_stringa[..2];          :{} 
            \n", s_stringa, slice_inizio_fine , slice_solo_inizio);
    }
    // III° ESEMPIO - RICERCA LO SPAZIO CON IL CONTROLLO DELL'INDICI INIZIALE E FINALE *** fine ***
    //_________________________________________________________________________________________________//

    // VI° ESEMPIO - RICERCA LA STRINGA CON L'ULTIMO BYTE INCLUSO
    //_________________________________________________________________________________________________//
    //note : richiamo la funzione del file stampa_4
            stampa_4_pfunct();
    
    // VI° ESEMPIO - RICERCA LA STRINGA CON L'ULTIMO BYTE INCLUSO  *** fine ***
    //_________________________________________________________________________________________________//

    // V ESEMPIO - ELIMINA GLI INDICI minore e maggiore
    //_________________________________________________________________________________________________//
    /*note : È inoltre possibile eliminare entrambi i valori per acquisire una 
            sezione dell'intera stringa. Quindi questi sono uguali.
            Nota: gli indici dell'intervallo di sezioni stringa devono essere presenti 
            a limiti di caratteri UTF-8 validi. Se si tenta di creare una fetta di stringa 
            al centro di un carattere multibyte, il programma verrà chiuso con un errore. 
            Ai fini dell'introduzione di sezioni di stringa, 
            assumiamo ASCII solo in questa sezione; una discussione più approfondita 
            sulla gestione di UTF-8 si trova nella sezione "Memorizzazione di testo 
            codificato UTF-8 con stringhe" del Capitolo 8.
    */
                //
                stampa_5_pfunct();

     //_________________________________________________________________________________________________//
    

    // VI ESEMPIO - API SOLIDA
    //_________________________________________________________________________________________________//
    /*note : ricerca dell'indice iniziale e finale con il compilatore che restituisce indice sempre validi
    */
                //
                stampa_6_pfunct();

     //_________________________________________________________________________________________________//
    
    // VI ESEMPIO - API GENERALE passaggio di una  &String che sui valori di riferimento &str
    //_________________________________________________________________________________________________//
    /*note : passaggio di String o di un riferimento ad una stringa.
    */
    

            stampa_7_slices_pfunct();
    //_________________________________________________________________________________________________//
    


    // //              RITARDO DELLA SCHELL
    // //---------------------------------------------------------------------------//
    // // utilizzare questa libreria:
    // //'use::std::io;'
    // //use std::io::stdin;

    // let mut x = String::new();
    // println!("\n\n RITARDO DELLA SHELL: premi invio per uscire!");
    // stdin()
    //     .read_line(&mut x)
    //     .expect("Did not enter a correct string");

    // //---------------------------------------------------------------------------//



    println!("\n\n\
    ---------------------------------------------------------------------------
                    FINE Capitolo_04 - the_slice_type - \n\n");


} /*FINE run  */

//*******************************************************************************************/
//                          funzioni esterne  INIZIO
// fuori dal run la funzione = FUNZIONI DI UTILITA
//*******************************************************************************************/
//LA FUNZIONE FIRST WORLD
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //CONVERSIONE DELLA STRINGA in byte
                              //println!("start");
                              //FOR = iteratore SULLA MATRICE DI BYTE utilizzando
                              // il metodo bytes.iter , che restituisce ogni elemento della raccolta.
                              // tupla .enumerate  il primo elemento è i = elemento indice;
                              //			    il secondo elemento è &item = riferimento
                              //ITERATORE SULLA MATRICE DI BYTES
    for (i, &item) in bytes.iter().enumerate() {
        //convertiremo il nostro parametro in una matrice di byte utilizzando il metodo:StringStringas_bytes
        //enumerate restituisce una TUPLA con
        //il primo elemento è il byte della raccolta OSSIA L'INDICE
        //il secondo ELEMENTO è il byte ispezionato. ATTENZIONE questo modello restituisce
        //una tupla che puo essere destrutturata con l'indice della tupla  e l'elemento della tupla.
        //questo è il modello per la destrutturazioine della tupla:
        //              modello=   .iter().enumerate(), we use & in the pattern.

        // if then
        //println!("next {}", i);
        if item == b' ' {
            //item = elemento della tupla e controlla se è uno spazio
            //then			//restituisce la posizione dello spazio
            //println!("end word");
            return i;
        }
    }
    //println!("end");
    s.len() //altrimenti restituisce l’intera 									lunghezza della stringa
}

//                     fine
//---------------------------------------------------------------------------------//
