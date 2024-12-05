//use std::f64::consts::PI;

fn main() {
    //sup_sept(4);
    //plus_conditions(4);
    //if_avec_let();
    //loop_function();
    //retour_valeur_loop();
    //affiche_triangle(10);
    //affiche_carre(10);
    //affiche_cercle();
    //while_loop();
    //other_while_loop();
    //for_loop();
    other_for_loop();
}

fn _sup_sept(nombre: i32){
    if nombre < 5{
        return println!("La condition est vérifiée.");
    }
    return println!("La condition n'est pas vérifiée.");
}

fn _plus_conditions(nombre: i32){
    if nombre % 4 == 0{
        println!("Le nombre est divisible par 4");
    }else if nombre % 3 == 0{
        println!("Le nombre est divisible par 3");
    }else if nombre % 2 == 0{
        println!("Le nombre est divisible par 2");
    }else{
        println!("Le nombre n'est pas divisible par 4, 3 ou 2");
    }
}

fn _if_avec_let(){
    let condition = true;
    let nombre = if condition { 5 } else { 6 };

    println!("La valeur du nombre est : {}", nombre);
}

fn _loop_function(){
    let mut compteur = 0;

    'increment: loop{
        println!("compteur = {}", compteur);
        let mut restant = 10;

        loop{
            println!("restant = {}", restant);
            if restant == 9{
                break;
            }

            if compteur == 2{
                break 'increment;
            }

            restant -= 1;
        }

        compteur += 1;
    }

    println!("Fin du compteur = {}", compteur)
}

fn _retour_valeur_loop(){
    let mut compteur = 0;

    let resultat = loop{
        compteur += 1;

        if compteur == 10{
            break compteur*2;
        }
    };
    println!("Le resultat est {}", resultat);
}

fn _affiche_triangle(nombre: usize){
    for i in 0..=nombre{
        println!("{}", "*".repeat(nombre - i));
    }
}

fn _affiche_carre(nombre: usize){
    for _i in 0..=nombre{
        println!("{}", "*".repeat(nombre*3));
    }
}

fn _affiche_cercle(){
    let radius = 10;
    let scale = 1.0;
    let _diameter = radius * 2;

    for y in -radius..=radius{
        for x in -radius..=radius{
            let distance = ((x as f64 / scale).powi(2) + (y as f64).powi(2)).sqrt();

            if distance <= radius as f64{
                print!("*");
            }else{
                print!(" ")
            }
        }
        println!();
    }
}

fn _while_loop(){
    let mut nombre = 3;

    while nombre != 0 {
        println!("{}", nombre);

        nombre -= 1;
    }
    println!("DÉCOLLAGE !!!");
}

fn _other_while_loop(){
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < 5{
        println!("La valeur est : {}", a[indice]);

        indice += 1;
    }
}

fn _for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("La valeur est : {}", element);
    }
}

fn other_for_loop(){
    for nombre in (1..4).rev(){
        println!("{} !", nombre);
    }

    println!("DÉCOLLAGE !!!");
}