use std::{num::NonZero, path::PathBuf};

use cbc::Decryptor;
use aes::{cipher::{generic_array::GenericArray, BlockDecryptMut, BlockSizeUser, KeyIvInit}, Aes128};
use inflate::inflate_bytes_zlib;
use lru::LruCache;
use rusqlite::Connection;
use sha2::{Sha256, Digest};

use super::tables::*;

const TARGET: &'static str = "catalog::publication";

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum ContentTables {
    Document,
    // Other aren't implemented
    DatedText,
    Endnote,
    Extract,
    ExtractMultimedia,
    Multimedia,
    Footnote,
    ParagraphCommentary,
    Question,
    SearchIndexBibleVerse,
    SearchIndexDocument,
    SearchTextRangeBibleVerse,
    SearchTextRangeDocument,
    VerseCommentary,
}

pub struct Publication {
    pub catalog_id: i64,
    db: Connection,
    master_key: Vec<u8>,
    decrypted_content_cache: LruCache<(ContentTables, i32), String>
}

impl Publication {
    pub fn from_database<'a>(database_path: PathBuf, id: i64) -> Result<Self, Box<dyn std::error::Error>> {
        let db = Connection::open(database_path)?;
        let master_key: Vec<u8>;

        debug!(
            target: TARGET,
            "Forging master key for decryption jobs..."
        );
        {
            let mut stmt = db
                .prepare("SELECT MepsLanguageIndex, Symbol, Year, IssueTagNumber FROM Publication")?;

            let (meps_language_index, 
                symbol, 
                year, 
                issue_tag_number
            ) = stmt.query_row([], |row| Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, String>(3)?,
            )))?;

            let key_string = if issue_tag_number == "0" {
                String::from(format!("{}_{}_{}", meps_language_index, symbol, year))
            } else {
                String::from(format!(
                    "{}_{}_{}_{}",
                    meps_language_index, symbol, year, issue_tag_number
                ))
            };

            let mut hasher = Sha256::new();
            hasher.update(key_string.as_bytes());
            let key_part1 = hasher.finalize().to_vec();
            let key_part2 = hex::decode("11cbb5587e32846d4c26790c633da289f66fe5842a3a585ce1bc3a294af5ada7").unwrap();

            master_key = key_part1
                .iter()
                .zip(key_part2)
                .map(|(x, y)| x ^ y)
                .collect();
        }

        debug!(
            target: TARGET,
            "Master key forged."
        );

        Ok(Self {
            catalog_id: id,
            db,
            master_key,
            decrypted_content_cache: LruCache::new(NonZero::new(5).unwrap())
        })
    }

    pub fn get_view_items(&self) -> Result<Vec<PublicationViewItem>, Box<dyn std::error::Error>> {
        let mut fallback = false;
        let mut stmt = match self.db.prepare("SELECT
            PublicationViewItemId,
            PublicationViewId,
            ParentPublicationViewItemId,
            Title,
            TitleRich,
            SchemaType,
            ChildTemplateSchemaType,
            DefaultDocumentId
        FROM PublicationViewItem") {
            Ok(stmt) => stmt,
            Err(_) => {
                fallback = true;
                info!(
                    target: TARGET,
                    "Falling back to deprecated PublicationViewItem table."
                );
                self.db.prepare("SELECT
                    PublicationViewItemId,
                    PublicationViewId,
                    ParentPublicationViewItemId,
                    Title,
                    SchemaType,
                    ChildTemplateSchemaType,
                    DefaultDocumentId
                FROM PublicationViewItem")?
            }
        };
        let mut rows = stmt.query([])?;

        let mut view_items = vec![];

        while let Some(row) = rows.next()? {
            let item = PublicationViewItem {
                id: row.get(0)?,
                publication_view_id: row.get(1)?,
                parent_publication_view_item_id: row.get(2)?,
                title: row.get(3)?,
                title_rich: match fallback {
                    false => row.get(4)?,
                    true => None,
                },
                schema_type: match fallback {
                    false => row.get(5)?,
                    true => row.get(4)?,
                },
                child_template_schema_type: match fallback {
                    false => row.get(6)?,
                    true => row.get(5)?,
                },
                default_document_id: match fallback {
                    false => row.get(7)?,
                    true => row.get(6)?,
                },
            };

            view_items.push(item);
        }

        Ok(view_items)
    }

    pub fn get_view_items_documents(&self) -> Result<Vec<PublicationViewItemDocument>, Box<dyn std::error::Error>> {
        let mut stmt = self.db.prepare("SELECT 
            PublicationViewItemDocumentId,
            PublicationViewItemId,
            DocumentId
        FROM PublicationViewItemDocument")?;
        let mut rows = stmt.query([])?;

        let mut documents = vec![];

        while let Some(row) = rows.next()? {
            let document = PublicationViewItemDocument {
                id: row.get(0)?,
                publication_view_item_id: row.get(1)?,
                document_id: row.get(2)?,
            };

            documents.push(document);
        }

        Ok(documents)
    }

    pub fn get_documents(&mut self) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
        let mut fallback = false;
        let mut stmt = match self.db.prepare("SELECT
            DocumentId,
            PublicationId,
            MepsDocumentId,
            MepsLanguageIndex,
            Class,
            Type,
            SectionNumber,
            ChapterNumber,
            Title,
            TitleRich,
            TocTitle,
            TocTitleRich,
            ContextTitle,
            ContextTitleRich,
            FeatureTitle,
            FeatureTitleRich,
            Subtitle,
            SubtitleRich,
            FeatureSubtitle,
            FeatureSubtitleRich,
            Content,
            FirstFootnoteId,
            LastFootnoteId,
            FirstBibleCitationId,
            LastBibleCitationId,
            ParagraphCount,
            HasMediaLinks,
            HasLinks,
            FirstPageNumber,
            LastPageNumber,
            ContentLength,
            PreferredPresentation,
            ContentReworkedDate,
            HasPronunciationGuide
        FROM Document") {
            Ok(stmt) => stmt,
            Err(_err) => {
                fallback = true;
                info!(
                    target: TARGET,
                    "Falling back to deprecated Document table."
                );
                self.db.prepare("SELECT
                    DocumentId,
                    PublicationId,
                    MepsDocumentId,
                    MepsLanguageIndex,
                    Class,
                    Type,
                    SectionNumber,
                    Title,
                    TocTitle,
                    ContextTitle,
                    FeatureTitle,
                    Subtitle,
                    FeatureSubtitle,
                    Content,
                    FirstFootnoteId,
                    LastFootnoteId,
                    FirstBibleCitationId,
                    LastBibleCitationId,
                    ParagraphCount,
                    HasMediaLinks,
                    HasLinks,
                    FirstPageNumber,
                    LastPageNumber,
                    ContentLength
                FROM Document")?
            }
        };
        let mut rows = stmt.query([])?;

        let mut documents = vec![];

        while let Some(row) = rows.next()? {
            let document = Document {
                id: row.get(0)?,
                publication_id: row.get(1)?,
                meps_document_id: row.get(2)?,
                meps_language_id: row.get(3)?,
                class: row.get(4)?,
                type_id: row.get(5)?,
                section_number: row.get(6)?,
                chapter_number: match fallback {
                    false => row.get(7)?,
                    true => None,
                },
                title: match fallback {
                    false => row.get(8)?,
                    true => row.get(7)?,
                },
                title_rich: match fallback {
                    false => row.get(9)?,
                    true => None,
                },
                toc_title: match fallback {
                    false => row.get(10)?,
                    true => row.get(8)?,
                },
                toc_title_rich: match fallback {
                    false => row.get(11)?,
                    true => None,
                },
                context_title: match fallback {
                    false => row.get(12)?,
                    true => row.get(9)?,
                },
                context_title_rich: match fallback {
                    false => row.get(13)?,
                    true => None,
                },
                feature_title: match fallback {
                    false => row.get(14)?,
                    true => row.get(10)?,
                },
                feature_title_rich: match fallback {
                    false => row.get(15)?,
                    true => None,
                },
                subtitle: match fallback {
                    false => row.get(16)?,
                    true => row.get(11)?,
                },
                subtitle_rich: match fallback {
                    false => row.get(17)?,
                    true => None,
                },
                feature_subtitle: match fallback {
                    false => row.get(18)?,
                    true => row.get(12)?,
                },
                feature_subtitle_rich: match fallback {
                    false => row.get(19)?,
                    true => None,
                },
                content: match fallback {
                    false => row.get(20)?,
                    true => row.get(13)?,
                },
                first_footnote_id: match fallback {
                    false => row.get(21)?,
                    true => row.get(14)?,
                },
                last_footnote_id: match fallback {
                    false => row.get(22)?,
                    true => row.get(15)?,
                },
                first_bible_citation_id: match fallback {
                    false => row.get(23)?,
                    true => row.get(16)?,
                },
                last_bible_citation_id: match fallback {
                    false => row.get(24)?,
                    true => row.get(17)?,
                },
                paragraph_count: match fallback {
                    false => row.get(25)?,
                    true => row.get(18)?,
                },
                has_media_links: match fallback {
                    false => row.get(26)?,
                    true => row.get(19)?,
                },
                has_links: match fallback {
                    false => row.get(27)?,
                    true => row.get(20)?,
                },
                first_page_number: match fallback {
                    false => row.get(28)?,
                    true => row.get(21)?,
                },
                last_page_number: match fallback {
                    false => row.get(29)?,
                    true => row.get(22)?,
                },
                content_length: match fallback {
                    false => row.get(30)?,
                    true => row.get(23)?,
                },
                preferred_presentation: match fallback {
                    false => row.get(31)?,
                    true => None,
                },
                content_reworked_date: match fallback {
                    false => row.get(32)?,
                    true => None,
                },
                has_pronunciation_guide: match fallback {
                    false => row.get(33)?,
                    true => false,
                }
            };

            documents.push(document);
        }
        
        Ok(documents)
    }

    pub fn get_document_content_by_id(&mut self, id: i32) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        let mut stmt = self.db.prepare("SELECT
            Content
        FROM Document WHERE DocumentId = ?1")?;
        
        let mut rows = stmt.query([id])?;

        let mut content = None;
        if let Some(row) = rows.next()? {
            content = row.get(0)?
        }
        
        Ok(content)
    }

    pub fn get_document_by_id(&mut self, id: i32) -> Result<Option<Document>, Box<dyn std::error::Error>> {
        let mut fallback = false;
        let mut stmt = match self.db.prepare("SELECT
            DocumentId,
            PublicationId,
            MepsDocumentId,
            MepsLanguageIndex,
            Class,
            Type,
            SectionNumber,
            ChapterNumber,
            Title,
            TitleRich,
            TocTitle,
            TocTitleRich,
            ContextTitle,
            ContextTitleRich,
            FeatureTitle,
            FeatureTitleRich,
            Subtitle,
            SubtitleRich,
            FeatureSubtitle,
            FeatureSubtitleRich,
            FirstFootnoteId,
            LastFootnoteId,
            FirstBibleCitationId,
            LastBibleCitationId,
            ParagraphCount,
            HasMediaLinks,
            HasLinks,
            FirstPageNumber,
            LastPageNumber,
            ContentLength,
            PreferredPresentation,
            ContentReworkedDate,
            HasPronunciationGuide
        FROM Document WHERE DocumentId = ?1") {
            Ok(stmt) => stmt,
            Err(_err) => {
                fallback = true;
                info!(
                    target: TARGET,
                    "Falling back to deprecated Document table."
                );
                self.db.prepare("SELECT
                    DocumentId,
                    PublicationId,
                    MepsDocumentId,
                    MepsLanguageIndex,
                    Class,
                    Type,
                    SectionNumber,
                    Title,
                    TocTitle,
                    ContextTitle,
                    FeatureTitle,
                    Subtitle,
                    FeatureSubtitle,
                    FirstFootnoteId,
                    LastFootnoteId,
                    FirstBibleCitationId,
                    LastBibleCitationId,
                    ParagraphCount,
                    HasMediaLinks,
                    HasLinks,
                    FirstPageNumber,
                    LastPageNumber,
                    ContentLength
                FROM Document WHERE DocumentId = ?1")?
            }
        };
        let mut rows = stmt.query([id])?;

        let mut document = None;

        if let Some(row) = rows.next()? {
            document = Some(Document {
                id: row.get(0)?,
                publication_id: row.get(1)?,
                meps_document_id: row.get(2)?,
                meps_language_id: row.get(3)?,
                class: row.get(4)?,
                type_id: row.get(5)?,
                section_number: row.get(6)?,
                chapter_number: match fallback {
                    false => row.get(7)?,
                    true => None,
                },
                title: match fallback {
                    false => row.get(8)?,
                    true => row.get(7)?,
                },
                title_rich: match fallback {
                    false => row.get(9)?,
                    true => None,
                },
                toc_title: match fallback {
                    false => row.get(10)?,
                    true => row.get(8)?,
                },
                toc_title_rich: match fallback {
                    false => row.get(11)?,
                    true => None,
                },
                context_title: match fallback {
                    false => row.get(12)?,
                    true => row.get(9)?,
                },
                context_title_rich: match fallback {
                    false => row.get(13)?,
                    true => None,
                },
                feature_title: match fallback {
                    false => row.get(14)?,
                    true => row.get(10)?,
                },
                feature_title_rich: match fallback {
                    false => row.get(15)?,
                    true => None,
                },
                subtitle: match fallback {
                    false => row.get(16)?,
                    true => row.get(11)?,
                },
                subtitle_rich: match fallback {
                    false => row.get(17)?,
                    true => None,
                },
                feature_subtitle: match fallback {
                    false => row.get(18)?,
                    true => row.get(12)?,
                },
                feature_subtitle_rich: match fallback {
                    false => row.get(19)?,
                    true => None,
                },
                content: None,
                first_footnote_id: match fallback {
                    false => row.get(20)?,
                    true => row.get(13)?,
                },
                last_footnote_id: match fallback {
                    false => row.get(21)?,
                    true => row.get(14)?,
                },
                first_bible_citation_id: match fallback {
                    false => row.get(22)?,
                    true => row.get(15)?,
                },
                last_bible_citation_id: match fallback {
                    false => row.get(23)?,
                    true => row.get(16)?,
                },
                paragraph_count: match fallback {
                    false => row.get(24)?,
                    true => row.get(17)?,
                },
                has_media_links: match fallback {
                    false => row.get(25)?,
                    true => row.get(18)?,
                },
                has_links: match fallback {
                    false => row.get(26)?,
                    true => row.get(19)?,
                },
                first_page_number: match fallback {
                    false => row.get(27)?,
                    true => row.get(20)?,
                },
                last_page_number: match fallback {
                    false => row.get(28)?,
                    true => row.get(21)?,
                },
                content_length: match fallback {
                    false => row.get(29)?,
                    true => row.get(22)?,
                },
                preferred_presentation: match fallback {
                    false => row.get(30)?,
                    true => None,
                },
                content_reworked_date: match fallback {
                    false => row.get(31)?,
                    true => None,
                },
                has_pronunciation_guide: match fallback {
                    false => row.get(32)?,
                    true => false,
                }
            });
        }
        Ok(document)
    }

    pub fn get_dated_texts(&mut self) -> Result<Vec<DatedText>, Box<dyn std::error::Error>> {
        let mut fallback = false;
        let mut stmt = match self.db.prepare("SELECT
            DatedTextId,
            DocumentId,
            Link,
            FirstDateOffset,
            LastDateOffset,
            FirstFootnoteId,
            LastFootnoteId,
            FirstBibleCitationId,
            LastBibleCitationId,
            BeginParagraphOrdinal,
            EndParagraphOrdinal,
            Caption,
            CaptionRich,
            Content
        FROM DatedText") {
            Ok(stmt) => stmt,
            Err(_e) => {
                fallback = true;
                info!(
                    target: TARGET,
                    "Falling back to deprecated DatedText table."
                );
                self.db.prepare("SELECT
                    DatedTextId,
                    DocumentId,
                    Link,
                    FirstDateOffset,
                    LastDateOffset,
                    FirstFootnoteId,
                    LastFootnoteId,
                    FirstBibleCitationId,
                    LastBibleCitationId,
                    Caption,
                    Content
                FROM DatedText")?
            }
        };
        let mut rows = stmt.query([])?;

        let mut dated_texts = vec![];
        while let Some(row) = rows.next()? {
            let dated_text = DatedText {
                id: row.get(0)?,
                document_id: row.get(1)?,
                link: row.get(2)?,
                first_date_offset: row.get(3)?,
                last_date_offset: row.get(4)?,
                first_footnote_id: row.get(5)?,
                last_footnote_id: row.get(6)?,
                first_bible_citation_id: row.get(7)?,
                last_bible_citation_id: row.get(8)?,
                begin_paragraph_ordinal: match fallback {
                    false => row.get(9)?,
                    true => 0
                },
                end_paragraph_ordinal: match fallback {
                    false => row.get(10)?,
                    true => 0
                },
                caption: match fallback {
                    false => row.get(11)?,
                    true => row.get(9)?
                },
                caption_rich: match fallback {
                    false => row.get(12)?,
                    true => None
                },
                content: match fallback {
                    false => row.get(13)?,
                    true => row.get(10)?
                },
            };

            dated_texts.push(dated_text);
        }

        Ok(dated_texts)
    }

    // This function is quite inefficient in terms of memory, since it
    // save a Document on frontend and backend, but anyway the `LruCache`
    // save some processing power, especially when we need to go to the
    // next or previous chapter multiple times.
    pub fn get_content_text_from(&mut self, content_table: ContentTables, id: i32) -> Result<Option<String>, Box<dyn std::error::Error>>{
        if let Some(content) = self.decrypted_content_cache.get(&(content_table, id)) {
            return Ok(Some(content.clone()))
        }

        match content_table {
            ContentTables::Document => {
                if let Some(content) = self.get_document_content_by_id(id)? {
                    let content = self.decrypt_content(content)?;
                    self.decrypted_content_cache.put((ContentTables::Document, id), content.clone());
                    Ok(Some(content))
                } else {
                    Ok(None)
                }
            }
            _ => Err("Unsupported content table or not implemented yet".into())
        }
    }

    pub fn decrypt_content(&self, content: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        let (key, iv) = self.master_key.split_at(16);
        let mut decryptor = Decryptor::<Aes128>::new(key.into(), iv.into());

        let block_size = Aes128::block_size();
        let mut blocks: Vec<GenericArray<u8, typenum::U16>> = content
            .chunks_exact(block_size)
            .map(|chunk| GenericArray::clone_from_slice(chunk))
            .collect();

        decryptor.decrypt_blocks_mut(&mut blocks);

        let mut decrypted_data: Vec<u8> = Vec::with_capacity(content.len());
        for block in blocks {
            decrypted_data.extend_from_slice(&block);
        }

        let inflated_content = inflate_bytes_zlib(&decrypted_data)?;
        Ok(String::from_utf8_lossy(&inflated_content).to_string())
    }
}
