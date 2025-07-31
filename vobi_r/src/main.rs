use clap::{Arg, Command};
fn main() {
    let matches = Command::new("VoBi")
        .version("0.1")
        .author("jethi05")
       .about("Sagt dir, von wann bis wann du gearbeitet hast")
        .arg(
            Arg::new("von")
                .short('v')
                .long("von")
                .value_name("START")
                .help("Startzeitpunkt")
                .required(true)
        )
        .arg(
            Arg::new("bis")
                .short('b')
                .long("bis")
                .value_name("ENDE")
                .help("Endzeitzeitpunkt")
                .required(true)
        )
        .get_matches();
        
    // Input wird erhalten und umgewandelt in Stunden und Minuten
	let v = matches.get_one::<String>("von").expect("REASON").as_str();
	let von_h: f32 = v[0..2].parse().unwrap();
	let von_m: f32 = v[2..4].parse().unwrap();
	let von = von_h + (von_m / 60.0);

	let b = matches.get_one::<String>("bis").expect("REASON").as_str();
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
