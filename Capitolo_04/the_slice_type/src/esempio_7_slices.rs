pub fn stampa_7_slices_pfunct() {
    
    // String Slices as Parameters – Slice di Stringhe passate come parametri
    println!("
    // VII° ESEMPIO - MIGLIORAMENTO DELLA FUNZIONE FIRST WORD - valori &String che sui valori &str.
    //_________________________________________________________________________________________________//
    //NOTE : Questo esempio, è un migliornamento della fuznone fistr_word con la flessibilita di 
        poter passare come parametro alternativante:
            - una parte della stringa con ...
            - una riferimento ad una stringa con ... 
    Questo miglioramento sfrutta le Coercizioni di deref implicite del Capitolo 15 ancora da esaminare,
    con la quale è possibile costruire funzioni che possono prendere una parte di stringa al posto di 
    un riferimento ad essa permette di costruire una API GENERALE ed utile senza perdere alcuna funzionalita.
  
    -----------------------------------------------------------------------------------------------------\n");
 
    print!("\n
        I TIPO DI ESEMPIO - parametro con String::from('hello world') - Nella memoria HEAP
    -------------------------------------------------------------------------------------------------");


    // 01)
    //------------------------------------------------------------------------------//
    let my_string = String::from("hello world");        //stringa sull'heap
    let word = first_word(&my_string[0..6]); 
    let msg_s=String::from ("01) MESSAGGIO - I° TIPO DI ESEMPIO STRINGA MEMORIZZATA NELLA MEMORA HEAP :
                                    first_word funziona su porzioni di Stringhe, parziali o intere 

                                    comando: let word = first_word(&my_string[0..6]); 
                                    ");
    
    //CHIAMO LA FUNZIONE COMUNE ID STAMPA con 2 parametri a riferimento ho inserito ad entrambi & = riferimento
    stampa_variabile(&word,&msg_s);
    //------------------------------------------------------------------------------//
  
    // 02)
    //------------------------------------------------------------------------------//
    
        let word = first_word(&my_string[..]);
        let msg_s=String::from ("02) MESSAGGIO - I° TIPO DI ESEMPIO STRINGA MEMORIZZATA NELLA MEMORA HEAP :
                                        first_word funziona su porzioni di Stringhe, parziali o intere 
                                        comando : let word = first_word(&my_string[..]);");

        //CHIAMO LA FUNZIONE COMUNE ID STAMPA con 2 parametri a riferimento ho inserito ad entrambi & = riferimento
        stampa_variabile(&word,&msg_s);
    
    //------------------------------------------------------------------------------//

// 03)
//------------------------------------------------------------------------------//
	// `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let word = first_word(&my_string[..]);
        let msg_s=String::from ("03) MESSAGGIO - I° TIPO DI ESEMPIO STRINGA MEMORIZZATA NELLA MEMORA HEAP :
                                    `first_word` funziona anche sui riferimenti a `String`s, che sono equivalenti
                                    a intere fette di `Strings`
                                    comando : let word = first_word(&my_string[..]);
            ");
        stampa_variabile(&word,&msg_s);
    
//------------------------------------------------------------------------------//

//                  II° TIPO DI ESEMPIO

let my_string_literal = "hello world";      //STRINGA COSTANTE NELLO STACK


print!("\n
        II TIPO DI ESEMPIO - parametro con  = hello world;  - Nella STACK
-------------------------------------------------------------------------------------------------");



// 04)
//------------------------------------------------------------------------------//
	// `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let msg_s=String::from ("04) MESSAGGIO - II° TIPO DI ESEMPIO STRINGA COSTANTE MEMORIZATA NELLO STACK: 
                                `first_word` funziona su porzioni di stringhe letterali, parziali o intere
                                
                                comando : let word = first_word(&my_string_literal[0..6]);
        ");
    stampa_variabile(&word,&msg_s);

//------------------------------------------------------------------------------//

// 05)
//------------------------------------------------------------------------------//
	// `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[..]);
    let msg_s=String::from ("05) MESSAGGIO - II° TIPO DI ESEMPIO STRINGA COSTANTE MEMORIZATA NELLO STACK:
                                ...
                                
                                comando : let word = first_word(&my_string_literal[..]);
        ");
    stampa_variabile(&word,&msg_s);

//------------------------------------------------------------------------------//


// 06)
//------------------------------------------------------------------------------//
	
	// Because string literals *are* string slices already,
    	// this works too, without the slice syntax!
        let word = first_word(my_string_literal);
        let msg_s=String::from ("06) MESSAGGIO - II° TIPO DI ESEMPIO STRINGA COSTANTE MEMORIZATA NELLO STACK:
                                    Poiché le stringhe letterali *sono* già porzioni di stringa,
                                    funziona anche questo, senza la sintassi slice!
                                    
                                    comando :  let word = first_word(my_string_literal);
            ");
        stampa_variabile(&word,&msg_s);
    
//------------------------------------------------------------------------------//
    






    
    
    println!("
    
    *** fine esempio 7 ***
    ----------------------------------------------------------------------------------------------------");

}

//funzione first_word riscritta
fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes(); 
		for (i, &item) in bytes.iter().enumerate() { 
			if item == b' ' { 
				return &s[0..i]; 
				} 
				} &s[..] 
		}
        
/*         */
fn stampa_variabile(s:&str, m:&str){
    print!("
            messaggio my_string passato alla funzione word; 
            parametro               s=  {}
            
            ESERCIZIO: parametro    m=   {}
        
        ",s,m );

} 