use clap::{Arg, Command}; // fuer argument parsing
use colored::Colorize; // fuer Lila
use chrono::Local; //zeitberechnung

fn berechne_zeit(v: &str, b: &str) {

    // Input wird erhalten und umgewandelt in Stunden und Minuten
    let von_h: f32 = v[0..2].parse().unwrap();
    let von_m: f32 = v[2..4].parse().unwrap();
    let von = von_h + (von_m / 60.0);

    let bis_h: f32 = b[0..2].parse().unwrap();
    let bis_m: f32 = b[2..4].parse().unwrap();
    let bis = bis_h + (bis_m / 60.0);

    // Ergebnis berechnung 
    let mut ergebnis = bis - von;
   // Minimale Zeitberechnung 
    if ergebnis >= 9.0 {
        println!("45 Minuten Pause abgezogen");
        ergebnis = ergebnis - 0.75;
    }else if ergebnis >= 6.0 {
        println!("30 Minuten Pause abgezogen");
        ergebnis = ergebnis - 0.5;
    }
    let ergebnis_s = ergebnis.trunc();
    let ergebnis_m = ((ergebnis - ergebnis_s) * 60.0).round() as u32;
    println!("Du hast {} Stunden und {} Minuten", ergebnis_s, ergebnis_m);
}

// Logo
const LOGO_TEXT: &str = concat!(
    r#" ___      ___ ________  ________  ___     
|\  \    /  /|\   __  \|\   __  \|\  \    
\ \  \  /  / | \  \|\  \ \  \|\ /\ \  \   
 \ \  \/  / / \ \  \\\  \ \   __  \ \  \  
  \ \    / /   \ \  \\\  \ \  \|\  \ \  \ 
   \ \__/ /     \ \_______\ \_______\ \__\
    \|__|/       \|_______|\|_______|\|__|
    By jethi05"#,
);

fn main() {
    let colored_logo = format!("{}\n", LOGO_TEXT.magenta());

    let matches = Command::new("VoBi")
        .version("0.1")
        .author("jethi05")
       .about("Sagt dir, von wann bis wann du gearbeitet hast")
       .before_help(colored_logo)
       .arg_required_else_help(true)
        .arg(
            Arg::new("von")
                //.short('v')
                //.long("von")
                .value_name("START")
                .help("Startzeitpunkt 0730 (7.30 Uhr)")
                .index(1) // Aufruf ohne "-v"
                .required(true)
        )
        .arg(
            Arg::new("bis")
                .short('b')
                .long("bis")
                .value_name("ENDE")
                .help("Endzeitzeitpunkt 1800 (18:00 Uhr)")
                .required(false)
        )
        .get_matches();

    // von var
    let v = matches.get_one::<String>("von").unwrap().to_string();
    // pruefen, ob bis da ist, wenn nicht dann local time nutzen
    let b = match matches.get_one::<String>("bis") {
        Some(zeit) => zeit.to_string(), // user hat --bis angegeben
        None => Local::now().format("%H%M").to_string(), // generiere die geradige Zeit
    };

    // func berechnung aufrufen
    berechne_zeit(&v, &b);

}
