pub fn stampa_5_pfunct() {
    println!("
    // V° ESEMPIO - RICERCA SENZA  
    //_________________________________________________________________________________________________//
    //NOTE : Riferimento ad una parte di una stringa: 
    Nota: gli indici dell'intervallo di sezioni stringa devono essere presenti a 
    limiti di caratteri UTF-8 validi. Se si tenta di creare una fetta di stringa al 
    centro di un carattere multibyte, il programma verrà chiuso con un errore. 
    Ai fini dell'introduzione di sezioni di stringa, assumiamo ASCII solo in questa sezione; 
    una discussione più approfondita sulla gestione di UTF-8 si trova nella sezione 
    'Memorizzazione di testo codificato UTF-8 con stringhe' del Capitolo 8.");

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    


}
