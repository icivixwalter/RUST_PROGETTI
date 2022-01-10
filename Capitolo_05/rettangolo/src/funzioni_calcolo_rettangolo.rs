
//nome precedente del file: funzioni_calcolo_rettangolo
struct Rectangle { width: u32, height: u32, } 
pub fn calcolo_rettangolo(){
        
    let rect_1 = Rectangle {
        width: 30,
        height: 50,
    };

  

}




//pubblica funzione calcolo area
pub fn area(width: u32, height: u32) -> u32 {
    width * height
}
