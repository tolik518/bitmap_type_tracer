pub struct Turkish;

impl super::Translation for Turkish {
    fn help(&self) -> &'static str {
        "Usage: \n \
       Yazı tipinizde zaten bir yapılandırma dosyası varsa:\n \
        bitmap_type_tracer <path_to_bitmap_font> <text>\n\n \
       Bir yapılandırma dosyası oluşturmak istiyorsanız: \n \
        bitmap_type_tracer <path_to_bitmap_font> <sequence> <text> <chars_per_row> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE] [--threshold VALUE] [--save-json]\n   \
        path_to_bitmap_font     karakterleri içeren bitmap.png dosyasının yolu\n   \
        sequence                bit eşlem yazı tipinde gördüğünüz karakter dizisi, örneğin 'ABCÇDEF...'\n   \
        text                    bitmap yazı tipiyle yazmak istediğiniz metin\n   \
        chars_per_row           bitmap yazı tipinde bir satırda kaç karakter olduğunu\n \
        Margins: \n   \
        --top VALUE             görüntünün üst kısmından kırpılacak piksel sayısı\n   \
        --bottom VALUE          görüntünün alt kısmından kırpılacak piksel sayısı\n   \
        --left VALUE            görüntünün solundan kırpılacak piksel sayısı\n   \
        --right VALUE           görüntünün sağından kırpılacak piksel sayısı\n \
        Other options: \n   \
        --threshold VALUE       arka planı saydam hale getirmeye yönelik eşiği belirleyen değer (0-255)\n   \
        --save-json             yapılandırmayı json dosyasına kaydedin\n   \
        --help                  yardım mesajını yazdır\n   \
        --version               programın sürümünü yazdır\n\n \
      Nasıl kullanıldığını öğrenmek için README.md dosyasını okuyun. \n"
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