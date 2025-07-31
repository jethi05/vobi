# VoBi
> Von -> Bis  

**VoBi** soll nachgeriechte Zeit einträge erleichtern, dadurch muss man nicht jedes mal nachdenken,
wenn man Zeit nachträgt :)  
Wenn Cargo installiert ist, den normalen weg gehen
- clonen
- in `vobi_r`
- cargo build
- datei in `vobi_r/target/debug/vobi_r` abholen
- in bin legen und umbennen (wenn man will) in **vobi**

## help
```
Sagt dir, von wann bis wann du gearbeitet hast

Usage: vobi --von <START> --bis <ENDE>

Options:
  -v, --von <START>  Startzeitpunkt
  -b, --bis <ENDE>   Endzeitzeitpunkt
  -h, --help         Print help
  -V, --version      Print version
```

## Aufruf
`vobi -v <Startzeit> -b <Endzeit>`  
`vobi -v 0900 -b 1800`
