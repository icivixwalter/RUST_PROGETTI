
use std::io;        //libreria di imput
use std::io::stdin;
fn main() {
    let mut x = 5; //modificata con let mut per renderla da Static fina a variabile.
    println!("The value of x is: {}", x);

    x = 6; //errore hai assegnato un nuovo valore ad una variabili immutabile impostrare la variabile x con (let mut x = 5;)
    println!("The value of x is: {}", x);

    //SECONDA FASE OMBREGGIAMENTO
    let x = x + 1;

    {
        let x = x * 2;
        println!("NUOVO VALORE: The value of x in the inner scope is: {}", x);

        // let spaces = " "; //con mut da errore non usare mut con l’ombreggiatura.
        // spaces = spaces.len();

        // 3.2. Data Types - Floating-Point Types – Tipo Float - i tipi primitivi decimali
        // I tipi a virgola mobile di Rust sono f32 e f64
        let x = 2.0; // f64 è il dipo primitivo predefinito f64

        let y: f32 = 3.0; // f32 - ridefinito a f32

        println!(
            " CAP. 3.2 TIPI DI DATI FLOAT : ci sono due tipi primitivi di tipo float \n
                        1) il tipo predifinito rappresentata dalla x f64 \n
                        2) il tipo primitivio a dimensione di 32 bit è stato 
                        impostato nella variabile \n
                        x f64 tipo definito = {} 
                        y f32               = {}",
            x, y
        );
    }

    {
        /* 3.2. Data Types - Numeric Operations
         *  Rust supporta le operazioni matematiche di base che
         * ti aspetteresti per tutti i tipi di numeri:
         * addizione, sottrazione, moltiplicazione, divisione e resto.
         *   - La divisione intera viene arrotondata per difetto all'intero più vicino.
         */

        // addition let sum {}
        let sum = 5 + 10;

        // subtraction let difference: {}
        let difference = 95.5 - 4.3;

        // multiplication let product  {}
        let product = 4 * 30;

        // division - let quotient: {}
        let quotient = 56.7 / 32.2;
        //let floored arrotondamento all'intero inferiore: {}
        let floored = 2 / 3; // Results in 0

        // resto - let remainder : {}
        let remainder = 43 % 5;

        //stampo
        println!(" \n
            INIZIO
            ==========================================================================\n
            CAP. 3.2 TIPI DI DATI FLOAT : Operazioni numeriche - Numeric Operations 
                           
            Rust supporta le operazioni matematiche di base che ti aspetteresti per tutti
             i tipi di numeri:
                addizione,  
                sottrazione, 
                moltiplicazione, 
                divisione e resto. 
            La divisione intera viene arrotondata per difetto all'intero più vicino. 
             Il codice seguente mostra come utilizzeresti ogni operazione numerica in una istruzione let:
             
             addition let sum = 5 + 10:                             {}
             subtraction let difference = 95.5 - 4.3:               {}
             multiplication let product = 4 * 30:                   {}
             division - let quotient 56.7 / 32.2 :                  {} 
             divisione - floored arrotondamento intero inferiore
             let floored = 2 / 3:                                   {}    
             resto - let remainder = 43 % 5 :                       {}{}",
             sum, difference, product, quotient, floored, remainder, " 
             FINE
             ==========================================================================\n
 
             \n");
    }

    {
        /* 3.2. Data Types - The Boolean Type – Tipi bolean
         *
         * Come nella maggior parte degli altri linguaggi di programmazione,
         * un tipo booleano in Rust ha due possibili valori: vero e falso.
         * I booleani hanno una dimensione di un byte.
         * Il tipo booleano in Rust viene specificato utilizzando bool.
         */

        let t = true;
        let f: bool = false; // with explicit type annotation

        //stampo
        println!(
            " \n
            INIZIO
            ==========================================================================
            3.2. Data Types - The Boolean Type – Tipi bolean
             
             Come nella maggior parte degli altri linguaggi di programmazione, 
             un tipo booleano in Rust ha due possibili valori: vero e falso. 
             I booleani hanno una dimensione di un byte. 
             Il tipo booleano in Rust viene specificato utilizzando bool.

            CAP. 3.2. Data Types - The Boolean Type – Tipi bolean 
            la variabili t a cui assegno true
            let t = true:                                           {}

            la variabile f  con definizione specifica :bool                                        
            let f: bool = false; // with explicit type annotation:  {}",
            t, f
        );

        println!(
            " 
            FINE
            ==========================================================================\n"
        );
    }

    {
        //IMPOSTO
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = "🤳 ✔ 👀 🚠 🎨 👨‍🦰";

        //stampo TIPO CARATTERI - THE CHARACTER
        println!(
            " \n
         INIZIO         3.2.02 The Character Type - Il tipo di carattere
         ==========================================================================\n 
         letterali char sono specificati con virgolette singole

         STAMPO LE VARIABII 
         c =                            {}
         z=                             {} 
         heart_eyed_cat (figura) =      {}",
            c, z, heart_eyed_cat
        );

        println!(
            " \n
         *** FINE ***   3.2.02 The Character Type - Il tipo di carattere
         ==========================================================================\n "
        );
    }


    /* 3.3. Data Types - The Boolean Type – Tipi bolean*/
        
    {
        //definizione della variabile tupla con 3 tipi diversi opzionali e assegnazione
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        //es. destrutturazione della tupla

        let (x, y, z) = tup; //pattern matching che destruttura la tupla e assegna i singoli valori a x,y,z

        println!("The value of y is: {}", y);
        //stampo
        println!(
            " \n
          stampo la tupla destrutturata con il matching
          tupla orignaria costruita con:
          let tup: (i32, f64, u8) = (500, 6.4, 1); 
          risultato destrutturato = x: {} y:{}  z:{}",
            x, y, z
        );

        //indici tupla
        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;

        //stampo
        println!(
        "\n    
        Stampo la tupla con gli indici (gli indici in rust partono da zero):
        let five_hundred = tup.0    = {}  
        six_point_four = tup.1      = {}
        one = tup.2                 = {}",
            five_hundred, six_point_four, one

        );
    }

    //ARRAY CON ELEMENTI FISSI
    {
        //array quando gli elementi sono fissi
            let a_Interi = [1, 2, 3, 4, 5]; 	//array di numeri interi
            //ARRAY per i mesi (numeri di elementi fissi)
            
            let a_months = ["January", "February", "March", "April", "May", "June", "July", 			
                            "August", "September", "October", "November", "December"]; 
            let a_months_array=a_months[1];     //array assegno il primo ed il 4
            let a_months_array4= a_months[4];

            let a_i32: [i32; 5] = [1, 2, 3, 4, 5];   //definizione del tipo di array i32 di 5 elementi 
            let a_definitivi_i32_03= a_i32[3];                  
                                            //array definito i32 per 5 elementi assegnati alla variabile a_definitivi


            let a_inzializzato = [3; 5]; //inizializzo l’array valore iniziale 3 lunghezza 5 (5 elementi di 3)
            let a_inzializzato_Pos3=a_inzializzato[3];  //assegno all'elemento 3   
         
            //accesso agli elementi dell'array (ATTENZIONE l'array viene allocato nello stack)
            let first = a_Interi[0];
            let second = a_Interi[1];

            //stampo TODO: ATTENZIONE DA ERRORE
        println!(
            "\n    
            stampo array  sopra definiti e riassegnati 
            alle variabili di controllo:
            a_Interi                            : {}
            a_months                            : {}
            a_months indice 4                   : {}
            a_definitivi_i32_03                 : {}
            a_inzializzato_Pos3                 : {}
            let first                           :{}
            let second                          :{}
            ",
            a_Interi[0],
            a_months_array, 
            a_months_array4,
            a_definitivi_i32_03,
            a_inzializzato_Pos3, first, second   
        ); //errore a_months
            
            
           /*
            stampo array :
            a_Interi                        : {0}
            a_months                        : {0}
            a_i32                           : {0}
            a_definitivi                    : {0}
            a_inzializzato                  : {0}
            first                           : {0}",
            a_months[0],
            a_Interi[0],
            a_i32[0],
            a_definitivi[1],
            a_inzializzato[0],
                       );
            */

    }

    //ESERCIZIO ACCESSO AGLI ELEMENTI DI UN ARRAY
    {
        /* */
            //CREO UN ARRAY di elementi interi 
            //creo un array (va nello stack ) di 5 elementi interi
            let a = [1, 2, 3, 4, 5];  
            /*ATTENZIONE L'ACCESSO ad un indice non valido 
            provoca l'uscita dal programma senza entrare in una memoria non valida questo è  la protezione di rusta
            che non permette in questa caso di andare a println rispetto ad altri linguaggi di basso livello che permettono
            di continuare accedendo alla memoria non valida e mandando in crash il pc.*/
            
            //con lo standard io leggo un imput da tastiera + controllo errori
            //-------------------------------------------------------------------------//
                println!("Please enter an array index."); 
                let mut index = String::new(); c
                io::stdin() 
                    .read_line(&mut index) 
                    .expect("ERRORE FALLITA LA LETTURA DA LINERA DI COMANDO:Failed to read line"); 
            //-------------------------------------------------------------------------//
    

        
       
        
            let index: usize = index
                .trim()
                .parse()
                .expect("\n\n ERRORE FUORI INDICE: Index entered was not a number");
        
            let element = a[index];
        
                  println!( 
            "STAMPO VALORE INDICE : The value of the element at INDICE, index {} VALORE is: {} \n\n", 
            index, element 
            ); 
        

    }

//---------------------------------------------------------------------------//
// utilizzare questa libreria: 
//use::std::io;
//use std::io::stdin;

let mut s= String::new();
println!("\n\n premi invio per uscire!");
stdin().read_line(&mut s).expect("Did not enter a correct string");  

//---------------------------------------------------------------------------//


}
