pub struct Turkish;

/*
 * Author: ARDA KÖLEMENOĞLU
 * Created: 2023-10-28
 */
impl super::Translation for Turkish
{
    fn help_usage(&self) -> &'static str {
        "Usage:\n\
        \u{20} Yazı tipinizde zaten bir yapılandırma dosyası varsa: \n\
        \u{20} bitmap_type_tracer <path_to_bitmap_font> <text>\n\n\
        \u{20} Bir yapılandırma dosyası oluşturmak istiyorsanız:\n\
        \u{20} bitmap_type_tracer <path_to_bitmap_font> <sequence> <text> <chars_per_row> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE] [--threshold VALUE] [--save-json]"
    }

    fn help_parameters(&self) -> &'static str {
        "\n\
        \u{20}\u{20} path_to_bitmap_font     karakterleri içeren bitmap.png dosyasının yolu\n\
        \u{20}\u{20} sequence                bit eşlem yazı tipinde gördüğünüz karakter dizisi, örneğin 'ABCÇDEF...'\n\
        \u{20}\u{20} text                    bitmap yazı tipiyle yazmak istediğiniz metin\n\
        \u{20}\u{20} chars_per_row           bitmap yazı tipinde bir satırda kaç karakter olduğunu\n"
    }

    fn help_margins(&self) -> &'static str {
        "Margins:\n\
        \u{20}\u{20} --top VALUE             örüntünün üst kısmından kırpılacak piksel sayısı\n\
        \u{20}\u{20} --bottom VALUE          örüntünün alt kısmından kırpılacak piksel sayısı\n\
        \u{20}\u{20} --left VALUE            görüntünün solundan kırpılacak piksel sayısı\n\
        \u{20}\u{20} --right VALUE           görüntünün sağından kırpılacak piksel sayısı"
    }

    fn help_other_options(&self) -> &'static str {
        "Other options:\n\
        \u{20}\u{20} --threshold VALUE       arka planı saydam hale getirmeye yönelik eşiği belirleyen değer (0-255)\n\
        \u{20}\u{20} --save-json             yapılandırmayı json dosyasına kaydedin\n\
        \u{20}\u{20} --help                  yardım mesajını yazdır\n\
        \u{20}\u{20} --version               programın sürümünü yazdır\n\
        \u{20}\u{20} --lang                  Specify the language (en|it|fr|tr|...) of the application. Default is your system lang or en" // needs translation
    }

    fn help_example_usage(&self) -> &'static str {"Nasıl kullanıldığını öğrenmek için README.md dosyasını okuyun"}

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
        format!("Karakter '{}' bulunamadı. Onun yerine arka plan rengini kullanıyoruz.", character)
    }

    fn err_invalid_num_of_chars(&self) -> &'static str {"Yetersiz karakter.."}
    fn err_invalid_threshold(&self) -> &'static str {"Geçerli bir threshold girmelisiniz. (0-255)."}
    fn err_invalid_left_margin(&self) -> &'static str {"Sağ margin ayarlanamadı, lütfen geçerli bir değer girin."}
    fn err_invalid_right_margin(&self) -> &'static str {"Sol margin ayarlanamadı, lütfen geçerli bir değer girin."}
    fn err_invalid_top_margin(&self) -> &'static str {"Üst margin ayarlanamadı, lütfen geçerli bir değer girin."}
    fn err_invalid_bottom_margin(&self) -> &'static str {"Alt margin ayarlanamadı, lütfen geçerli bir değer girin."}

    fn err_failed_to_read_config(&self) -> &'static str {"Yazı tipi yapılandırma dosyası (json) okunamadı."}
    fn err_failed_to_parse_config(&self) -> &'static str {"Yazı tipi yapılandırma dosyası (json) analiz edilemedi."}
    fn err_failed_to_open_config(&self) -> &'static str {"Yazı tipi yapılandırma dosyası (json) analiz açılamadı."}
    fn err_failed_to_serialize_config(&self) -> &'static str {"Yazı tipi yapılandırma dosyası (json) dizilemedi."}
    fn err_failed_to_save_config(&self) -> &'static str {"Yazı tipi yapılandırma dosyası (json) kaydedilemedi."}
    fn err_invalid_font_path(&self) -> &'static str {"Geçersiz yazı tipi dosyası yolu."}

    fn err_failed_to_save_output_image(&self) -> &'static str {"Çıktı görüntüsü kaydedilemedi."}
    fn err_failed_to_open_font_image(&self) -> &'static str {"Yazı tipi görüntüsü açılamadı."}
}