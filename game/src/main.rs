use rand::Rng; 
use std::cmp::Ordering; 
use std::io; 

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1..=100);

    println!("Le nombre secret est : {}", nombre_secret);

    loop {
        println!("Veuillez entrer un nombre.");

        // Lire l'entrée utilisateur
        let mut supposition = String::new();
        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        // Convertir l'entrée en un entier (u32)
        let supposition: u32 = match supposition.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue; // Recommence la boucle si l'entrée est invalide
            }
        };

        // Comparer la supposition avec le nombre secret
        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break; // Sort de la boucle si l'utilisateur a deviné correctement
            }
        }
    }
}
