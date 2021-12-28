// region: strutture
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// endregion: strutture




fn main() {

    


    //modificato con let mut
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

   
    user1.email = String::from("anotheremail@example.com"); //per modificare l'email è stata inserito mut

    println!("creata una struttura USER 1
            per il suo utilizzo occore utilizzare la variabile 
            oggetto che punta ad un suo campo:
            - user1.email            : {}
            - user1.username         : {}
            - user1.active           : {}
            - user1.sign_in_count    : {}
            ",&user1.email, &user1.username, &user1.active, &user1.sign_in_count );

            let user2 = User {
                active: user1.active,
                username: user1.username,
                email: String::from("another@example.com"),
                sign_in_count: user1.sign_in_count,
            };
    
            println!("II ESEMPIO DI USER 2 - creata una struttura 
            per il suo utilizzo occore utilizzare la variabile 
            oggetto che punta ad un suo campo:
            - user2.email            : {}
            - user2.username         : {}
            - user2.active           : {}
            - user2.sign_in_count    : {}
            ",&user2.email, &user2.username, &user2.active, &user2.sign_in_count );

    



}//main *** fine ***



//le funzioni:
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
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




