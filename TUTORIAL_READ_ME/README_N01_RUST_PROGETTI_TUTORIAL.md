> # RUST LINGUAGGIO E PROGETTI

path dei progetti:

```
C:\CASA\RUST_PROGETTI\
```

salvataggio tutorial  su git: path di salvataggio

```
C:\CASA\PROGRAMMI\GITHUB\GESTIONI\CASA\LINGUAGGI\RUST_PROGETTI

```

NOME FILE

```
README_RUST_PROGETTI_TUTORIAL.md

```

# RUST INSTALLAZIONE

1. scarica RUST https://www.rust-lang.org/it 64 BIT
2. F:\CASA\LINGUAGGI\RUST
   salvato eseguibile
3. su VS code installare plug-in di rust Su IJ installare plug-in di rust
4. Vai su gitub e copia URL del progetto: https://github.com/redjack96/first_rust

> ATTENZIONE se manca Visual Studio 2019 "linker `link.exe` not found"
> occorre installare Visual C++
> scaricalo qui:
> https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16
> questo file eseguibile:
> vs_community__2055935248.1638292004.exe
> METTI SOLO LA FUNZIONE FACOLTATIVA msvc il resto no.

# TUTORIAL RUST

NOTE: i vari comandi ed i collegamenti tutorial.

vedi qui:
https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html

RIGA DI COMANDO RUST = https://rust-cli.github.io/book/tutorial/setup.html

01.01 ) Configurazione del progetto
Un file che contiene metadati Nper il nostro progetto,
incluso un elenco di dipendenze / librerie esterne
che utilizziamo.Cargo.toml

01.01.a) RUST -> CREARE UN PROGETTO
Per la creazione di un nuovo progetto il comando è : cargo new `<nome progetto>`

cargo new HelloWord

viene creato il progetto con la configurazione tipica per un progetto Rust

- Un file che contiene metadati per il nostro progetto, incluso un elenco di dipendenze / librerie esterne che utilizziamo.Cargo.toml
- Un file che è il punto di ingresso per il nostro binario (principale).src/main.rs

01.01.b) RUST -> COMPILAZIONE DI UN PROGETTO (COMPILARE O COMPILAZIONE DI UN FILE O DI UN PROGETTO)
CARGO RUN
Per la compilazione utilizzare questi comandi:
cargo run --example  = visualizza i progetti compilabili

CARGO RUN --EXAMPLE `<nome progetto>`
Per compilare un singolo progetto
posizionarsi sulla tabella principale dei compilati ed usare il comando:
cargo run --example echo_d
cargo run 	= comando esegui
--example 	= ??
echo_d 		= nome del compilato

01.02 Analisi degli argomenti della riga di comando
https://rust-cli.github.io/book/tutorial/cli-args.html

invocazione del nostro strumento CLI sarà simile alla
$ grrs foobar test.txt

Il testo dopo il nome del programma è spesso chiamato "argomenti della riga di comando" o "flag della riga di comando" (specialmente quando assomigliano a ). Internamente, il sistema operativo di solito li rappresenta come un elenco di stringhe - in parole povere, vengono separati da spazi.--this

01.02.A) Ottenere gli argomenti
La libreria standard contiene la funzione std::env::args() che fornisce un iteratore degli argomenti dati.

difficile trovato questo link:
https://learntutorials.net/it/rust/topic/7015/argomenti-della-riga-di-comando

Argomenti della riga di comando
algorithm C Language C# Language C++ iOS JavaScript GNU/Linux Node.js Python Language Regular Expressions

introduzione
La libreria standard di Rust non contiene un parser argomento appropriato (a differenza di argparse in Python), preferendo invece lasciarlo a casse di terze parti. Questi esempi mostreranno l'uso sia della libreria standard (per formare un gestore di argomenti grezzi) che della libreria clap grado di analizzare gli argomenti della riga di comando in modo più efficace.

Sintassi
usa std :: env; // Importa il modulo env
let args = env :: args (); // Archivia un iteratore Args nella variabile args.
Utilizzando std :: env :: args ()

II TUTORIAL IN ITALIANO = https://doc.rust-lang.org/book/ch01-02-hello-world.html

1. Per iniziare
   1.1. Installazione
   Installazione di rustup su Windows
   Su Windows, vai su https://www.rust-lang.org/tools/install e segui le istruzioni per l'installazione di Rust. Ad un certo punto dell'installazione, verrà visualizzato un messaggio che spiega che saranno necessari anche gli strumenti di compilazione C++ per Visual Studio 2013 o versione successiva. Il modo più semplice per acquisire gli strumenti di compilazione consiste nell'installare gli strumenti di compilazione per Visual Studio 2019. Quando viene chiesto quali carichi di lavoro installare, assicurarsi che sia selezionato "Strumenti di compilazione C++" e che siano inclusi i componenti di Windows 10 SDK e del Language Pack in inglese.
   Aggiornamento e disinstallazione
   Dopo aver installato Rust tramite , l'aggiornamento all'ultima versione è semplice. Dalla shell, eseguire il seguente script di aggiornamento:rustup
   $ rustup update  (pawershell)
   su cmd
   rustc --version  (controlla la versione)
   rustup update  (su cmd)

Creazione di una directory di progetto
Inizierai creando una directory per memorizzare il tuo codice Rust. Non importa a Rust dove vive il tuo codice, ma per gli esercizi e i progetti in questo libro, ti suggeriamo di creare una directory di progetti nella tua home directory e di mantenere tutti i tuoi progetti lì.

CREARE UNA DIRECTORY
Apri un terminale e inserisci i seguenti comandi per creare una directory di progetti e una directory per il progetto "Hello, world!" all'interno della directory dei progetti.
ATTENZIONE NON UTILIZZATO PERCHE' HO CREATO GIA UNA DIRECTORY (c:\CASA\PROGRAMMI\RUST_PROGETTI\HelloWord\)

> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world

Scrittura ed esecuzione di un programma Rust - CREARE UN FIL SORGENTE
Quindi, crea un nuovo file sorgente e chiamalo
main.rs.
I file di ruggine terminano sempre con l'estensione .rs. Se si utilizzano più parole nel nome del file, utilizzare un carattere di sottolineatura per separarli. Ad esempio, utilizzare hello_world.rs anziché helloworld.rs.

su pawer shell
#rustc main.rs
su dos
rustc main.rs  (senza cancelletto)

(attenzione il file .rs viene trasformato in exe) e stampa a termina il messaggio

rustc = compila in eseguibile il nome del file (rustc main.r)
per attivare l'eseguibile occorre scrivere ./main e viene attivato.
Anatomia di un programma Rust
Esaminiamo in dettaglio il file
fn main() {
} = funzione speciale rust che non ha parametri e non restituisce nulla
Se ci fossero parametri, andrebbero tra parentesi, .mainmain()
Inoltre, si noti che il corpo della funzione è racchiuso tra parentesi graffe, . La ruggine richiede questi intorno a tutti i corpi funzionali

Quarto, finiamo la riga con un punto e virgola (), che indica che questa espressione è finita e la successiva è pronta per iniziare. La maggior parte delle righe del codice Rust termina con un punto e virgola.;

STRUMENTO PER LA FORMATTAZIONE AUTOMATICA del codice rust es. rustfmt rustc

La compilazione e l'esecuzione sono passaggi separati
Prima di eseguire un programma Rust, è necessario compilarlo utilizzando il compilatore Rust inserendo il comando e passandogli il nome del file sorgente, in questo modo:
rustc `<nome file sorgente>`

fino qui
https://doc.rust-lang.org/book/ch01-02-hello-world.html

# LE LIBRERIE DI RUST IN ORDINE AA-ZZ

## GUI RUST	dove scaricare le librerie

AZUL            https://lib.rs/crates/fltk
gui nativive di windows
https://github.com/gabdube/native-windows-gui

DRUID
vari esempi di gui (CALCOLATRICE + BUTTON, BARRE)
https://github.com/linebender/druid

FLTK
??

# CONCETTI CHIAVE DI RUST

## CONVENZIONI RUST

### Costanti

```
Tutte le costanti devono essere rappresentate con lettere MAIUSCOLE SEPARATA DA _ (underscore)
```

### TIPI DI DATI

rust due tipi di dati: SCALARI  e  COMPOSTO

#### Tipo scalare
@tipo.scalare

Il tipo scalare è formato da 4 tipi:

```
interi, 
numeri in virgola mobile,
booleani 
caratteri
```

Tipi di interi:

```
Length	Signed	Unsigned
8-bit	i8		u8
16-bit	i16		u16
32-bit	i32		u32
64-bit	i64		u64
128-bit	i128	u128
arch	isize	usize
```

quelli con u sono con segno quelli con i senza segno.
es.
i8 =  -(27) a 27 - 1 cioè da  -128 a 127
u8 =   0 a 28 - 1, cioè  da 0 - 255.

## STRUCT

```
NOTA: struct è una struttura che in Java viene definita come classe con solo campi senza metodi.
```

## TRAIT

```
NOTA: è un INTERFACCIA  che in Java viene definita come Interface  con solo  metodi senza implementazione.
```

## IMPL

```
NOTA: rappresenta l'implementazione dei metodi di uan Struct o Trait??
```

## ALTRE DEFINIZIONI

# LIBRERIE DI RUST

## I

### io

io è una  LIBRERIA che fornisce funzioni di input e output

#### Importazione

```
	use std::io;
```

#### Esempio - leggere dalla linea di comando

INPUT DA TASTIERA e lettura dalla linera di comando
io:stdin().read_lines(&mut guess)
.expect("Fallito la lettura di linea")

##### Spiegazione

- io:stdin(): ritorna un'istanza di stdin::io::Stdin
- .read_lines(&mut guess): invoca il metodo di Stdin che legge una linea di testo

## R

### rand

```
use rand::Rng;						libreria numeri 
```

rand genera un intervallo di argomento di numeri casuali tra 1-101; es:
let secret_number = rand::thread_rng().gen_range(1..101); //errore
per utilizzare questa libreria occorre inserire le dipendenze nel file
.tomm in questo modo:

> per il progetto ho dovuto inserire una dipendenza

```
...
[dependencies]
rand = "0.8.4"
```

> ATTENZIONE le librerie si aggiornano di continuo: ogni tanto controlla se ci sono nuovi aggiornamenti con `cargo update`
> ESEMPIO DI GENERAZIONE DI UN NUMERO CASUALE CON RAND:

# PAROLE CHIAVE DI RUST

NOTA: sono le parole chiave per attivare funzioni in Rust. Le paro chiave sono riservate e non
è possibile utilizzarle come nome di VARIABILI O FUNZIONI. L'elenco delle parole chiave si trovano
nell' APPENDICE A.

# A  

## Array 
@push.in.Rust
CICLO FOR  AD INTERVALLO INVERTITO, DA 1 a 4
Si prevede il conto alla rovescia tramite .rev.
Tale ciclo è stato modifcato costruendo un ciclo sempre avendo
come base  un array i cui elementi possono essere aumentati
o diminuiti calcolando in modo automatico l'indice min e max
con la seguente istruzione:
for a_for2 in (min..max).rev() { ....
In questo modo il ciclo è esente da errori fuori indice in quanto
utilizza il costrutto FOR molto piu sicuro del while ed il calcolo
automatico degli indici dell'array.

```
esempio di array con il ciclo for che permette il controllo perfetto dell'indice.

 
            //assegnazione degli elemti dell'array di tipo i32 = numerici
            let array = [10, 20, 30, 40, 50, 60];  //array
            //calcolo dell'intervallo minimo e massimo
            let min=0;
            let max=array.len();
            //ciclo for sull'intervallo calcolato e con ciclo inventivo 
            for a_for2 in (min..max).rev() { 
                //stampo varore indice e varlore dell'array
                println!("valore indice: a_for2: {}! -  valore array : {}", a_for2, array[a_for2]);
            
            } 
            println!(" fine array : LIFTOFF!!!  \n\n"); 
```

# B
@B.lettera.B.in.Rust

# C 
@C.lettera.C.in.Rust

# D 
@D.lettera.D.in.Rust

## DICHIARAZIONI 
@DICHIARAZIONI.in@push.in.Rust.Rust

Le DICHIARAZIONI (Statements) in rust sono ISTRUZIONI che eseguono alcune azioni e
non restituiscono un valore. I corpi funzione sono costituiti da una serie di istruzioni che terminano facoltativamente con un'espressione. Le funzioni possono avere una ESPRESSIONE FINALE cioè una
espressione che restituisce un valore (vedi le espressioni).
vedi Esempio di utilizzo della macro [ESEMPIO DI MACRO] ```[ESEMPIO DI MACRO]-macro-debug-122-nrust)

# E  
@E.lettera.E.inRust
## enum
@enum.in.rust, @enumeratori.in.rust, @strutture.in.rust
@le.strutture.in.rust

gli enumeratori in rust si costruiscono cosi con la parola chiave 1) enum, 2) poi la funzione di scelta e 3) il modo di chiarama:

	//ENUMERATORE = enumeratore con 4 scelte mono linea ogni braccio restituisce 1 valore
	//---------------------------------------------------------------------------------//
		//1)
		//COSTRUZIONE DI ENUMERATORE
			enum Coin {
			Penny,
			Nickel,
			Dime,
			Quarter,
		}

		//2)
		//la funzione di scelta con 4 bracci 
		fn value_in_cents(coin: Coin) -> u8 {
			match coin {
				Coin::Penny => 1,
				Coin::Nickel => 5,
				Coin::Dime => 10,
				Coin::Quarter => 25,
			}
		}
		//3)
		//chiami l'enumeratore e restituisce un valore
		let ni:u8= value_in_cents(Coin::Nickel);
	//---------------------------------------------------------------------------------//


@faq:
@come.richiamare.una.struttura.associata.ad.un'altra.struttura
@struttura.associata.ad.un'altra.struttura

Inoltre per vedere un esempio di una struttura che richiama un'altra struttura occorre fare
riferimento al Capitolo 6 esercizio bind_values 

	//STRUTTURA CHE RICHIAMA UN'ALTRA STRUTTUA
	//---------------------------------------------------------------------------------//
	//ESERCIZIO COMPLETO NEL CAPITOLO 6 bind_values ....

			//struttura STATI
			enum UsState { 
				Alabama, 
				Alaska, // --snip taglia-- 
			} 

			//struttura monete
			enum Coin { 
				Penny, 
				Nickel, 
				Dime, 
				Quarter(UsState),       //associazione all'altra struttura
				} 

				
			fn value_in_cents(coin: Coin) -> u8 { 
				match coin 
				{ Coin::Penny => 1, 
				Coin::Nickel => 5, 
				Coin::Dime => 10, 
				Coin::Quarter(state) => { 
					println!("State quarter from {:?}!", state); 25 } 
				} 
			} 
				
		//PER RICHIAMARE la struttura Stati associata alla struttura monete:
			value_in_cents(Coin::Quarter(UsState::Alaska))
			

	//---------------------------------------------------------------------------------//


## Espressioni 
@Espressioni.in@push.in.Rust.Rust

Le espressioni vengono valutate in un valore risultante (ESPRESSIONE FINALE).
Le espressioni in rust  VALUTANO UN VALORE e possono essere parti di una istruzione
con la restituzione di un valore. Esempio di espressioni che possono essere costruite:

espressioni che valutano un valore e possono essere parti di una istruzione sono :

- operazioni matematica 		(es.	:	 operazione 5+7)
- creare una funzione			(es.	: 	fn main() { fine };
- chiamare una funzione;		(es.	:	 another_function(5);
- chiamare una macro;
- creare un blocco di istruzione con restituzione
  del valore mendiante le parentesi graffe:{}

  ```
    (es.	:{ let x = 3; x + 1 };) questa espressione 
    restituisce un valore perché l’ultima variabile x viene
    incrementata e restituisce il valore in quanto priva del
    punto e virgola ( ; ). I rust il RETURN si ottiene senza 
    il punto e virgola (;) altrimenti diventa una espressione SENZA RITORNO.
  ```

faq_Espressioni
Le espressioni RITORNANO IL VALORE in quanto prive del ;
rust Return 	= senza il ;
Return in rust	= senza il ; è una espressione.
faq_Istruzioni
Le istruzioni non sono associate a nulla e quindi
non restituiscono valori

## Etichetta - label 
@Etichetta.label.in.R@push.in.Rustust

in rust l'etichetta viene costruita con 'counting_up:

```
   'counting_up: loop {                //ETICHETTA DEL CICLO LOOP- counting_up:	
   		println!("LOOP 1 CICLO ESTERNO count = {}", count);
		   	.....

			    if count == 2 {
                    break 'counting_up; //richiama l'etichetta
                }
				......
   }
```

## ERRORI IN RUST
faq:
@errori.in.rust,  @error.in.rust
Gli errori in rust vengono classificato per numero con la soluzione
### error[E0463]
faq:
@error[E0463]

	01)
	error[E0463]: can't find crate for `native_windows_gui`

	Quando nel file originale si ha questo riferimento:
	extern crate native_windows_gui as nwg;
	per evitare questo occorre inserire nelle dipendenze del toml questo comando:
	[dependencies]
	native-windows-gui = "1.0.12"

	02)
	error[E0463]: can't find crate for `native_windows_derive`

	Quando nel file originale è stato inserito questo riferimento: 
	extern crate native_windows_derive as nwd;  // Optional. Only if the derive macro is used.
	Per evitare questo  tipo di errore è perchè non trova questa libreria nel file .toml
		
	inserire tra le dipendenze
	[dependencies]
	native-windows-derive = "1.0.4" # Optional. Only if the derive macro is used.


[An internal link](#atext) to arbitrary text.

# F 
@F.lettera.F.in.Rust 

## fn 
@fn.push.in.Rust,  @fn.funzioni.in.Rust, @la.funzione.in.Rust, @costruire.una.funzione.in.Rust
@fn.comando.funzione.in.Rust

#### DEFINIZIONE DELLE FUNZIONI 
@definizione.delle.funzioni.in.Rustust

```
Le funzioni vengono definite con la parola chiave:
	fn
i parametri devono essere definiti nella firma della funzione e le stesse possono essere chiamate ovunque
a condizioni che siano definite e che sia fornito il parametro richiesto. Di seguito viene indicato il metodo
di una chimata ad una funzione passando il parametro 5 e la firma della stessa:

	another_function(5);	//chiamata della funzione

	// FIRMA DELLA FUNZIONE COSTRUITA CON 1 parametro x:i32 – i parametro ha il tipo : i32
	fn another_function(x: i32) {
		println!("The value of x is: {}", x);  //esempio di funzione - definizione di una funzione

	}
```

[An internal link](#atext) to arbitrary text.

```
//FIRMA DELLA FUNZIONE CON 2 PARAMETRI
//funzione con 2 parametri, 1° i32, 2° char (nella fuzione i parametri devono essere separati da virgole)
fn print_labeled_measurement(value: i32, unit_label: char) { 
			println!("The measurement is: {}{}", value, unit_label); 
		} 

		faq: come si costruisce una funzione - costruire una funzione
			la definizione di una funzione
			firma di una fuzione, la firma della funzione
			rust le funzioni, rust funzioni
			rust funzioni con piu parametri - funzioni con 2 parametri
```

CARATTERISTICA PECULIARE DELLE FUNZIOI IN RUST

Le funzioni possono restituire valori al codice chiamante. Non VENGONO CHIAMATI i valori restituiti, ma dichiariamo il tipo dopo una freccia (->).

```
es. di funzione:
	//FIRMA DELLA FUNZIONE
	fn plus_one(x: i32) -> i32 {
				x + 1}  //viene restituito la x incrementata al chiamante, ATTENZIONE SENZA ;

	//CHIAMATA DELLA FUNZIONE
	 let x = plus_one(5);			\\ la chiamata può essere inserita in qualsiasi parte del codice
	 								\\ purchè la funzione sia definita in anticipo.
```

In Rust, il valore restituito della funzione è sinonimo del valore dell'espressione finale nel blocco del corpo di una funzione. È possibile restituire in anticipo da una funzione utilizzando la parola chiave RETURN e specificando un valore, ma la maggior parte delle funzioni restituisce implicitamente l'ultima espressione IN MODO IMPLICITO.

```
faq_Funzione
	rust le funzioni, firma della funzione, chiamata della funzione, Rerturn della funzione (da spiegare
	meglio sul libro!!)
```

# G 
@G.lettera.G.in.Rust

# H  
@H.lettera.H.in.Rust

# I  
@I.lettera.I.in.Rust

## if 
@if.con.push.in.Rustndzione.in.Rust., @condizione.if.in.Rust, @costrutto.if.in.Rust

vedi: 3.5.01 if Expressions

Rust permette il controllo di flusso con la parola chiave if
esempio di if con l'opzione else facoltativa:

```
	let number = 3; 
	if number < 5 { 
			println!("condition was true"); 
		} 
		else { 
			println!("condition was false"); 
		}
```

## if con let 
@if.con.let
@push.in.Rust
Vedi 3.5.03 Using if in a let Statement - Utilizzo di if in un'istruzione let
Poiché if è un'espressione, possiamo usarla sul lato destro di un'istruzione let.
Nella if i valori che hanno il potenziale per il  risultato per  ciascun braccio dell'if
devono essere dello stesso tipo i32 altrimenti eccezione di errore es.

```
let number = if condition { 5 } else {6}; //I DUE BRACCI DEVONO AVERE LO STESSO TIPI i32
```

## if con piu condizioni else
faq:	@if.then; @condizione.if;  @if.then.else

vedi : 3.5.02 Handling Multiple Conditions with else if - Gestire più condizioni con else if

la if con piu condizioni else sono costuirete con il temine if else, viene attivata la prima vera e scartate
tutte il resto. Per evitare troppe conzioni if si puo rifattorizzare il codice con il costruto match

```
let number = 6; 
if number % 4 == 0 { 
	println!("number is divisible by 4"); } 
else if number % 3 == 0 { 
	println!("number is divisible by 3"); } 
else if number % 2 == 0 { 
	println!("number is divisible by 2"); } 
else { println!("number is not divisible by 4, 3, or 2"); } 
```

# L 
@L.lettera.L.in.Rust

## let 
@push.in.Rust
COMANDO LET assegna valore ALLE VARIABILI  (java = var) in rust le assegnazioni sono Final o costanti
per renderli MUTEVOLI occorre mut. TIPO DI VARIABILI non è necessario definirle
in rust perchè le deduce il compilatore.
es.

```
let apples = 5;
```

altro esempio di costante: tipo u32 : i secondi contenuti in 3h

```
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

shadowing = oscuramento e riutilizzo con altro valore
se eiste nel pgramma un'altra variabile a cui è stato assegnato una stringa denominata
guess tipo un assegnamento di una strina: ( let mut guess = String::new(); //qui viene ASSEGNATO )
per trasformara la variabile in intero 32 occorre per prima cosa (trim) eliminare gli spazi e
new line di imput e poi trasformare la strina in intero u32.

```
 let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

## loop 
@loop.in@push.in.Rust.Rust

NOTE:  Rust ha tre tipi di loop:  `loop, while e for `

modello Loop esempio:
si crea con la parola chiave loop e le parentesi graffe, es.

```
//esempio di loop
loop {
    println!("Please input your guess.");

    // --snip--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Loop base - esempio di doppio ciclo loop

```
'counting_up: loop        //ETICHETTA DEL CICLOC ESTERNO- counting_up:
{   //loop esterno         
        		...........
        	//loop interno
            loop 	{
              	.............
            	}
  
            count += 1;
   
    }
```

## loop while 
@loop.while.in@push.in.Rust.Rust, @ciclo.while.in.Rust

Loop con while
Il ciclo while presuppone che finche la condizione sia vera il ciclo viene eseguito quando è
diversa il ciclo viene interrotto con break.
Questo costrutto elimina molti annidamenti che sarebbero necessari se si utilizzasse loop, if, else e break, ed è più chiaro. Mentre una condizione è vera, il codice viene eseguito; in caso contrario, esce dal ciclo.

```
Esempio di ciclo while :

 				let mut number = 3; 
                //finche è diverso da zero continua la stampa
                while number != 0 { 
                    println!("CONTEGGIO ALL'INDIETRO {}!", number); number -= 1; //=-1 per conteggio all'indietro
                } 
                println!("\n
                        LIFTOFF!!! \n\n"
                ); 
```

## loop for rust
@for.in.Rust, @ciclo.for.in.Rust

loop con for

# M 

(rust M - comandi con la lettera M)

## match 
@push.in.Rust
rifattorizzazione delle condizioni else .....

## mut 
@mut.in@push.in.Rust.Rust

mut = variabili mutevole altrimenti di default sono final.
es.

```
mut a=10;
```

# N 
@N.lettera.N.in.Rust

# O 
@O.lettera.O.in.Rust

# P 
@P.lettera.P.in.Rust

(rust P - comandi con la lettera P)

## push 
@push.in.Rust
```
comando rust dedicato alla stringhe che permette l'aggiunta ad una variabile di 
un ulteriore pezzo di stringa come ad es.

//RUST COMANDO PUSH
//--------------------------------------------------------------------------------------------------//
	//01) creo una variabile mut con s, di tipo stringa inserita nella memoria heap con ::from
	//02) chiamo la funzione walter passando la variabile stringa per riferimento e mutabile
	//		con il comando (&mut s)
	//03) stampo il risultato finale
		//01
	    let mut s = String::from("provo la stringa mutabile: ");
		//02
		walter(&mut s);  // se metto riferimento &mut = presto il riferimento mutabile
		//03
		println!("stampo la stringa restituita dalla funzione walter() {}",s);



	//funzione walter che riceve un parametro stringa mutevole &mut
	//questa funzione deve essere posizionata fuori dal main
	fn walter(stringa: &mut String) {
		stringa.push_str("walter");  //push aggiunge alla fine e lo restituisce al chiamante
		}
//--------------------------------------------------------------------------------------------------//
```

# Q 

# R 

# S 

# T 

# U 

# V 

# Z 

# XX FINE ------------------------------------------------------------------------

# LE APPENDICI DI RUST (LIBRERIA STANDARD) ---------------------------------------

Le appendici in rust da studiare:

### APPENDICE A 
@push.in.Rust
### APPENDICE B 
@push.in.Rust
### APPENDICE C 
@push.in.Rust
### APPENDICE D 
@push.in.Rust
# APPENDICE A  - ELENCO PAROLE CHIAVE IN RUST -
@push.in.Rust
| PAROLA CHIAVE | definizione parola chiave                                                              |
| ------------- | -------------------------------------------------------------------------------------- |
| as            | - perform primitive casting, disambiguate the specific trait containing an item, or    |
| async         | - return a Future instead of blocking the current thread                               |
| await         | - suspend execution until the result of a Future is ready                              |
| break         | - exit a loop immediately                                                              |
| const         | - define constant items or constant raw pointers                                       |
| continue      | - continue to the next loop iteration                                                  |
| crate         | - link an external crate or a macro variable representing the crate in which the macro |
| dyn           | - dynamic dispatch to a trait object                                                   |
| else          | - fallback for if and if let control flow constructs                                   |
| enum          | - define an enumeration                                                                |
| extern        | - link an external crate, function, or variable                                        |
| false         | - Boolean false literal                                                                |
| fn            | - define a function or the function pointer type                                       |
| for           | - loop over items from an iterator, implement a trait, or specify a higher-ranked      |
| if            | - branch based on the result of a conditional expression                               |
| impl          | - implement inherent or trait functionality                                            |
| in            | - part of for loop syntax                                                              |
| let           | - bind a variable                                                                      |
| loop          | - loop unconditionally                                                                 |
| match         | - match a value to patterns                                                            |
| mod           | - define a module                                                                      |
| move          | - make a closure take ownership of all its captures                                    |
| mut           | - denote mutability in references, raw pointers, or pattern bindings                   |
| pub           | - denote public visibility in struct fields, impl blocks, or modules                   |
| ref           | - bind by reference                                                                    |
| return        | - return from function                                                                 |
| Self          | - a type alias for the type we are defining or implementing                            |
| self          | - method subject or current module                                                     |
| static        | - global variable or lifetime lasting the entire program execution                     |
| struct        | - define a structure                                                                   |
| super         | - parent module of the current module                                                  |
| trait         | - define a trait                                                                       |
| true          | - Boolean true literal                                                                 |
| type          | - define a type alias or associated type                                               |
| union         | - define a union and is only a keyword when used in a union declaration                |
| unsafe        | - denote unsafe code, functions, traits, or implementations                            |
| use           | - bring symbols into scope                                                             |
| where         | - denote clauses that constrain a type                                                 |
| while         | - loop conditionally based on the result of an expression                              |

# APPENDICE B  - Gli Operatori  -
@push.in.Rust
Operator	Example	Explanation	Overloadable?

```
Table B-1 
La tabella B-1 contiene gli operatori in Rust, un esempio di come l'operatore apparirebbe nel contesto, una breve spiegazione e se quell'operatore è sovraccaricabile. Se è possibile eseguire l'overload di un operatore, viene elencata la caratteristica pertinente da utilizzare per eseguire l'overload di tale operatore.


OPERATORI   | Esempi			| Spiegazione						|Sovraccaricabile?
------------|-------------------------------------------------------------------------
			|ident!(...),		|									|
!			|ident!{...},		| Macro expansion					|
			|ident![...]		| 									|
!			|!expr	 			|Bitwise or logical complement		|Not
!=			|var != expr		|Nonequality comparison				|PartialEq			
%			|expr % expr		|Arithmetic remainder				|Rem
			|					| 									|
%=			|var %= expr		|Arithmetic remainder and 			|RemAssign
			|					|assignment							|
&			|&expr, &mut expr	|Borrow								|
			|&type, &mut type, 	|									|
			 &'a type, &'a mut  |Borrowed pointer type				|
			 type				|									|
&			|expr & expr		|Bitwise AND						|BitAnd
&=			|var &= expr		|Bitwise AND and assignment			|BitAndAssign
&&			|expr && expr		|Short-circuiting logical AND		|
*			|expr * expr		|Arithmetic multiplication			|Mul
*=			|var *= expr		|Arithmetic multiplication and 		|
			|					|assignment	MulAssign				|
*			|*expr				|Dereference						|Deref
*			|*const type, *mut 	|									|
			|type				|Raw pointer						|
+			|trait + trait, 'a 	|									|
			|+ trait			|Compound type constraint			|
+			|expr + expr		|Arithmetic addition				|Add
+=			|var += expr		|Arithmetic addition and 			|
			|					|assignment							|AddAssign
,			|expr, expr			|Argument and element				|
			|					|separator							|
-			|- expr				|Arithmetic negation				|Neg
-			|expr - expr		|Arithmetic subtraction				|Sub
-=			|var -= expr		|Arithmetic subtraction 			|
			|					|and assignment						|SubAssign
->			|fn(...) -> type, 	|									|
			||...| -> type		|Function and closure return type	|
.			|expr.ident			|Member access						|
..			|.., expr.., 		|									|
			|..expr, 			|									|
			|expr..expr			|Right-exclusive range literal		|PartialOrd
..=			|..=expr, 			|									|
			|expr..=expr		|Right-inclusive range literal		|PartialOrd
..			|..expr				|Struct literal update syntax		|
..			|variant(x, ..), 	|									|
			|struct_type { 		|									|
			|x, .. }			|“And the rest” pattern binding		|
...			|expr...expr		|									|
			|					|(Deprecated, use ..= instead) In 	|
			|					|a pattern: inclusive range 		|
			|					|pattern							|
/			|expr / expr		|Arithmetic division				|Div
/=			|var /= expr		|Arithmetic division and 			|
			|					|assignment							|DivAssign
:			|pat: type, ident: 	|									|
			|type				|Constraints						|
:			|ident: expr		|Struct field initializer			|
:			|'a: loop {...}		|Loop label							|
;			|expr;				|Statement and item terminator		|
;			|[...; len]			|Part of fixed-size array syntax	|
<<			|expr << expr		|Left-shift							|Shl
<<=			|var <<= expr		|Left-shift and assignment			|ShlAssign
<			|expr < expr		|Less than comparison				|PartialOrd
<=			|expr <= expr		|Less than or equal to 				|PartialOrd
			|					|comparison							|
=			|var = expr, ident 	|Assignment/equivalence				|
			|= type				|									|
==			|expr == expr		|Equality comparison				|PartialEq
=>			|pat => expr		|Part of match arm syntax			|
>			|expr > expr		|Greater than comparison			|PartialOrd
>=			|expr >= expr		|Greater than or equal to 			|PartialOrd
			|					|comparison							|
>>			|expr >> expr		|Right-shift						|Shr
>>=			|var >>= expr		|Right-shift and assignment			|ShrAssign
@			|ident @ pat		|Pattern binding					|
^			|expr ^ expr		|Bitwise exclusive OR				|BitXor
^=			|var ^= expr		|Bitwise exclusive OR and 			|BitXorAssign
			|					|assignment							|			
|			|pat | pat			|Pattern alternatives				|
|			|expr | expr		|Bitwise OR	BitOr					|
|=			|var |= expr		|Bitwise OR and assignment			|BitOrAssign
||			|expr || expr		|Short-circuiting logical OR		|
?			|expr?				|Error propagat						|
```

# LE MACRO RUST IN ORDINE AA-ZZ

NOTE le macro sono inserite nel linguaggio e fanno manipolazione del Toke stream; sono riconosciute
dal punto esclamativo (!)

# D MacroInRust

le macro in rust della lettera D

## debug!

La macro in rust che permette di utilizzare la stampa sul flusso della console di errore standard (stderr),
utile per il debug.

```
width: dbg!(30 * scale), //qui dbg stampa anche la riga 30 con il valore all'interno di una struttura.
```

### dbg!

Utilizzo del comando debug in rust mediante il comand dbg! che puo stampare la riga individuata oppure l'intera struttura:

```[ESEMPIO


# P MacroInRust
## println! 
	println!@push.in.Rust("text,tex");	
La macro di stampa che sono esterne al compilatore. viene espansa dal
compilatore in una serie di istruzioni
La macro  println! in rust permette di stampare sul flusso della console di output standard (stdout).









# I COMANDI CARGO IN ORDINE AA-ZZ					 
NOTE: il punto e virgola identifica uno STATEMENT cioè non ritorna un valore quindi è sempre necessario altrimenti
puoi scrivere funzioni senza il ; e quindi ritornano il valore al posto di Reuturn

## C

	cargo --version				
CONTROLLO DI VERSIONE = controlla la versione è ma è piu lenta di rustc 
strumento che esegue il programma, dopo la compilazione ed il link. Cargo è l'equivalente di MAVEN
gestisce le dipendenze.

### cargo new
	cargo new <progetto>		

CREARE UN NUOVO PROGETTO ESEGUIBILE
(crea la directory con la serie di file compresa la configurazione
di git) cargo new = genera i file main.rs quindi sono i sorgenti costruiti
nella directory src

### `cargo -- version`

	Cargo controllo della VERSIONE se installato.


### `cargo new --vcs=git`
	Cargo git non viene generato

### `cargo help`
	Cargo I flag o le opzioni disponibili


### `cargo build`
	Cargo compila esegue la compilazione ed assembla l'eseguibile, 	creando una nuova directory chiamata
	target + debug

### `cargo build [OPTIONS]`
	CARGO LE OPZIONE scrivere 
	cargo.exe help oppure cargo -h

 parametro                   	| significato
--------------------------------| ----------------------------------------
-q, --quiet					 	|No output printed to stdout
-p, --package \<SPEC>			|Package to build (see `cargo help pkgid`)
--wokspace		   			 	| Build all packages in the workspace
--exclude \<SPEC>...			| Exclude packages from the build
--all						 	| Alias for --workspace (deprecated)
-j, --jobs \<N>               	| Number of parallel jobs defaults to # of CPUs
--lib                        	| Build only this package's library
--bin \<NAME>...              	|Build only the specified binary
--bins                       	| Build all binaries
--example \<NAME>...          	| Build only the specified example
--examples                   	| Build all examples
--test \<NAME>...             	| Build only the specified test target
--tests                      	| Build all tests
--bench \<NAME>...            	| Build only the specified bench target
--benches                    	| Build all benches
--all-targets                	| Build all targets
--release                    	| Build artifacts in release mode, with optimizations
--profile \<PROFILE-NAME>     	| Build artifacts with the specified profile
--features \<FEATURES>...     	| Space or comma separated list of features to activate
--all-features               	| Activate all available features
--no-default-features        	| Do not activate the `default` feature
--target \<TRIPLE>...         	| Build for the target triple
--target-dir \<DIRECTORY>     	| Directory for all generated artifacts
--out-dir \<PATH>             	| Copy final artifacts to this directory (unstable)
--manifest-path \<PATH>       	| Path to Cargo.toml
--ignore-rust-version        	| Ignore `rust-version` specification in packages
--message-format \<FMT>...    	| Error format
--build-plan                 	| Output the build plan in JSON (unstable)
--unit-graph                 	| Output build graph in JSON (unstable)
--future-incompat-report     	| Outputs a future incompatibility report at the end of the build 								  (unstable)
-v, --verbose                	| Use verbose output (-vv very verbose/build.rs output)
--color \<WHEN>               	| Coloring: auto, always, never
--frozen                     	| Require Cargo.lock and cache are up to date
--locked                     	| Require Cargo.lock is up to date
--offline                    	| Run without accessing the network
--config \<KEY=VALUE>...      	| Override a configuration value (unstable)
-Z \<FLAG>...                 	| Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for 									 details
-h, --help                   	|  Prints help information - help di cargo

>**cargo** help build  Run cargo con le informazioni (attiva cargo con i dettagli del comando)

Comando		      | Descrizione
----------------- | ---------------------------------------
cargo build, b    | Compile the current package
cargo check, c    | Analyze the current package and report errors, but don't build object files
cargo clean       | Remove the target directory
cargo doc, d      | Build this package's and its dependencies' documentation
cargo new         | Create a new cargo package
cargo init        | Create a new cargo package in an existing directory
cargo run, r      | Run a binary or example of the local package
cargo test, t     | Run the tests
cargo bench       | Run the benchmarks
cargo update      | Update dependencies listed in Cargo.lock
cargo search      | Search registry for crates
cargo publish     | Package and upload this package to the registry
cargo install     | Install a Rust binary. Default location is $HOME/.cargo/bin
cargo uninstall   | Uninstall a Rust binary
cargo help \<command> |  da le informazioni per il singolo comando es. `cargo help build`


### `cargo build --realese`
	DIRECTORY REALESE
	la directory realese costruita con cargo build --realese che crea l'eseguibile definito ma molto piu lenta rispetto a build che crea
	la directory debug molto piu veleoce

### `cargo run`					

	crea l'eseguibile direttamente saltando la parte build

### `cargo -- lib`
	CREARE UNA LIBRERIA: crea una libreria 


### `cargo check`
	CONTROLLO COPILAZIONE: controlla l'esatto complilazione. Si puo anche 										installare mediante il rustpu un language service ..... 


### `cargo check`
cargo fmt							FORMATTARE I SORGENTI: formatta i sorgenti modica i soggetti

### `cargo test`
	lancia i test vengono costruite molto vicini al codice e permette di testare
	pezzi di codice

### `cargo clippy`
	chek piu evoluto per avere consigli sulla costruzioni di rust e deve essere 																	installato con
	rustup update rustup component add clippy

### `cargo modello`
	MODELLO TITOLO: .....											
	Descrizione .......


##P
### `println!`
	stampa una stringa


##R
### `Return`

	return è necessario per il ritorno del valore oppure omettere il ; alla fine della funzione 

### `rustc --version `
	(controlla la versione) rustc compila linka ma viene sostituito da cargo



> Il self è una parola chiave che si usa come primo parametro nei metodi e deve essere **&mut**. 
Rappresenta il this di Java, o Me di Visual Basic


# FILE CREATI QUANDO SI CREA UN  NUOVO PROGETTO CARGO
**cargo.lock**	File rust creato alla prima costruzione del progetto e viene aggiornato solo se esiste una nuova versione con nuove dipendenze tramite il comando rand. L'aggiornamento deve essere automatico.

**cargo.toml** 
Si chiama cargo.toml che contiene al suo interno le sezioni package e depdendecies. Cargo scrive in questo file la versione nella prima costruzione del progetto con tutte le dipendenze che vengono cristallizzate nel file cargo.lock
							

	[package] 
	# ----- nome del progetto, versione del tuo progetto e edizione di Rust
	nome 	 = "hello"
	version  = "0.1.0"
	edition  =	"2021"
	# testo di commento .......

	[dependencies]

Nella sezione [dependencies] vengono insere le libreria che servono al programma
la versione semantic version per la gestione le dipendenze ossia versioni compatibili rispetto ad un'altro;


# ESEMPI InRUST
Esempi di utilizzo di vari codici in rust
## 01) Esempio_01 Struttura dati 
[ESEMPIO MACRO DEBUG](#esempio-ma@push.in.Rustcro-debug)
	.... esempio di struttura dati da inserire nell'intestazione del file
		#[derive(Debug)]
		sruct
		{
		    width: u32,
		    height: u32,
		}

	assegnazione della struttura dati Rectangle alla variabile costante rect1 
	impostazione dei due parametri per il calcolo dell'area ed esempio di attivazione della macro
	debug con la quale viene stampata la riga attivata con i valori mediante il comando dbg!

		 let rect1 = Rectangle {
	    	width: dbg!(30 * scale), //qui dbg stampa anche la riga 30 con il valore limitato a width
	     	height: 50,
	 	};
	In questo caso l'utilizzo della macro a cui passo come riferimento la variabile proprietaria della
	struttura rettanglo stampa a debug l'intera struttura con i valori. Ottimo per il controllo in debug.
		dbg!(&rect1); //utilizzando dbg!(&rect1) utilizzo il riferimento e non ottengo la proprieta



## 02) ESEMPIO_02 
	....todo: da f@push.in.Rustare





# MARKDOWN PER VISUAL STUDIO CODE  - FORMATTAZIONE DATI -
## MARKDOWN
> ISTRUZIONI PER MARKDOWN https://www.markdownguide.org/basic-syntax
> LINKS: https://www.w3schools.io/file/markdown-links/
## TESTO COLORATO
per marcare il testo con il colore utilizzare le virgolette singole 
es. `
faq_testo: testo colorato, 

	`rustc --version `

## TABELLA
Il markdown è un formato di file che ti permette di scrivere in modo semplice e viene utilizzato in  tutti gli IDE che lo supportano compreso README.md di GitHub. 

Per implementare una tabella, usa ------ per delimitare una riga e | per le colonne:
faq tabella: come impostare una colonna, costruire una colonna, costruire una riga, costruire una tabella

	COLONNA	| significato
	-------	| -----------
	RIGA	| 1
	RIGA	| 2



# GIT TUTORIAL MEMO
## LE ISTRUZIONI DI GIT PER LE REPOSITORY
NOTE: per utilizzare git occorre prima creare una repository nella cartella scelta per l'archiviazione
ed eseguire questi comandi.

### git new repository
	per creare una nuova repository locale come primo passo, posizionarsi con il cmd nella cartella di base, ad
	aprire  la shell dei comandi posizionarsi nella cartella in cui creare la repository LOCALE come ad es.:
		c:\CASA\PROGRAMMI\RUST_PROGETTI\

		scrivere i seguenti comandi nella shell:
			a) git init						= crea una nuova repository locale
			b) git add <file oppure .>		= scrivere il nome del file da aggiungere nel commit oppure usare il 									punto (. il punto) per utti i file;
			c)git commit -m "first commit"  = la prima commit con il messaggio - m ....
			d)git branch -M main			= cra una repository main
			e)git remote add origin https://github.com/icivixwalter/RUST_PROGETTI.git
											= aggiunge al memoto la cartella corrente 
			f)git push -u origin main		= fa il push sulla repository main della nuvola.


	il tutorial  si trova qui: [https://devconnected.com/how-to-push-git-branch-to-remote/]


### git risoluzione dei problemi - Troubleshooting

Risoluzione dei problemi
In alcuni casi, è possibile che si verifichino errori durante il tentativo di eseguire il push di un ramo Git su un telecomando. [https://devconnected.com/how-to-push-git-branch-to-remote/]

//todo da finire.....





```
