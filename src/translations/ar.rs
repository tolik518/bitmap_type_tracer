pub struct Arabic;

/*
 * Author: Anatolij "tolik518" Vasilev
 * Created: 2023-10-26
 */
impl super::Translation for Arabic
{
    fn help_usage(&self) -> &'static str {
        "الاستخدام:\n\
        \u{20} إذا كان للخط النقطي ملف تكوين مسبق: \n\
        \u{20} bitmap_type_tracer <مسار_خط_البتماب> <النص>\n\n\
        \u{20} إذا أردت إنشاء ملف تكوين: \n\
        \u{20} bitmap_type_tracer <مسار_خط_البتماب> <التسلسل> <النص> <عدد_الأحرف_في_الصف> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE] [--threshold VALUE] [--save-json] [--lang en|it|fr|tr|...]"
    }

    fn help_parameters(&self) -> &'static str {
        "\n\
        \u{20}\u{20} مسار_خط_البتماب        حرفيًا المسار إلى ملف bitmap.png الذي يحتوي الأحرف\n\
        \u{20}\u{20} التسلسل                تسلسل الأحرف التي تراها في خط البتماب مثل 'ABCDEF...'\n\
        \u{20}\u{20} النص                    النص الذي ترغب في كتابته باستخدام خط البتماب المتوفر\n\
        \u{20}\u{20} عدد_الأحرف_في_الصف      كم عدد الأحرف في صف في خط البتماب المتوفر\n"
    }

    fn help_margins(&self) -> &'static str {
        "الهوامش:\n\
        \u{20}\u{20} --top VALUE             عدد البكسلات التي يتم قصها من أعلى الصورة\n\
        \u{20}\u{20} --bottom VALUE          عدد البكسلات التي يتم قصها من أسفل الصورة\n\
        \u{20}\u{20} --left VALUE            عدد البكسلات التي يتم قصها من يسار الصورة\n\
        \u{20}\u{20} --right VALUE           عدد البكسلات التي يتم قصها من يمين الصورة"
    }

    fn help_other_options(&self) -> &'static str {
        "خيارات أخرى:\n\
        \u{20}\u{20} --threshold VALUE       القيمة لتحديد العتبة لجعل الخلفية شفافة (0-255)\n\
        \u{20}\u{20} --save-json             حفظ التكوين إلى ملف json\n\
        \u{20}\u{20} --help                  طباعة رسالة المساعدة هذه\n\
        \u{20}\u{20} --version               طباعة إصدار البرنامج\n\
        \u{20}\u{20} --lang                  حدد لغة التطبيق (en|it|fr|tr|...) الافتراضية هي لغة النظام أو en"
    }

    fn help_example_usage(&self) -> &'static str { "لأمثلة الاستخدام اطلع على ملف README.md في المستودع" }

    fn help(&self) -> String {
        format!(
            "{}\n\n{}\n\n{}\n\n{}\n",
            self.help_usage(), self.help_margins(), self.help_other_options(), self.help_example_usage()
        )
    }

    fn version(&self) -> &'static str { env!("CARGO_PKG_VERSION") }
    fn repository(&self) -> &'static str { env!("CARGO_PKG_REPOSITORY") }
    fn name(&self) -> &'static str { env!("CARGO_PKG_NAME") }
    fn author(&self) -> &'static str { env!("CARGO_PKG_AUTHORS") }
    fn full_help(&self) -> String {
        format!(
            "{} بواسطة {}\nالإصدار: {}\nالمستودع: {}\n{}",
            self.name(), self.author(), self.version(), self.repository(), self.help()
        )
    }
    fn warn_character_not_found(&self, character: char) -> String {
        format!("لم يتم العثور على الحرف '{}' في التسلسل. يتم محاولة استخدام لون الخلفية بدلاً من ذلك.", character)
    }

    fn err_invalid_num_of_chars(&self) -> &'static str { "يجب أن تقدم عددًا صحيحًا من الأحرف لكل صف." }
    fn err_invalid_threshold(&self) -> &'static str { "يجب أن تقدم قيمة عتبة صحيحة (0-255)." }
    fn err_invalid_left_margin(&self) -> &'static str { "فشل في قراءة وسيط هامش اليسار. يرجى تقديم قيمة صحيحة." }
    fn err_invalid_right_margin(&self) -> &'static str { "فشل في قراءة وسيط هامش اليمين. يرجى تقديم قيمة صحيحة." }
    fn err_invalid_top_margin(&self) -> &'static str { "فشل في قراءة وسيط هامش الأعلى. يرجى تقديم قيمة صحيحة." }
    fn err_invalid_bottom_margin(&self) -> &'static str { "فشل في قراءة وسيط هامش الأسفل. يرجى تقديم قيمة صحيحة." }

    fn err_failed_to_read_config(&self) -> &'static str { "فشل في قراءة ملف تكوين الخط (json)." }
    fn err_failed_to_parse_config(&self) -> &'static str { "فشل في تحليل ملف تكوين الخط (json)." }
    fn err_failed_to_open_config(&self) -> &'static str { "فشل في فتح ملف تكوين الخط (json)." }
    fn err_failed_to_serialize_config(&self) -> &'static str { "فشل في تسلسل ملف تكوين الخط (json)." }
    fn err_failed_to_save_config(&self) -> &'static str { "فشل في حفظ ملف تكوين الخط (json)." }
    fn err_invalid_font_path(&self) -> &'static str { "مسار الخط غير صالح" }

    fn err_failed_to_save_output_image(&self) -> &'static str { "فشل في حفظ صورة الإخراج" }
    fn err_failed_to_open_font_image(&self) -> &'static str { "فشل في فتح صورة الخط" }
}