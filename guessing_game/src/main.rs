use std::io; 

// ca sa fac citire de la tastatura, am nevoie de biblioteca aceasta care e standard

use rand::Rng;
use std::cmp::Ordering;

// tipul Ordering e iar enum cu Less, greater, Equal

// acesta e preludiul, chestii care sunt deja definite in bibl std
// daca un tip de care ai nevoie nu e in prelude, atunci pui in fata lui cuv cheie use

fn main() { 
    // entry point in program
    
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // thread_rng e locala acestui
    
    // thread/fir de executie si e implementat de so
    // urmatoarea chestie e facut de Rng (gen e un trait a lui rand)
    // start..=end (un fel de for)(de la mai mic la mai mare!)
    
    // daca nu stii trait uri de fctii e ok, 
    // exista cargo doc --open care iti deschide un fel de pag de manual

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); 
    
    // let creeaza variab + mut ca e modificabila
    // String::new returneaza ce e legat la var guess dar e o enumerrare!
    // cand amn fct asta cu string::new crede ca punemn string, desi vrem un int!
    // daca nu-i spui ce tip numeric folosesti, rust il pun by deafult la i32 (signed int)
    // noi trebuie sa-l convertim 
    
        io::stdin() // handler de user input
            .read_line(&mut guess) 
            .expect("Failed to read line");
        
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue, // _ ia toate tipurile, un fel de orice (catchall)
             };
    
    // ia ce scrie userul si il appenduieste la un string
    // & referinta si e immutable
    // mai cream o noua variabila guess ce o umbreste pe prima
    // guess.trim() afce referiere la primul guess
    // trimul scapa de spatiile albe de la inceput si final
    //parse converteste un string in altceva
    // : anoteaza tipul si programul crede ca ceea ce compar cu el ^ va fi de acelasi tip
    // parse merge acum numnai cu chestii ce pot fi convertite logic in numere
    //ptr ca poate da gres, parse da un rezultat ce se trateaza cu expect
        
    // astea 3 linii de mai sus mergeau si pe un singur rand cu atatea pct cate sunt
    // variantele rezultatului sunt ok si err
    // un fel de try and catch cu expect, adica functioneaza numai in caz de eroare si ia argum
    // transmis ca param (mesajul) si-l afiseaza ca eroare, altfel compileaza si da warning =/
    // {} placeholder; se pot printa mai multe cu {}
    
        println!("You guessed: {guess}");
    
    // pentru a genera nr random exista rand crate de biblioteca
    // crate ul e o colectie de coduri sursa rust
    // in fis toml punem dependinta cu rand si versiunea sa, 
    // cargo intelege ca prin "versiune" e orice versiune mai mica sau egala, adica ^versiune
    // cargo considera publice versiunile ca un fel de api-uri compatibile cu versiunea versiune
    // Crates.io e unde cargo pune crate uri
    // cargo stie daca modifici sau nu chestii si chiar daca ii dai sa compileze, nu mai compileaza iar
    // in cargo.lock sunt scrise versiunile de dependinmte pe care le considera el ca fiind optime
    // however exista cargo update daca vrei neaparat sa updatezi dependinte
    // aici se ignora cargo.lock si se suprascrie versiunea
    
        match guess.cmp(&secret_number){ 
    
    //cmp e folosit la orice comparari si returneaza o varianta a enumului de Ordering
    // match e un fel de decizie ptr un fel de if
    // o expresie match e facuta din arms(maini)
    // match e un fel de pattern de verificat pe baza lui gen 
    // e cu virgula!!
        
            Ordering::Less => println!("Too smol!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}