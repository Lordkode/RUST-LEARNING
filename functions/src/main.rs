fn main() {
    println!("Hello, world!");

    une_autre_fonction();
    fonction_a_arguments(5);
    fonction_a_arguments(10);
    afficher_mesure_avec_unite(12, 'h');
    let x = cinq();
    println!("La valeur de la fonction cinq est : {}", x);

    let y = plus_un(5);
    println!("La valeur de la fonction plus_un est : {}", y);
}

fn une_autre_fonction() {
    println!("Une autre fonction.")
}

fn fonction_a_arguments(x: i32) {
    println!("La valeur de x est : {}", x);
}

fn afficher_mesure_avec_unite(valeur: i32, unite: char) {
    println!("La valeur est : {}{}", valeur, unite);
}

//Une fonction qui retourne une valeur
fn cinq() -> i32{
    5
}

fn plus_un(x: i32) -> i32{
    x + 1;
}
