//TODO: NON FUNZIONA ???
// // region: test
// #[cfg(test)]
// mod tests {
   

// //LIBRERIA FRONT OF HOUSE – creare una libreria con cargo new --lib new front_of_house 
// #[test]
// mod front_of_house { //modulo padre

// 	//modulo hosting = è pubblico
// 	pub mod hosting { 
// 		pub fn add_to_waitlist() {//adesso la funzione è pubblica e si puo
// 								  //accedere ai percorsi relativi ed assoluti
// 		} 
// 	} 
// } 
// //anche  eat_at_restaurant è pubblica e fratello di  add_to_waitlist perche appartengono come moduli figlio di  front_of_house
// pub fn eat_at_restaurant() { // Absolute path --- puoi accedere ai percorsi perche pub 	crate::front_of_house::hosting::add_to_waitlist(); // Relative path 	front_of_house::hosting::add_to_waitlist(); 
// } 
// //Listing 7-7: Adding the pub keyword to mod hosting and fn add_to_waitlist lets us call the function from eat_at_restaurant 


// }// endregion: test
