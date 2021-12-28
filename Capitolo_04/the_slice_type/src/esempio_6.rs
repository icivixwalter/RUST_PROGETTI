pub fn stampa_6_pfunct() {
    println!("
    // VI° ESEMPIO - RICERCA SENZA  
    //_________________________________________________________________________________________________//
    //NOTE : l'esempio nro 6 si basa sulla riscrittura della first world in cui viene sempre fatta
        una ricerca della prima occorrenza dello spazio, con il seguente utilizzo:
            - UTILIZZO DELL'INIZIO DELLA STRINGA    - rappresenta l'indice INIZIALE;
            - UTILIZZO DELL'INIZIO DELLO SPAZIO     - rappresenta l'indice FINALE;
    Chiamando la funzione firs world otteniamo UN SINGOLO VALORE LEGATO AI DATI SOTTOSTANTI, e questoù
    valore è costituito da :
        UN RIFERIMENTO ALL'INDICE DI PARTENZA + IL NUMERO DEGLI ELEMENTI DELLA PARTE DI STRINGA.

    Si è quindi costruito una API solida in quanto il campilatore assicurera che i riferimenti nella
    stringa rimangono sempre validi.
    E' stato anche eliminato il bug del listato 4-8 che si basava sul fatto  che una volta trova 
    l'indice della prima parola poi la stessa è stata cancellata e quindi l'indice non è piu valido;
    questo bug ERA LOGICO e quindi l'errore si sarebbe manifestato successivamente quando  si sarebbe 
    dovuto UTILIZZARE UN INDICE CHE PUNTAVA AD UNA STRINGA VUOTA.
    IL vantaggio delle sezioni è CHE IL COMPILATORE INDIVIDUA L'ERRORE PRIMA DELL'USO delle linee di 
    codice che hanno questo problema.

    PARTE I°
    ------------------------------------------------------------------------
    I° parte del codice - assegno alla variabile mutabile il testo di stringa
                        - assegno il valore di ritorno a let word 
                            chiamando la funzione first_word(&s);

                        - s.clear = da errore perche rende invalido l'indce prima del print
    
 .");

    let s = String::from("hello");
    let word = first_word(&s); //chiamo la funzione first word
                               //
                               //s.clear(); // error! Qui da errore perche crei un indice e poi cancelli la parola.
    println!("the first word is: {}", word);
    //s.clear();   // qui da errore perchè è mutabile la parola
    println!("\n
    
    
        *** fine esempio 6 ***
        ----------------------------------------------------------------------------------------------------
      
    ");
}

//la funzione first word
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
