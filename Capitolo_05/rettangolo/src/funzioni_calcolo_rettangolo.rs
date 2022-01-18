
#[allow(dead_code)]
struct RectangleNw { width:u32, height:u32, } 


//per utilizzarla la struttura del rettangolo usare impl
impl RectangleNw {
    // metodo statico non richiede l'istanza ma basta il nome del tipo es. Rectangle::square(20)
    // in java era public static Rectangle square (int size)
    #[allow(dead_code)]
    fn square(size: u32) -> RectangleNw {
        return RectangleNw {//utilizzo Rectangle per restituire un oggetto Rectangle che rappresenta un quadrato
            width: size,    //in quanto ricevento un solo parametro u32 si valorizza la larghezza e l'altezza
            height: size,   //CON LO STESSO VALORE lxl = quadrato.
        };
    }
}


pub fn calcolo_rettangolo(){
        
    let _rect_1 = RectangleNw {
        width: 30,
        height: 50,
    };
  

}




//pubblica funzione calcolo area
pub fn area(width: u32, height: u32) -> u32 {
    width * height
}
