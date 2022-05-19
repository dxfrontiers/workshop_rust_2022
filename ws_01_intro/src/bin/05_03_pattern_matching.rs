use std::collections::HashMap;

enum Gender {
    Male,
    Female,
    Diverse,
}

struct Address {
    street: Option<String>,
    city: String,
}

struct Person {
    first_name: String,
    last_name: String,
    age: i32,
    gender: Gender,
    address: Address,
}

fn main() {

    let person = Person {
        first_name: "Kim".to_string(),
        last_name: "Chi".to_string(),
        age: 42,
        gender: Gender::Female,
        address: Address {
            street: Some("Elm Street 42".to_string()),
            city: "Springwood".to_string()
        },
    };

    match person {
        Person { gender: Gender::Male, .. } => println!("Is male."),
        Person { age, ..} if age > 9000 => println!("It's over 9000!"),
        Person { gender: Gender::Female, address: ref addr @ Address { street: Some(_), .. }, .. } =>
            println!("Female person having street value set to {}", addr.city),
        _ => println!("Nothing else matters... matches!"),
    }

}

fn match_first_name(person: &Person) {
    //match person {
    //    Person { first_name: "Hans", .. } => println!("It's Hans!"),
    //    _ => println!("Definitively not Hans"),
    //}
    todo!()
}
