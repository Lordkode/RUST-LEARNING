fn main() {
    let _supposition: u32 = "42".parse().expect("Ce n'est pas un nombre !");
    println!("La supposition est : {}", _supposition);

    //Les opérateurs
    //SOMME D'ENTIERS
    let somme = 5 + 10;
    println!("La somme de 5 et 10 est égale à : {}", somme);

    //SOUSTRACTION
    let soustraction = 95.5 - 4.3;
    println!("La soustraction de 4.3 dans 95.5 donne : {:.3}", soustraction);

    //MULTIPLICATION
    let produit = 4 * 30;
    println!("Le produit de 4 et 30 est égale à : {:.3}", produit);

    //DIVISION
    let quotient = 56.7 / 32.2;
    println!("Le quotient de 56.7 et 32.2 est de : {:.3}", quotient);

    //MODULO
    let reste = 43 % 5;
    println!("Le reste de la division de 43 par 5 est égale à : {}", reste);


    //Type chaîne de caractères
    let c = "z";
    let z = "ℤ";
    let chat_aux_yeux_de_coeur = "😻";

    println!("Affichage de c : {}", c);
    println!("Affiche de z: {}", z);
    println!("Affichage du chat : {}", chat_aux_yeux_de_coeur);

    //Les types composés qui sont de deux (les tuples et les tableaux)
    //Les tuples
    let tup = (i32, f64, u8) = (500, 6.4, 1);
}
