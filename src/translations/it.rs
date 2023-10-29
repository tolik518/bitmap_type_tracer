pub struct Italian;

impl super::Translation for Italian {
    fn help(&self) -> &'static str {
        "Usage: \n \
        Se il tuo font ha già un file di configurazione:\n \
        bitmap_type_tracer <path_to_bitmap_font> <text>\n\n \
        Se vuoi generare un file di configurazione: \n \
        bitmap_type_tracer <path_to_bitmap_font> <sequence> <text> <chars_per_row> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE] [--threshold VALUE] [--save-json]\n   \
        path_to_bitmap_font     letteralmente il percorso per il bitmap.png che contiene i caratteri\n   \
        sequence                la sequenza di caratteri che vedi nel font bitmap es. 'ABCDEF...'\n   \
        text                    il testo che vuoi scrivere con il font bitmap fornito\n   \
        chars_per_row           quanti caratteri ci sono in una riga nel font bitmap fornito\n \
        Margins: \n   \
        --top VALUE             il numero di pixel da ritagliare dalla parte superiore dell'immagine\n   \
        --bottom VALUE          il numero di pixel da ritagliare dalla parte inferiore dell'immagine\n   \
        --left VALUE            numero di pixel da ritagliare dalla parte sinistra dell'immagine\n   \
        --right VALUE           numero di pixel da ritagliare dalla parte destra dell'immagine\n \
        Other options: \n   \
        --threshold VALUE       il valore per determinare la soglia per rendere trasparente lo sfondo (0-255)\n   \
        --save-json             salva la configurazione in un file json\n   \
        --help                  stampa questo messaggio di aiuto\n   \
        --version               stampa la versione del programma\n\n \
        Per esempi di utilizzo consulta il README.md nel repository \n"
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
    fn character_not_found(&self, character: char) -> String {
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