pub struct French;

/*
 * Author: Apoorv "Sam654123"
 * Created: 2023-10-30
 */
impl super::Translation for French
{
    fn help_usage(&self) -> &'static str {
        "Usage:\n\
        \u{20} Si votre police possède déjà un fichier de configuration : \n\
        \u{20} bitmap_type_tracer <path_to_bitmap_font> <text>\n\n\
        \u{20} Si vous souhaitez générer un fichier de configuration : \n\
        \u{20} bitmap_type_tracer <path_to_bitmap_font> <sequence> <text> <chars_per_row> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE] [--threshold VALUE] [--save-json]"
    }

    fn help_parameters(&self) -> &'static str {
        "\n\
        \u{20}\u{20} path_to_bitmap_font     littéralement le chemin vers le bitmap.png contenant les caractères\n\
        \u{20}\u{20} sequence                la séquence de caractères que vous voyez dans la police bitmap e.x 'ABCDEF...'\n\
        \u{20}\u{20} text                    le texte que vous souhaitez écrire avec la police bitmap fournie\n\
        \u{20}\u{20} chars_per_row           combien de caractères sont alignés dans la police bitmap fournie\n"
    }

    fn help_margins(&self) -> &'static str {
        "Margins:\n\
        \u{20}\u{20} --top VALUE             le nombre de pixels à recadrer à partir du haut de l'image\n\
        \u{20}\u{20} --bottom VALUE          le nombre de pixels à recadrer à partir du bas de l'image\n\
        \u{20}\u{20} --left VALUE            le nombre de pixels à recadrer à gauche de l'image\n\
        \u{20}\u{20} --right VALUE           le nombre de pixels à recadrer à droite de l'image"
    }

    fn help_other_options(&self) -> &'static str {
        "Other options:\n\
        \u{20}\u{20} --threshold VALUE       la valeur pour déterminer le seuil de transparence du fond (0-255)\n\
        \u{20}\u{20} --save-json             enregistrer la configuration dans un fichier json\n\
        \u{20}\u{20} --help                  imprimer ce message d'aide\n\
        \u{20}\u{20} --version               imprimer la version du programme"
    }

    fn help_example_usage(&self) -> &'static str {"Pour des exemples d'utilisation, consultez le README.md dans le référentiel"}

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
        format!("Caractère '{}' introuvable dans la séquence. Essayer d'utiliser la couleur d'arrière-plan à la place.", character)
    }

    fn err_invalid_num_of_chars(&self) -> &'static str {"Vous devez fournir un nombre valide de caractères par ligne."}
    fn err_invalid_threshold(&self) -> &'static str {"Vous devez fournir une valeur seuil valide (0-255)."}
    fn err_invalid_left_margin(&self) -> &'static str {"Échec de la lecture de l'argument de la marge gauche. Veuillez fournir une valeur valide."}
    fn err_invalid_right_margin(&self) -> &'static str {"Échec de la lecture de l'argument de marge correct. Veuillez fournir une valeur valide."}
    fn err_invalid_top_margin(&self) -> &'static str {"Échec de la lecture de l'argument de la marge supérieure. Veuillez fournir une valeur valide."}
    fn err_invalid_bottom_margin(&self) -> &'static str {"Échec de la lecture de l'argument de la marge inférieure. Veuillez fournir une valeur valide."}

    fn err_failed_to_read_config(&self) -> &'static str {"Échec de la lecture du fichier de configuration de police (json)."}
    fn err_failed_to_parse_config(&self) -> &'static str {"Échec de l'analyse du fichier de configuration de police (json)."}
    fn err_failed_to_open_config(&self) -> &'static str {"Échec de l'ouverture du fichier de configuration de police (json)."}
    fn err_failed_to_serialize_config(&self) -> &'static str {"Échec de la sérialisation du fichier de configuration de police (json)."}
    fn err_failed_to_save_config(&self) -> &'static str {"Échec de l'enregistrement du fichier de configuration de police (json)."}
    fn err_invalid_font_path(&self) -> &'static str {"Chemin de police invalide"}

    fn err_failed_to_save_output_image(&self) -> &'static str {"Échec de l'enregistrement de l'image de sortie"}
    fn err_failed_to_open_font_image(&self) -> &'static str {"Échec de l'ouverture de l'image de la police"}
}
