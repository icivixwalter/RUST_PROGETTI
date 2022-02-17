#[allow(dead_code)]  //serve per togliere i warnings sul codice non utilizzato
mod front_of_house {
    mod hosting {
        /// AGGIUNGI ALLA LISTA DI ATTESA
        fn add_to_waitlist() {}
        /// INVITA A SEDERE A TAVOLA
        fn seat_at_table() {}
    }

    mod serving {
        /// PRENDI UN ORDINE
        fn take_order() {}

        /// SERVI UN ORDINE
        fn serve_order() {}

        ///PRENDI IL PAGAMENTO
        fn take_payment() {}
    }
}



//creato una liberia con un test
#[cfg(test)]    //annotazione per fare riconoscere come test
mod tests { // modulo tst
    #[test] // annota la singola funzione come test
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4); // se result == 4 non da errori altrimenti PANIC
    }
}
