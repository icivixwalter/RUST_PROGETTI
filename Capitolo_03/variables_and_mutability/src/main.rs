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

    

        
            println!(" CAP. 3.2 TIPI DI DATI FLOAT : ci sono due tipi primitivi di tipo float \n
                        1) il tipo predifinito rappresentata dalla x f64 \n
                        2) il tipo primitivio a dimensione di 32 bit è stato 
                        impostato nella variabile \n
                        x f64 tipo definito = {} 
                        y f32               = {}", x,y);
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
             resto - let remainder = 43 % 5 :                       {}"
             
             ,sum, difference, product, quotient, floored, remainder
             
            );
            println!(" 
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
            println!(" \n
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
            let f: bool = false; // with explicit type annotation:  {}", t,f );

            println!(" 
            FINE
            ==========================================================================\n");

       

        }

}