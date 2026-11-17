use std::{env, println};
use postgres::{Client, NoTls};
mod kutya;

fn osszeadas(elso_szam: i32, masodik_szam: i32) -> i32{
    elso_szam + masodik_szam // Rustban nem kell return egy sor esetén!
}

fn main() {
    println!("Szia, buzi!");
    println!("Összeadás eredménye: {}\n", osszeadas(1, 2));

    let mut bloki = kutya::Kutya::new(String::from("Blöki"), 3);
    println!("{}", bloki.get_nev());
    bloki.set_nev(String::from("Frakk"));
    println!("{}", bloki.get_nev());

    // Postgres

    let mut client = Client::connect("host=IP user=USERNAME", NoTls);
}

// Unit tesztelés
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn osszeadas_teszt(){
        assert_eq!(osszeadas(1,2), 3);
    }
}