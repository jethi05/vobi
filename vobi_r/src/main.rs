use std::io;
fn main() {
    let mut start_eingabe = String::new();
    let mut end_eingabe = String::new();
    println!("Beispiel 0900");
    println!("Startzeit:");
    io::stdin()
        .read_line(&mut start_eingabe)
        .unwrap();
    let mut start_int_eingabe: i32 = start_eingabe.trim().parse().unwrap();
    println!("Beispiel 1800");
    println!("Endzeit:");
    io::stdin()
        .read_line(&mut end_eingabe)
        .unwrap();
    let mut end_int_eingabe: i32 = end_eingabe.trim().parse().unwrap();
    println!("Folgende Zeiten wurden berücksichtigt:");
    println!("Start {}", start_eingabe.trim());
    println!("end {}", end_eingabe.trim());
    let mut ergebnis = end_int_eingabe - start_int_eingabe;
    println!("{}", ergebnis);
}
