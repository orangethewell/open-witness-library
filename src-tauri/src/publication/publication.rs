use std::{num::NonZero, path::PathBuf};

use cbc::Decryptor;
use aes::{cipher::{generic_array::GenericArray, BlockDecryptMut, BlockSizeUser, KeyIvInit}, Aes128};
use inflate::inflate_bytes_zlib;
use lru::LruCache;
use rusqlite::Connection;
use sha2::{Sha256, Digest};

use super::tables::*;

#[derive(Hash, Eq, PartialEq)]
pub enum ContentTables {
    Document
}

pub struct Publication {
    db: Connection,
    master_key: Vec<u8>,
    decrypted_content_cache: LruCache<(ContentTables, i32), String>
}

impl Publication {
    pub fn from_path<'a>(pub_path: PathBuf, name: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        let db = Connection::open(pub_path.join(format!("{}.db", name)))?;
        let mut master_key = vec![];

        // Forge master key for document decrypting
        {
            let mut stmt = db
                .prepare("SELECT MepsLanguageIndex, Symbol, Year, IssueTagNumber FROM Publication")?;

            let (meps_language_index, 
                symbol, 
                year, 
                issue_tag_number
            ) = stmt.query_row([1], |row| Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
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

        Ok(Self {
            db,
            master_key,
            decrypted_content_cache: LruCache::new(NonZero::new(5).unwrap())
        })
    }

    pub fn get_view_items(&self) -> Result<Vec<PublicationViewItem>, Box<dyn std::error::Error>> {
        let mut stmt = self.db.prepare("SELECT * FROM PublicationViewItem")?;
        let mut rows = stmt.query([])?;

        let mut view_items = vec![];

        while let Some(row) = rows.next()? {
            let item = PublicationViewItem {
                id: row.get(0)?,
                publication_view_id: row.get(1)?,
                parent_publication_view_item_id: row.get(2)?,
                title: row.get(3)?,
                title_rich: row.get(4)?,
                schema_type: row.get(5)?,
                child_template_schema_type: row.get(6)?,
                default_document_id: row.get(7)?,
            };

            view_items.push(item);
        }

        Ok(view_items)
    }

    pub fn get_view_items_documents(&self) -> Result<Vec<PublicationViewItemDocument>, Box<dyn std::error::Error>> {
        let mut stmt = self.db.prepare("SELECT * FROM PublicationViewItemDocument")?;
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

    pub fn get_document_by_id(&mut self, id: i32) -> Result<Option<Document>, Box<dyn std::error::Error>> {
        let mut stmt = self.db.prepare("SELECT * FROM Document WHERE DocumentId =?")?;
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
                chapter_number: row.get(7)?,
                title: row.get(8)?,
                title_rich: row.get(9)?,
                toc_title: row.get(10)?,
                toc_title_rich: row.get(11)?,
                context_title: row.get(12)?,
                context_title_rich: row.get(13)?,
                feature_title: row.get(14)?,
                feature_title_rich: row.get(15)?,
                subtitle: row.get(16)?,
                subtitle_rich: row.get(17)?,
                feature_subtitle: row.get(18)?,
                feature_subtitle_rich: row.get(19)?,
                content: row.get(20)?,
                first_footnote_id: row.get(21)?,
                last_footnote_id: row.get(22)?,
                first_bible_citation_id: row.get(23)?,
                last_bible_citation_id: row.get(24)?,
                paragraph_count: row.get(25)?,
                has_media_links: row.get(26)?,
                has_links: row.get(27)?,
                first_page_number: row.get(28)?,
                last_page_number: row.get(29)?,
                content_length: row.get(30)?,
                preferred_presentation: row.get(31)?,
                content_reworked_date: row.get(32)?,
                has_pronunciation_guide: row.get(33)?
            });
        }
        Ok(document)
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
