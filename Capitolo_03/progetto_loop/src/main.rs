//use std::fmt::Display;
use std::io::stdin;



fn main() {
    

    
        //3.5.04.01 Repeating Code with loop -Codice a ripetizione con loop
        //----------------------------------------------------------------------------//
        /* Il ciclo esterno ha l'etichetta 'counting_up e conterà da 0 a 2. 
            Il ciclo interno senza etichetta conta da 10 a 9. La prima interruzione
            che non specifica un'etichetta uscirà solo dal ciclo interno. 
            La pausa 'counting_up; l'istruzione uscirà dal ciclo esterno.
         */        
        
        let mut count = 0;
        
        //loop 01 esterno + label 'counting_up = conta da 0 a 2
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;

                //loop 2 interno senza etichetta conta da 10 a 9
                loop {
                    println!("remaining = {}", remaining);
                    if remaining == 9 {
                        break;
                    }
                    if count == 2 {
                        println!("\n 
                            --------------------------------
                            uscita dal ciclo esterno 1 loop 
                            ................................");

                        break 'counting_up;  //pausa ed esce dal ciclo esterno
                    }
                    remaining -= 1;
                }

                count += 1;

                println!("FINE CONTEGGIO: End count = {}\n\n", count);

            }

           
                     


           
                //3.5.04.02 Returning Values from Loops - Restituzione di valori dai loop
                //----------------------------------------------------------------------------//
                /*Uno degli usi di un ciclo consiste nel riprovare un'operazione che 
                si sa potrebbe non riuscire, come controllare se un thread ha 
                completato il proprio lavoro. Tuttavia, potrebbe essere 
                necessario passare il risultato di tale operazione al resto del codice. 
                Per fare ciò, puoi aggiungere il valore che desideri venga
                restituito dopo l'espressione break che utilizzi per interrompere
                il ciclo; quel valore verrà restituito fuori dal ciclo in modo 
                che tu possa usarlo, come mostrato qui:

                    */    



                    {
                        println!("\n\n
                        -------------------------------------------------------
                        II ESEMPIO DI LOOP
                        -------------------------------------------------------
                        3.5.04.02 Returning Values from Loops - Restituzione di valori dai loop
                        \n");

                        let mut counter = 0; //contatore inizializzato a 0
                        
                        let result = loop {  //result contiene il valore del ciclo
                            counter += 1;  //ad ogni ciclo +1	
                            println!("ciclo loop - counter +1 = {}",counter);

                            if counter == 10 { //controllo se il contatore = 10
                            break counter * 2; // se il contatore = 10 usiamo break per counter * 2
                            } 
                        }; //il punto e virgola per terminare l’istruzione ed assegnare a 		   result
                        
                        println!("
                        II ESEMPIO: Restituzione di valori dai loop
                        The result is {}", result); //fine stampiamo
                    }

                //* FINE  */
                //----------------------------------------------------------------------------//

                /* 3.5.4.02 Conditional Loops with while - Cicli condizionali con while
                  Questo tipo di ciclo può essere implementato utilizzando una combinazione 
                  di ciclo, if, else e break
                    chiamato ciclo while.
                */
                //----------------------------------------------------------------------------//
    
                {

                    println!("\n\n
                    -------------------------------------------------------
                                CICLO WHILE
                    -------------------------------------------------------
                    3.5.4.02 Conditional Loops with while - Cicli condizionali con while
                    \n");

                    let mut number = 3; 
                    //finche è diverso da zero continua la stama
                    while number != 0 { 
                        println!("CONTEGGIO ALL'INDIETRO {}!", number); number -= 1; //=-1 per conteggio all'indietro
                    } 
                    println!("\n
                            LIFTOFF!!! \n\n"
                    ); 
                    



                }
                //* FINE  */
                //----------------------------------------------------------------------------//



                /* 3.5.4.03 Looping Through a Collection with for Passare attraverso una collection con for
                    È possibile utilizzare il costrutto while per eseguire il ciclo sugli
                    elementi di una COLLEZIONE, ad esempio un array.
                */
                //----------------------------------------------------------------------------//
    
                {
                    println!("\n\n
                    -------------------------------------------------------
                            I° ESEMPIO   
                            CICLO WHILE SU UN ARRAY DI 5 ELEMENTI
                    -------------------------------------------------------
                    3.5.4.03 Looping Through a Collection with for Passare attraverso una collection con for 
                        questo I° esempio utilizza un ciclo while per stampare l'array, ma essendo stato 
                        costruito con l'indice < 5 ossia con 4 elementi non ci sono pericoli per il fuori 
                        indice. Il pericolo è dato dal fatto che il while potrebbe andare fuori indice se 
                        viene cambiato il valore di controllo da < 5 ad esempio a < 7 ???? PANIC ????
                    \n");

                    let a = [10, 20, 30, 40, 50]; //imposto l’array di 5 elementi indice a 0-4
                    let mut index = 0; // imposto e reset indice a zero
                    
                    while index < 5 { //filtro fino a 4
                        //stampo la posizione dell’indice
                        println!("VALORE DELL'INDICE = the value is: index: {}", a[index]); 
                        index += 1; //incremento l’indice
                    }  

                    print!("\n\n");

                }

                //* FINE  */
                //----------------------------------------------------------------------------//


            
                /* 3.5.4.03 Looping Through a Collection with for Passare attraverso una collection con for
                    II ° ESEMPIO utilizzando un ciclo for al posto di while in quanto piu sicuro
                    per gli elementi di un array ed il controllo fuori indice.
                */
                //----------------------------------------------------------------------------//
    
                {
                    println!("\n\n
                    -------------------------------------------------------
                            II° ESEMPIO    
                            CICLO FOR SU UN ARRAY DI 5 ELEMENTI
                    -------------------------------------------------------
                    Per un maggiore controllo dell'indice di una arry è megio utilizzare un 
                    ciclo for al posto del while in quanto consente di evitare errori dell'indice.
                    Questo è un ciclo for semplificato SENZA RANGE ma semplicemente con l'assegnazione
                    del numero dell'array alla variabile a_for1.
                    
                    \n");
             
                        let a_for1 = [10, 20, 30, 40, 50]; 
                        for element in a_for1 { 
                            println!("FOR SEMPLIFICATO the value is: a_for1 {}", element); 
                        }
                        
                        print!("\n\n");

                }

                //* FINE  */
                //----------------------------------------------------------------------------//


            
                /* 3.5.4.03 Looping Through a Collection with for Passare attraverso una collection con for
                    III ° ESEMPIO 
                    ciclo for con range invertito che consente di creare codice sicuro per la 
                    gestione degli array
                */
                //----------------------------------------------------------------------------//
    
                {
                    println!("\n\n
                    -------------------------------------------------------
                            III° ESEMPIO    
                            CICLO FOR SU UN ARRAY DI 5 ELEMENTI
                            A RANGE INVERTITO
                    Questo ciclo for utilizza un range di elementi tramite
                    il comando .rev che permette sia di evitare il rischio 
                    del fuori indice che il conteggio contrario :
                      ...   for number in (1..4).rev()   ....
                      Questa istruzione è stata modificata con questa versione
                            for a_for2 in (min..max).rev()
                      la quale permette il ricalco dell'indice massimo dell'array
                      mediante  array.len(); ed assegnato alla variabile max; 
                      mentre l'indice base è sempre zero:  min = 0. 
                      In questo modo posso ridurre o aumentare i componenti 
                      dell'array senza rischio  del fuori indice  unito al 
                      perfetto controllo del ciclo for al posto del while.
                    ------------------------------------------------------- \n

               ");  
               
               /*
               CICLO FOR  AD INTERVALLO INVERTITO, DA 1 a 4 
                 Si prevede il conto alla rovescia tramite .rev.
                 vecchia procedura questa:
                     for number in (1..4).rev() {.....
                MODIFICA:
                 Tale ciclo è stato modifcato costruendo un ciclo sempre avendo
                 come base  un array i cui elementi possono essere aumentati
                 o diminuiti calcolando in modo automatico l'indice min e max
                con la seguente istruzione: 
                    for a_for2 in (min..max).rev() { ....
                In questo modo il ciclo è esente da errori fuori indice in quanto
                utilizza il costrutto FOR molto piu sicuro del while ed il calcolo
                automatico degli indici dell'array.

                 */
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


            }

            //* FINE  */
            //----------------------------------------------------------------------------//

            // PERMETTE DI RIMANERE VISIBILE LA SHELL 
            //---------------------------------------------------------------------------//
            // utilizzare questa libreria: use std::io::stdin;
                let mut s= String::new();
                println!("premi invio per uscire!");
                stdin().read_line(&mut s).expect("Did not enter a correct string");  
                
            //---------------------------------------------------------------------------//


}//main fine
