#[allow(dead_code)]  //serve per togliere i warnings sul codice non utilizzato
pub mod front_of_house {
    pub mod hosting {
        /// AGGIUNGI ALLA LISTA DI ATTESA
        pub fn add_to_waitlist() {
            println!("AGGIUNGI ALLA LISTA DI ATTESA ");
        }

        /// INVITA A SEDERE A TAVOLA
        pub fn seat_at_table() {}
    }

    pub mod serving {
        /// PRENDI UN ORDINE
        pub fn take_order() {
            println!("PRENDI UN ORDINE");
        }

        /// SERVI UN ORDINE
        pub fn serve_order() {}

        ///PRENDI IL PAGAMENTO
        pub fn take_payment() {}
    }
}

pub fn run() {
    front_of_house::hosting::add_to_waitlist();
}


//creato una liberia con un test
#[cfg(test)]    //annotazione per fare riconoscere come test
mod tests {
    // modulo tst
    #[test] // annota la singola funzione come test
    fn it_works() {
        super::run();
    }
}
