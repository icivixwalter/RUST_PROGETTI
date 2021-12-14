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
    }

    


}


 




  