pub struct Italian;

/*
 * Author: Mattia "Tizzz-555" Beccari
 * Created: 2023-10-29
 */
impl super::Translation for Italian
{
    fn help_usage(&self) -> &'static str {
        "Usage: \n\
        \u{20} Se il tuo font ha già un file di configurazione:\n
        \u{20} bitmap_type_tracer <path_to_bitmap_font> <text>\n\n\
        \u{20} Se vuoi generare un file di configurazione:\n\
        \u{20} bitmap_type_tracer <path_to_bitmap_font> <sequence> <text> <chars_per_row> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE] [--threshold VALUE] [--save-json]"
    }

    fn help_parameters(&self) -> &'static str {
        "\n\
        \u{20}\u{20} path_to_bitmap_font     letteralmente il percorso per il bitmap.png che contiene i caratteri\n\
        \u{20}\u{20} sequence                la sequenza di caratteri che vedi nel font bitmap es. 'ABCDEF...'\n\
        \u{20}\u{20} text                    il testo che vuoi scrivere con il font bitmap fornito\n\
        \u{20}\u{20} chars_per_row           quanti caratteri ci sono in una riga nel font bitmap fornito\n"
    }

    fn help_margins(&self) -> &'static str {
        "Margins:\n\
        \u{20}\u{20} --top VALUE             il numero di pixel da ritagliare dalla parte superiore dell'immagine\n\
        \u{20}\u{20} --bottom VALUE          il numero di pixel da ritagliare dalla parte inferiore dell'immagine\n\
        \u{20}\u{20} --left VALUE            numero di pixel da ritagliare dalla parte sinistra dell'immagine\n\
        \u{20}\u{20} --right VALUE           numero di pixel da ritagliare dalla parte destra dell'immagine"
    }

    fn help_other_options(&self) -> &'static str {
        "Other options:\n\
        \u{20}\u{20} --threshold VALUE       il valore per determinare la soglia per rendere trasparente lo sfondo (0-255)\n\
        \u{20}\u{20} --save-json             salva la configurazione in un file json\n\
        \u{20}\u{20} --help                  stampa questo messaggio di aiuto\n\
        \u{20}\u{20} --version               stampa la versione del programma"
    }

    fn help_example_usage(&self) -> &'static str {"Per esempi di utilizzo consulta il README.md nel repository"}

    fn help(&self) -> String {
        format!(
            "{}\n\n{}\n\n{}\n\n{}\n",
            self.help_usage(), self.help_margins(), self.help_other_options(), self.help_example_usage()
        )
    }

    fn version(&self) -> &'static str {env!("CARGO_PKG_VERSION")}
    fn repository(&self) -> &'static str {env!("CARGO_PKG_REPOSITORY")}
    fn name(&self) -> &'static str {env!("CARGO_PKG_NAME")}
    fn author(&self) -> &'static str {env!("CARGO_PKG_AUTHORS")}
    fn full_help(&self) -> String {
        format!(
            "{} by {}\nVersion: {}\nRepository: {}\n{}",
            self.name(), self.author(), self.version(), self.repository(), self.help()
        )
    }
    fn warn_character_not_found(&self, character: char) -> String {
        format!("Carattere '{}' non trovato nella sequenza. Si tenta di utilizzare il colore di sfondo in alternativa.", character)
    }

    fn err_invalid_num_of_chars(&self) -> &'static str {"Devi fornire un numero valido di caratteri per riga."}
    fn err_invalid_threshold(&self) -> &'static str {"Devi fornire un valore di soglia valido (0-255)."}
    fn err_invalid_left_margin(&self) -> &'static str {"Lettura del margine sinistro non riuscita. Si prega di fornire un valore valido."}
    fn err_invalid_right_margin(&self) -> &'static str {"Lettura del margine destro non riuscita. Si prega di fornire un valore valido."}
    fn err_invalid_top_margin(&self) -> &'static str {"Lettura del margine superiore non riuscita. Si prega di fornire un valore valido."}
    fn err_invalid_bottom_margin(&self) -> &'static str {"Lettura del margine inferiore non riuscita. Si prega di fornire un valore valido."}

    fn err_failed_to_read_config(&self) -> &'static str {"Lettura del file di configurazione del font (json) non riuscita."}
    fn err_failed_to_parse_config(&self) -> &'static str {"Analisi del file di configurazione del font (json) non riuscita."}
    fn err_failed_to_open_config(&self) -> &'static str {"Apertura del file di configurazione del font (json) non riuscita."}
    fn err_failed_to_serialize_config(&self) -> &'static str {"Serializzazione del file di configurazione del font (json) non riuscita."}
    fn err_failed_to_save_config(&self) -> &'static str {"Salvataggio del file di configurazione del font (json) non riuscito."}
    fn err_invalid_font_path(&self) -> &'static str {"Percorso del font non valido"}

    fn err_failed_to_save_output_image(&self) -> &'static str {"Salvataggio dell'immagine di output non riuscito"}
    fn err_failed_to_open_font_image(&self) -> &'static str {"Apertura dell'immagine del font non riuscita"}
}