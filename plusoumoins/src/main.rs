use rand::prelude::*;
use std::io::{BufRead, Write, stdin, stdout};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let random_int: i32 = rng.random_range(0..99);
    let mut chance = 10;
    let mut is_playing = true;
    let mut handle_out = stdout().lock();
    let mut handle_in = stdin().lock();
    let mut in_string: String = String::new();

    handle_out.write_all(b"Jeu du plus ou moins\n")?;
    handle_out.write_all(b"Le principe est le suivant: j'ai choisi un entier compris entre 0 et 99 et tu dois le deviner. Tu as 10 chances.\n")?;
    handle_out.write_all(b"C'est parti, quelle est ta proposition ?\n")?;

    while is_playing {
        in_string.clear();
        let _ = handle_in.read_line(&mut in_string);
        let user_number = in_string.trim().parse::<i32>();
        match user_number {
            Ok(value) => {
                if value == random_int {
                    is_playing = false;
                    handle_out.write_all(b"Bravo !!!\n")?
                } else if value < random_int {
                    // on incrémente si nombre inexacte
                    chance -= 1;
                    handle_out.write_all(b"C'est plus !!!\n")?
                } else {
                    chance -= 1;
                    handle_out.write_all(b"C'est moins !!!\n")?
                }
                if chance == 0 {
                    is_playing = false;
                    let message = String::from(
                        "Perdu, le nombre est:  ".to_owned()
                            + random_int.to_string().as_str()
                            + "\n",
                    );
                    handle_out.write_all(message.as_bytes())?
                }
            }
            Err(_) => handle_out.write_all(b"Veuillez saisir un nombre\n")?,
        }
    }
    Ok(())
}
