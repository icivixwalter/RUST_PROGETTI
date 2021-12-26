
//la funzione è pubblica con pub (altrimenti è di base private), inoltre per utilizzarla
// occorre applicare questo codice : use crate::esempio_4::stampa_4_Pfunct;
pub fn stampa_4_pfunct() {
    println!("
    // VI° ESEMPIO - RICERCA LA STRINGA CON L'ULTIMO BYTE INCLUSO  
    //_________________________________________________________________________________________________//
    //NOTE : Riferimento ad una parte di una stringa: 
        Allo stesso modo, se la tua slice (pezzo di stringa ) include l'ultimo byte della stringa, 
        puoi eliminare il numero finale. Ciò significa che sono uguali: (l’esempio si base sul comando [3….len] o [3..] )
    "
    );

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice2 = &s[3..];

    println!(
        "\n
        la stringa base in questa variabile s:                                  {} 
            codice  ---->let s = String::from(hello);
        la lunghezza della stringa con questa variabile-  s.len:                {}
        il valore della variabile slice                                         {}
            codice  ----> let slice = &s[3..len];
        il valore della variabile slice2                                       {}
            codice  ----> let slice2 = &s[3..];
        ",
        s, len, slice, slice2
    );
}
