use rust_xlsxwriter::Workbook;

use crate::import::data::ImportChapter;
use crate::import::data::ImportWord;
use crate::import::data::ImportWordbook;

const HEADER_CHAPTER_NAME: &str = "chapter_name";
const HEADER_SOURCE: &str = "source";
const HEADER_TRANSLATION: &str = "translation";
const HEADER_NOTE: &str = "note";
const SAMPLE_CHAPTER_1: &str = "Chapter 1";
const SAMPLE_CHAPTER_2: &str = "Chapter 2";
const SAMPLE_SOURCE_1: &str = "source_text_1";
const SAMPLE_SOURCE_2: &str = "source_text_2";
const SAMPLE_SOURCE_3: &str = "source_text_3";
const SAMPLE_TRANSLATION_1: &str = "translation_text_1";
const SAMPLE_TRANSLATION_2: &str = "translation_text_2";
const SAMPLE_TRANSLATION_3: &str = "translation_text_3";
const SAMPLE_NOTE_1: &str = "note_1";
const SAMPLE_NOTE_3: &str = "note_3";

pub struct TemplateGenerator;

impl TemplateGenerator {
    pub fn json_wordbook_template() -> String {
        let template = ImportWordbook {
            name: "wordbook_name".to_string(),
            description: Some("wordbook_description".to_string()),
            chapters: vec![ImportChapter::with_words(
                "chapter_name".to_string(),
                vec![
                    ImportWord::new(
                        SAMPLE_SOURCE_1.to_string(),
                        SAMPLE_TRANSLATION_1.to_string(),
                        Some(SAMPLE_NOTE_1.to_string()),
                    ),
                    ImportWord::new(
                        SAMPLE_SOURCE_2.to_string(),
                        SAMPLE_TRANSLATION_2.to_string(),
                        None,
                    ),
                ],
            )],
        };
        serde_json::to_string_pretty(&template).unwrap_or_default()
    }

    pub fn json_chapter_template() -> String {
        let template = ImportChapter::with_words(
            "chapter_name".to_string(),
            vec![
                ImportWord::new(
                    SAMPLE_SOURCE_1.to_string(),
                    SAMPLE_TRANSLATION_1.to_string(),
                    Some(SAMPLE_NOTE_1.to_string()),
                ),
                ImportWord::new(
                    SAMPLE_SOURCE_2.to_string(),
                    SAMPLE_TRANSLATION_2.to_string(),
                    None,
                ),
            ],
        );
        serde_json::to_string_pretty(&template).unwrap_or_default()
    }

    pub fn xml_wordbook_template() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<wordbook>
    <name>wordbook_name</name>
    <description>wordbook_description</description>
    <chapter>
        <name>chapter_name</name>
        <word>
            <source>source_text_1</source>
            <translation>translation_text_1</translation>
            <note>note_1</note>
        </word>
        <word>
            <source>source_text_2</source>
            <translation>translation_text_2</translation>
        </word>
    </chapter>
</wordbook>"#
            .to_string()
    }

    pub fn xml_chapter_template() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<chapter>
    <name>chapter_name</name>
    <word>
        <source>source_text_1</source>
        <translation>translation_text_1</translation>
        <note>note_1</note>
    </word>
    <word>
        <source>source_text_2</source>
        <translation>translation_text_2</translation>
    </word>
</chapter>"#
            .to_string()
    }

    pub fn xlsx_wordbook_template() -> Vec<u8> {
        Self::create_xlsx_content(true)
    }

    pub fn xlsx_chapter_template() -> Vec<u8> {
        Self::create_xlsx_content(false)
    }

    pub fn csv_wordbook_template() -> String {
        let mut content = String::new();
        content.push_str(&format!("{},{},{},{}\n", HEADER_CHAPTER_NAME, HEADER_SOURCE, HEADER_TRANSLATION, HEADER_NOTE));
        content.push_str(&format!("{},{},{},{}\n", SAMPLE_CHAPTER_1, SAMPLE_SOURCE_1, SAMPLE_TRANSLATION_1, SAMPLE_NOTE_1));
        content.push_str(&format!("{},{},{},\n", SAMPLE_CHAPTER_1, SAMPLE_SOURCE_2, SAMPLE_TRANSLATION_2));
        content.push_str(&format!("{},{},{},{}\n", SAMPLE_CHAPTER_2, SAMPLE_SOURCE_3, SAMPLE_TRANSLATION_3, SAMPLE_NOTE_3));
        content
    }

    pub fn csv_chapter_template() -> String {
        let mut content = String::new();
        content.push_str(&format!("{},{},{}\n", HEADER_SOURCE, HEADER_TRANSLATION, HEADER_NOTE));
        content.push_str(&format!("{},{},{}\n", SAMPLE_SOURCE_1, SAMPLE_TRANSLATION_1, SAMPLE_NOTE_1));
        content.push_str(&format!("{},{},\n", SAMPLE_SOURCE_2, SAMPLE_TRANSLATION_2));
        content
    }

    fn create_xlsx_content(is_wordbook: bool) -> Vec<u8> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        if is_wordbook {
            let _ = worksheet.write_string(0, 0, HEADER_CHAPTER_NAME);
            let _ = worksheet.write_string(0, 1, HEADER_SOURCE);
            let _ = worksheet.write_string(0, 2, HEADER_TRANSLATION);
            let _ = worksheet.write_string(0, 3, HEADER_NOTE);

            let _ = worksheet.write_string(1, 0, SAMPLE_CHAPTER_1);
            let _ = worksheet.write_string(1, 1, SAMPLE_SOURCE_1);
            let _ = worksheet.write_string(1, 2, SAMPLE_TRANSLATION_1);
            let _ = worksheet.write_string(1, 3, SAMPLE_NOTE_1);

            let _ = worksheet.write_string(2, 0, SAMPLE_CHAPTER_1);
            let _ = worksheet.write_string(2, 1, SAMPLE_SOURCE_2);
            let _ = worksheet.write_string(2, 2, SAMPLE_TRANSLATION_2);

            let _ = worksheet.write_string(3, 0, SAMPLE_CHAPTER_2);
            let _ = worksheet.write_string(3, 1, SAMPLE_SOURCE_3);
            let _ = worksheet.write_string(3, 2, SAMPLE_TRANSLATION_3);
            let _ = worksheet.write_string(3, 3, SAMPLE_NOTE_3);
        } else {
            let _ = worksheet.write_string(0, 0, HEADER_SOURCE);
            let _ = worksheet.write_string(0, 1, HEADER_TRANSLATION);
            let _ = worksheet.write_string(0, 2, HEADER_NOTE);

            let _ = worksheet.write_string(1, 0, SAMPLE_SOURCE_1);
            let _ = worksheet.write_string(1, 1, SAMPLE_TRANSLATION_1);
            let _ = worksheet.write_string(1, 2, SAMPLE_NOTE_1);

            let _ = worksheet.write_string(2, 0, SAMPLE_SOURCE_2);
            let _ = worksheet.write_string(2, 1, SAMPLE_TRANSLATION_2);
        }

        workbook.save_to_buffer().unwrap_or_default()
    }
}