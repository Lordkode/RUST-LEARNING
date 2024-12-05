fn main() {
    //Variable mutable (avec le mot clé `mut`)
    let mut x = 5;
    println!("La valeur de x est : {}", x);

    x = 6;
    println!("La valeur de x est : {}", x);

    //Cas d'une variable immuables
    let y = 64;
    println!("La valeur de y est : {}", y);

    //À ce niveau il faut remarquer que
    //la variable ne peux pas changer de valeur

    /*y = 48;*/

    //Ici il y aura donc une erreur lors de la compilation

    /*println!("La valeur de y est : {}", y);*/

    //Les constantes
    const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;
    println!(
        "Trois heures en secondes vaut : {}",
        TROIS_HEURES_EN_SECONDES
    );

    //Le masquage de variable
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("La valeur de z est la portée interne est : {}", z);
    }

    println!("La valeur de z est : {}", z);
}
