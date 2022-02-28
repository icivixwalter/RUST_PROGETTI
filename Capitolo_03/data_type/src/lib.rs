pub fn run() {
    // [1] ZERO-COST ABSTRATION: astrazione a costo zero con il massimo della performance
    /* Invece di usare la printf come in c con %d, %s, %l...,
    su Rust basta qualificare con le graffe gli argomenti da stampare
    ci pensa il compilatore a recuperare il tipo */
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // al posto di %d ecc. si usano  le {}
    println!("DATA TIPE: stampo i numeri f64 e f32: {} {}", x, y);

     println!("\n\n\
    ---------------------------------------------------------------------------
                    FINE Capitolo_03 - data_type - \n\n");

}
