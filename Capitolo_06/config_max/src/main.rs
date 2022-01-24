fn main() {
    // Utilizzo di una stringa utilizzato l'heap e quindi variabile con mut
    let mut _my_string = String::from(
        "\n\t
    
    //UTILIZZO DELLE OPZIONI: MODIFICA CREATA LA FUNZIONE CHE ACCETTA 
    //PARAMETRI NULL O NUMERICI
    //------------------------------------------------------------------------------------//
    Modulo std::opzione -
    Il Type Option rappresenta un valore facoltativo: ogni opzione è Some e contiene un valore, 
    oppure None = Null non contiene un  valore. 
    I tipi di opzione sono molto comuni nel codice Rust, in quanto hanno un certo numero di usi:

    Valori iniziali
    Valori restituiti per funzioni non definite nell'intero intervallo di input 
                    (funzioni parziali)
    Valore restituito per segnalare altrimenti errori semplici, dove Nessuno viene 
                    restituito per errore
    Campi struct facoltativi
    Campi struttura che possono essere prestati o 'presi'
    Argomenti facoltativi delle funzioni
    Puntatori Nullable
    Scambiare le cose da situazioni difficili
    Le opzionis sono comunemente abbinate alla corrispondenza dei modelli per interrogare 
    la presenza di un valore e agire, tenendo sempre conto del caso Nessuno.
    //------------------------------------------------------------------------------------//

    ",
    );

    print!("messaggio: {}", _my_string);

    println!(
        "\n\t
        //CAPITOLO 6 6.3.00_Concise Control_Flow_with_if_let_Flusso 
        //di controllo conciso con if let + 
        //-----------------------------------------------------------------------------//
        Nota: La sintassi if let if  consente (con let) di combinare if let if in modo meno
        dettagliato per gestire i valori che corrispondono a un modello ignorando il resto.
        Con questo esempio viene confermato il principio di esaustivita  attivando solo
        la variabile Some cioè IL MODELLO NUMERICO  ed escludendo IL RESTO ossia 
        i valori null mediante l'utilizzo di una tupla vuota.
        ESEMPIO LISTATO:
        Listing 6-6: A  match  that only cares about executing code when the value is  Some
        \n"
    );

    let config_max = Some(9u8); //valore Some = numerico e chiamo la funzione
    some_match(config_max);
    let config_max = Some(3u8); //valore Some = numerico e chiamo la funzione
    some_match(config_max);

    let config_max = None; //valore NONE = ossia
    some_match(config_max);
    println!(
        "\n\t
        //PASSAGGIO DI UN VALORE NULLO CON None es. UTILIZZO DELL'OPZIONE
        //---------------------------------------------------------------------------------------
        Per gestire sia i valori numerici che i valori nulli un selezione di tipo match viene
        CREATA UNA NUOVA FUNZIONE DENOMINATA :
            ---> fn some_match(config_max: Option<u8>) {{....
        che ha come parametro base ... :Option<u8> ... il quale permette alla funzione di accettare
        sia i valori numerici u8 e i valori nulli che vengono gestini nell'ultimo braccio della
        Select: 
            match config_max {{....
                            Some(max) => println!(.....)    //se il valore è numerico e valorizzato
                            _ => (),                        //se il valore è nullo


        Il resquisito di esaustivita viene soddisfatto anche in questo caso con il passaggio 
        di un parametro impostato a Null
                        ---> let config_max = None;
        in quanto viene saltata la condizione Some ....
        e viene attivata la condizione della tupla vuota grazie all'utilizzo 
        dell'Opzione in  some_match(config_max: Option<u8>):
                        ---> tupla vuota!:     _ => (), \n"
    );

    //II ESEMPIO CON IL CODICE BREVE
    codice_breve();

    //III ESEMPIO UTILIZZO IF LET CON LE CARTE
    tira_carta();
}

//utilizzo del modulo Option il quale permette il passaggio di due tipi di parametri,
//Some = valorizzato numerico oppure None = nullo con il comando Option<u8>
fn some_match(config_max: Option<u8>) {
    //match = select case con 2 scelte valori numeri oppure nulli
    match config_max {
        //se max non è nullo eseguo solo questo
        Some(max) => println!(
            "\n\t   MAX E' VALORIZZATO: The maximum is configured to be {}",
            max
        ),
        _ => (), //qui se il valore è nullo NONE occorre inserire una tupla
                 //vuota	 per soddisfare il requisito di esaustivita.
    }
}

//MODIFICA: questa funzione è stata creata per evidenziare il II esercizio
fn codice_breve() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!(
            "\n\t
        //II ESEMPIO CON IL CODICE PIU BREVE UTILIZZO DI IF LET Listing 6-6:
        //---------------------------------------------------------------------------------------
        //Questo esempio è abbreviato rispetto al precedente in quanto utilizza solo il
        ramo if let:
            --->   if let Some(max) = config_max {{...
                   escludendo la tupla vuota del listato precedente:
                                ---> _ => (), ...
        con questo risultato        
        LA MASSIMA CONFIGURAZIONE: The maximum is configured to be {} \n",
            max
        );
    }
}


//esempio carta e giocatori, con l'utilizzo della Option
#[derive(Clone)]
struct Carta(u8);

struct Giocatore {
    //nome campo : tipo
    carta_posseduta: Option<Carta>,
}

//implemento il giocatoure
impl Giocatore {
    fn get_prima_carta(&mut self) -> Option<Carta> {
        //resituisce il puntatore della carta del giocatore (con &)
        self.carta_posseduta.take()
    }
}

//implemento carta = metodo tira carta

impl Carta {
    fn tira(&self) {
        println!("Il giocatore ha tirato la carta {}", self.0); // self.0 = il primo elemento della struttura-tupla Carta
    }
}

fn tira_carta() {
    let mut giocatore = Giocatore {
        carta_posseduta: Some(Carta(5)),
    };

    //il giocatore prende la carta (se c'è, prende l'optional)
    // metodo1: prende l'option lo spacchetta = unwrap e poi lo tira. (attenzione se NONE= CRASH)
    giocatore.get_prima_carta().unwrap().tira();
    // medoto 2 migliore con la tupla per il null
    match giocatore.get_prima_carta() {
        //carta_opt= espression in input al match
        //Some(...) = pattern
        Some(prima_carta) => prima_carta.tira(),
        //_  = pattern
        _ => println!("non hai carte (1)"),
    }
    // metodo 3 con la if let
    if let Some(prima_carta_2) = giocatore.get_prima_carta() {
        prima_carta_2.tira();
    } else {
        println!("Non hai carte (2)");
    }
}
