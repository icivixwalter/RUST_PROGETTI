use std::fmt::Display;

// region: strutture
pub struct User { //la struttura e gli elementi devono essere pub altrimenti sono considerati privati.
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}
// endregion: strutture
//restituisce la stringa che rappresenta la struttura in Java è il metodo toString
impl Display for User{//Display è una intefaccia (trait)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {// todo da finire manca result
        // Uso la macro write per stampare la rappresentazione stringa della struct User
        return write!(f, "User [
            active: {}, 
            username: {}, 
            email: {}, 
            sign_in_count: {}
        ])", self.active, self.username, self.email, self.sign_in_count);
    }
}
//endregion: strutture

// metodi per la struct User (come i metodi di una classe Java)
impl User {
    pub fn setEmail(&mut self, email :&str){// pub fn = metodo pubblico
        self.email = email.to_string();
    }
}

//      tutte le funzioni
//------------------------------------------------------------------------------//

//le funzioni:
fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}

//LA FUNZIONE BUILD USER MODIFICATA
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
