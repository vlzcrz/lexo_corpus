pub mod modules;

use std::{thread, time::Duration};

use modules::{
    cli_handlers::clear_screen, file_handlers::initialize_main_folders, menu::main_menu,
};
use owo_colors::OwoColorize;
use symspell::{AsciiStringStrategy, SymSpell};

fn main() {
    println!("Inicializando corrector sintactico y semantico Symspell");
    println!("lang: {}", "en".on_white().bold());
    println!("[en] Cargando Unigramas...");
    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
    symspell.load_dictionary(
        "./symspell_dictionaries/frequency_dictionary_en_82_765.txt",
        0,
        1,
        " ",
    );
    println!("{}", "Finalizado".on_green());
    println!("[en] Cargando Bigramas...");
    symspell.load_bigram_dictionary("./symspell_dictionaries/bigrams.txt", 0, 2, " ");
    println!("{}", "Finalizado".on_green());

    initialize_main_folders();
    thread::sleep(Duration::new(1, 0));

    clear_screen();
    main_menu(&symspell);
}
