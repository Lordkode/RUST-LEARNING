use std::io;

fn main() {
    println!("Calculatrice");
    println!("Entrez la valeur 1 :");

    // Entrée valeur 1
    let mut valeur1 = String::new();
    io::stdin()
        .read_line(&mut valeur1)
        .expect("Erreur lors de la lecture de l'entrée");

    let valeur1: i32 = match valeur1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide.");
            return;
        }
    };

    // Affichage valeur 1
    println!("La valeur 1 est : {}", valeur1);

    // Entrée valeur 2
    println!("Entrez la valeur 2 :");
    let mut valeur2 = String::new();
    io::stdin()
        .read_line(&mut valeur2)
        .expect("Erreur lors de la lecture de l'entrée");

    let valeur2: i32 = match valeur2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez entrer un nombre valide.");
            return;
        }
    };

    // Affichage valeur 2
    println!("La valeur 2 est : {}", valeur2);

    // Choix de l'opération
    println!("Choisissez l'opérateur svp : ");
    println!("1 - Addition");
    println!("2 - Soustraction");
    println!("3 - Division");
    println!("4 - Multiplication");
    println!("5 - Modulo");

    let mut choix = String::new();
    io::stdin()
        .read_line(&mut choix)
        .expect("Erreur lors de la lecture de l'entrée");

    let choix = choix.trim(); // On supprime les espaces ou les sauts de ligne

    let response = match choix {
        "1" => addition(valeur1, valeur2),
        "2" => soustraction(valeur1, valeur2),
        "3" => division(valeur1, valeur2),
        "4" => multiplication(valeur1, valeur2),
        "5" => modulo(valeur1, valeur2),
        _ => {
            println!("Choix invalide !");
            return;
        }
    };

    println!("Le résultat est : {}", response);
}

// Fonction pour faire l'addition
fn addition(x: i32, y: i32) -> i32 {
    x + y
}

// Fonction pour faire la soustraction
fn soustraction(x: i32, y: i32) -> i32 {
    x - y
}

// Fonction pour faire la division
fn division(x: i32, y: i32) -> i32 {
    if y == 0 {
        println!("Erreur : Division par zéro !");
        return 0;
    }
    x / y
}

// Fonction pour faire la multiplication
fn multiplication(x: i32, y: i32) -> i32 {
    x * y
}

// Fonction pour le reste de la division
fn modulo(x: i32, y: i32) -> i32 {
    if y == 0 {
        println!("Erreur : Modulo par zéro !");
        return 0;
    }
    x % y
}
