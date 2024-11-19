use std::{
    fs,
    io::{self, Cursor, Read},
    num::NonZero,
    path::PathBuf,
};

use chrono::NaiveDateTime;
use colored::Colorize;
use lru::LruCache;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::utils::unpack_zip;

use super::{
    manifest::{Image, IssueProperties},
    Manifest, Publication,
};

const TARGET: &'static str = "catalog";

pub struct Catalog {
    pub_path: PathBuf,
    catalog_db: Connection,

    current_open: String,
    publication_cache: LruCache<String, Publication>,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionPublication {
    pub id: i32,

    pub language_index: i32,
    pub publication_type: String,
    pub publication_category_symbol: String,

    pub title: String,
    pub short_title: String,
    pub display_title: String,
    pub symbol: String,
    pub unique_english_symbol: String,

    pub year: i32,
    pub volume_number: i32,
    pub issue_tag_number: Option<i32>,

    pub first_dated_text_date_offset: i32,
    pub last_dated_text_date_offset: i32,

    pub root_symbol: String,
    pub root_year: i32,
    pub root_meps_language_index: i32,

    pub version_number: i32,
    pub schema_version_number: i32,

    pub hash: String,
    pub timestamp: String,

    pub jwpub: String,

    pub database_path: String,
    pub on_external_storage: String,

    pub undated_reference_title: String,
    pub expanded_size: i32,

    pub min_platform_version: i32,
    pub key_symbol: String,
    pub meps_build_number: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionImage {
    pub id: i32,
    pub publication_id: i32,
    pub image_type: String,
    pub attribute: String,
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub signature: String,
}

impl Catalog {
    pub fn init<T: Into<PathBuf>>(location: T) -> Result<Self, Box<dyn std::error::Error>> {
        debug!(target: TARGET, "Initializing catalog...");
        let location: PathBuf = location.into();
        if !location.exists() {
            fs_extra::dir::create_all(&location, false)?;
            info!(target: TARGET, "Creating catalog location...");
        }

        let db = Connection::open(location.join("collections.db"))?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "Publication".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS Publication (
                PublicationId INTEGER PRIMARY KEY AUTOINCREMENT,
                
                LanguageIndex INTEGER NOT NULL,
                PublicationType TEXT NOT NULL,
                PublicationCategorySymbol TEXT NOT NULL,
                
                Title TEXT NOT NULL,
                ShortTitle TEXT NOT NULL,
                DisplayTitle TEXT NOT NULL,
                
                Symbol TEXT NOT NULL,
                UniqueEnglishSymbol TEXT NOT NULL,
                
                Year INTEGER NOT NULL,
                VolumeNumber INTEGER NOT NULL,
                IssueTagNumber INTEGER,
                FirstDatedTextDateOffset INTEGER NOT NULL,
                LastDatedTextDateOffset INTEGER NOT NULL,
                
                RootSymbol TEXT NOT NULL,
                RootYear INTEGER NOT NULL,
                RootMepsLanguageIndex INTEGER NOT NULL,
                
                VersionNumber INTEGER NOT NULL,
                SchemaVersionNumber INTEGER NOT NULL,
                Hash TEXT NOT NULL,
                Timestamp TEXT NOT NULL,
                
                JwPub TEXT NOT NULL,
                DatabasePath TEXT NOT NULL,
                
                OnExternalStorage TEXT NOT NULL,
                UndatedReferenceTitle TEXT NOT NULL,
                
                ExpandedSize INTEGER NOT NULL,
                MinPlatformVersion INTEGER NOT NULL,
                KeySymbol TEXT NOT NULL,
                MepsBuildNumber INTEGER NOT NULL,

                UNIQUE(JwPub)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PublicationAttribute".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PublicationAttribute (
                PublicationAttributeId INTEGER PRIMARY KEY AUTOINCREMENT,
                PublicationId INTEGER NOT NULL,
                Attribute TEXT NOT NULL,
                
                UNIQUE(PublicationId,Attribute),
                FOREIGN KEY (PublicationId) REFERENCES Publication(PublicationId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PublicationIssueAttribute".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PublicationIssueAttribute (
                PublicationIssueAttributeId INTEGER PRIMARY KEY AUTOINCREMENT,
                PublicationId INTEGER NOT NULL,
                Attribute TEXT NOT NULL,
                
                UNIQUE(PublicationId,Attribute)
                FOREIGN KEY (PublicationId) REFERENCES Publication(PublicationId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PublicationIssueProperty".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PublicationIssueProperty (
                PublicationIssuePropertyId INTEGER PRIMARY KEY AUTOINCREMENT,
                PublicationId INTEGER NOT NULL,
                
                Title TEXT NOT NULL,
                UndatedTitle TEXT NOT NULL,
                CoverTitle TEXT NOT NULL,
                
                Symbol TEXT NOT NULL,
                UndatedSymbol TEXT NOT NULL,
                
                UNIQUE(PublicationId,Symbol)
                FOREIGN KEY (PublicationId) REFERENCES Publication(PublicationId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "Image".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS Image (
                ImageId INTEGER PRIMARY KEY AUTOINCREMENT,
                PublicationId INTEGER NOT NULL,
                Type TEXT NOT NULL,
                Attribute TEXT NOT NULL,

                Path TEXT NOT NULL,
                Width INTEGER NOT NULL,
                Height INTEGER NOT NULL,

                Signature TEXT NOT NULL,
                
                UNIQUE(PublicationId,Signature)
                FOREIGN KEY (PublicationId) REFERENCES Publication(PublicationId)
            )",
            (),
        )?;

        debug!(target: TARGET, "initializing \"{}\" table...", "Document".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS Document (
                LanguageIndex INTEGER NOT NULL,
                MepsDocumentId INTEGER NOT NULL,
                PublicationId INTEGER NOT NULL,

                UNIQUE(PublicationId,MepsDocumentId)
                FOREIGN KEY (PublicationId) REFERENCES Publication(PublicationId)
            )",
            (),
        )?;

        debug!(target: TARGET, "initializing \"{}\" table...", "DatedText".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS DatedText (
                DatedTextId INTEGER PRIMARY KEY AUTOINCREMENT,
                PublicationId INTEGER NOT NULL,
                Start INTEGER NOT NULL,
                End INTEGER NOT NULL,
                Class INTEGER NOT NULL,

                UNIQUE(PublicationId,Start,End)
                FOREIGN KEY (PublicationId) REFERENCES Publication(PublicationId)
            )",
            (),
        )?;

        debug!(target: TARGET, "initializing \"{}\" table...", "AvailableBibleBook".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS AvailableBibleBook (
                Id INTEGER PRIMARY KEY AUTOINCREMENT,
                PublicationId INTEGER NOT NULL,
                Book TEXT NOT NULL,

                UNIQUE(PublicationId,Book)
                FOREIGN KEY (PublicationId) REFERENCES Publication(PublicationId)
            )",
            (),
        )?;

        debug!(target: TARGET, "Catalog initialized at {}!", location.display().to_string().green());

        Ok(Self {
            pub_path: location,
            catalog_db: db,
            current_open: String::new(),
            publication_cache: LruCache::new(NonZero::new(5).unwrap()),
        })
    }

    pub fn update_metadata_for_publication(
        &mut self,
        id: i64,
        pub_manifest: &Manifest,
        first_dated_text_offset: Option<i32>,
        last_dated_text_offset: Option<i32>,
        database_path: String,
        on_external_storage: Option<i32>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "UPDATE Publication SET
                LanguageIndex = ?1,
                PublicationType = ?2,
                PublicationCategorySymbol = ?3,
                Title = ?4,
                ShortTitle = ?5,
                DisplayTitle = ?6,
                Symbol = ?7,
                UniqueEnglishSymbol = ?8,
                Year = ?9,
                VolumeNumber = ?10,
                IssueTagNumber = ?11,
                FirstDatedTextDateOffset = ?12,
                LastDatedTextDateOffset = ?13,
                RootSymbol = ?14,
                RootYear = ?15,
                RootMepsLanguageIndex = ?16,
                VersionNumber = ?17,
                SchemaVersionNumber = ?18,
                Hash = ?19,
                Timestamp = ?20,
                JwPub = ?21,
                DatabasePath = ?22,
                OnExternalStorage = ?23,
                UndatedReferenceTitle = ?24,
                ExpandedSize = ?25,
                MinPlatformVersion = ?26,
                KeySymbol = ?27,
                MepsBuildNumber = ?28
            WHERE PublicationId=?29",
            params![
                &pub_manifest.publication.language,
                &pub_manifest.publication.publication_type,
                {
                    if pub_manifest.publication.categories.len() > 1 {
                        "Unknown"
                    } else {
                        &pub_manifest.publication.categories[0]
                    }
                },
                &pub_manifest.publication.title,
                &pub_manifest.publication.short_title,
                &pub_manifest.publication.display_title,
                &pub_manifest.publication.symbol,
                &pub_manifest.publication.unique_english_symbol,
                &pub_manifest.publication.year,
                0, // Volume number
                &pub_manifest.publication.issue_number,
                first_dated_text_offset.unwrap_or(19691231),
                last_dated_text_offset.unwrap_or(19691231),
                &pub_manifest.publication.root_symbol,
                &pub_manifest.publication.root_year,
                &pub_manifest.publication.root_language,
                &pub_manifest.publication.schema_version,
                &pub_manifest.publication.schema_version,
                &pub_manifest.hash,
                &pub_manifest.timestamp,
                &pub_manifest.name,
                &database_path,
                on_external_storage.unwrap_or(0),
                &pub_manifest.publication.undated_reference_title,
                &pub_manifest.expanded_size,
                &pub_manifest.publication.min_platform_version,
                &pub_manifest.publication.undated_symbol,
                &pub_manifest.meps_build_number,
                id
            ],
        )?;

        let pub_id = self.catalog_db.last_insert_rowid();

        debug!(target: TARGET, "Metadata updated on collection database for ID {}!", pub_id);

        Ok(pub_id)
    }

    pub fn insert_metadata_for_publication(
        &mut self,
        pub_manifest: &Manifest,
        first_dated_text_offset: Option<i32>,
        last_dated_text_offset: Option<i32>,
        database_path: String,
        on_external_storage: Option<i32>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "INSERT INTO Publication (
                LanguageIndex,
                PublicationType,
                PublicationCategorySymbol,
                Title,
                ShortTitle,
                DisplayTitle,
                Symbol,
                UniqueEnglishSymbol,
                Year,
                VolumeNumber,
                IssueTagNumber,
                FirstDatedTextDateOffset,
                LastDatedTextDateOffset,
                RootSymbol,
                RootYear,
                RootMepsLanguageIndex,
                VersionNumber,
                SchemaVersionNumber,
                Hash,
                Timestamp,
                JwPub,
                DatabasePath,
                OnExternalStorage,
                UndatedReferenceTitle,
                ExpandedSize,
                MinPlatformVersion,
                KeySymbol,
                MepsBuildNumber
            ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28)",
            params![
                &pub_manifest.publication.language,
                &pub_manifest.publication.publication_type,
                {if pub_manifest.publication.categories.len() > 1 {
                    "Unknown"
                } else {
                    &pub_manifest.publication.categories[0]
                }},

                &pub_manifest.publication.title,
                &pub_manifest.publication.short_title,
                &pub_manifest.publication.display_title,

                &pub_manifest.publication.symbol,
                &pub_manifest.publication.unique_english_symbol,

                &pub_manifest.publication.year,
                0, // Volume number
                &pub_manifest.publication.issue_number,
                first_dated_text_offset.unwrap_or(19691231),
                last_dated_text_offset.unwrap_or(19691231),

                &pub_manifest.publication.root_symbol,
                &pub_manifest.publication.root_year,
                &pub_manifest.publication.root_language,

                &pub_manifest.publication.schema_version,
                &pub_manifest.publication.schema_version,

                &pub_manifest.hash,
                &pub_manifest.timestamp,
                &pub_manifest.name,

                &database_path,
                on_external_storage.unwrap_or(0),
                &pub_manifest.publication.undated_reference_title,

                &pub_manifest.expanded_size,
                &pub_manifest.publication.min_platform_version,
                &pub_manifest.publication.undated_symbol,
                &pub_manifest.meps_build_number,
            ]
        )?;

        let pub_id = self.catalog_db.last_insert_rowid();

        debug!(target: TARGET, "Metadata inserted to collection database for ID {}!", pub_id);

        Ok(pub_id)
    }

    pub fn delete_attribute_for_publication<'a>(
        &mut self,
        id: i64,
        attribute: &'a str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "DELETE FROM PublicationAttribute WHERE 
                PublicationId=?1 AND
                Attribute=?2",
            params![id, attribute],
        )?;

        debug!(target: TARGET, "Attribute \"{}\" deleted for publication ID {}.", attribute.bright_yellow(), id.to_string().bold());

        Ok(())
    }

    pub fn insert_attribute_for_publication<'a>(
        &mut self,
        id: i64,
        attribute: &'a str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "INSERT INTO PublicationAttribute (
                PublicationId,
                Attribute
            ) VALUES (?1,?2)",
            params![id, attribute],
        )?;

        debug!(target: TARGET, "Attribute \"{}\" added to publication ID {}!", attribute.bright_yellow(), id.to_string().bold());

        Ok(())
    }

    pub fn delete_issue_attribute_for_publication<'a>(
        &mut self,
        id: i64,
        attribute: &'a str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "DELETE FROM PublicationIssueAttribute WHERE
                PublicationId=?1 AND
                Attribute=?2",
            params![id, attribute],
        )?;

        debug!(target: TARGET, "Issue attribute \"{}\" deleted from publication ID {}.", attribute.bright_yellow(), id.to_string().bold());

        Ok(())
    }

    pub fn insert_issue_attribute_for_publication<'a>(
        &mut self,
        id: i64,
        attribute: &'a str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "INSERT INTO PublicationIssueAttribute (
                PublicationId,
                Attribute
            ) VALUES (?1,?2)",
            params![id, attribute],
        )?;

        debug!(target: TARGET, "Issue attribute \"{}\" added to publication ID {}!", attribute.bright_yellow(), id.to_string().bold());

        Ok(())
    }

    pub fn delete_issue_property_for_publication(
        &mut self,
        id: i64,
        property: &IssueProperties,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "DELETE FROM PublicationIssueProperty WHERE
                PublicationId=?1 AND
                Symbol=?2",
            params![id, &property.symbol,],
        )?;

        debug!(target: TARGET, "Issue properties deleted from publication ID {}.", id.to_string().bold());

        Ok(())
    }

    pub fn insert_issue_property_for_publication(
        &mut self,
        id: i64,
        property: &IssueProperties,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "INSERT INTO PublicationIssueProperty (
                PublicationId,
                Title,
                UndatedTitle,
                CoverTitle,
                Symbol,
                UndatedSymbol
            ) VALUES (?1,?2,?3,?4,?5,?6)",
            params![
                id,
                &property.title,
                &property.undated_title,
                &property.cover_title,
                &property.symbol,
                &property.undated_symbol,
            ],
        )?;

        debug!(target: TARGET, "Issue properties with cover title \"{}\" added to publication ID {}!", property.cover_title.bright_yellow(), id.to_string().bold());

        Ok(())
    }

    pub fn delete_image_for_publication(
        &mut self,
        id: i64,
        image: &Image,
        path: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "DELETE FROM Image WHERE
                PublicationId=?1 AND
                Signature=?2",
            params![id, &image.signature,],
        )?;

        debug!(target: TARGET, "Media path \"{}\" removed from Image table: publication ID {}.", path.bright_yellow(), id.to_string().bold());

        Ok(())
    }

    pub fn insert_image_for_publication(
        &mut self,
        id: i64,
        image: &Image,
        path: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "INSERT INTO Image (
                PublicationId,
                Type,
                Attribute,
                Path,
                Width,
                Height,
                Signature
            ) VALUES (?1,?2,?3,?4,?5,?6,?7)",
            params![
                id,
                &image.image_type,
                &image.attribute,
                &path,
                image.width,
                image.height,
                &image.signature,
            ],
        )?;

        debug!(target: TARGET, "Media path \"{}\" added to Image table: publication ID {}!", path.bright_yellow(), id.to_string().bold());

        Ok(())
    }

    pub fn remove_indexed_dated_texts(
        &mut self,
        publication: &mut Publication,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dated_texts = publication.get_dated_texts()?;

        for dated_text in dated_texts.iter() {
            let document = publication
                .get_document_by_id(dated_text.document_id)?
                .unwrap_or_default();

            debug!(
                target: TARGET,
                "Removing indexed dated text (Start: \"{}\"; End: \"{}\") for publication ID {}... (Class {})",
                dated_text.first_date_offset.to_string().bright_blue(),
                dated_text.last_date_offset.to_string().bright_blue(),
                publication.catalog_id.to_string().bold(),
                document.class.to_string().yellow()
            );

            self.catalog_db.execute(
                "DELETE FROM DatedText WHERE
                    PublicationId=?1 AND
                    Start=?2 AND
                    End=?3 AND
                    Class=?4",
                params![
                    publication.catalog_id,
                    dated_text.first_date_offset,
                    dated_text.last_date_offset,
                    document.class
                ],
            )?;
        }

        Ok(())
    }

    fn index_dated_texts(
        &mut self,
        publication: &mut Publication,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dated_texts = publication.get_dated_texts()?;

        for dated_text in dated_texts.iter() {
            let document = publication
                .get_document_by_id(dated_text.document_id)?
                .unwrap_or_default();

            debug!(
                target: TARGET,
                "Indexing dated text (Start: \"{}\"; End: \"{}\") for publication ID {}... (Class {})",
                dated_text.first_date_offset.to_string().bright_blue(),
                dated_text.last_date_offset.to_string().bright_blue(),
                publication.catalog_id.to_string().bold(),
                document.class.to_string().yellow()
            );

            self.catalog_db.execute(
                "INSERT INTO DatedText (
                    PublicationId,
                    Start,
                    End,
                    Class
                ) VALUES (?1,?2,?3,?4)",
                params![
                    publication.catalog_id,
                    dated_text.first_date_offset,
                    dated_text.last_date_offset,
                    document.class
                ],
            )?;
        }

        Ok(())
    }

    fn remove_indexed_documents(
        &mut self,
        publication: &mut Publication,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let documents = publication.get_documents()?;

        for document in documents.iter() {
            debug!(
                target: TARGET,
                "Removing indexed document (MepsDocumentId: \"{}\") for publication ID {}... (Lang {})",
                document.meps_document_id.to_string().bright_blue(),
                publication.catalog_id.to_string().bold(),
                document.meps_language_id.to_string().yellow()
            );

            self.catalog_db.execute(
                "DELETE FROM Document WHERE
                    LanguageIndex=?1 AND
                    MepsDocumentId=?2 AND
                    PublicationId=?3",
                params![
                    document.meps_language_id,
                    document.meps_document_id,
                    publication.catalog_id
                ],
            )?;
        }

        Ok(())
    }

    fn index_documents(
        &mut self,
        publication: &mut Publication,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let documents = publication.get_documents()?;

        for document in documents.iter() {
            debug!(
                target: TARGET,
                "Indexing document (MepsDocumentId: \"{}\") for publication ID {}... (Lang {})",
                document.meps_document_id.to_string().bright_blue(),
                publication.catalog_id.to_string().bold(),
                document.meps_language_id.to_string().yellow()
            );

            self.catalog_db.execute(
                "INSERT INTO Document (
                    LanguageIndex,
                    MepsDocumentId,
                    PublicationId
                ) VALUES (?1,?2,?3)",
                params![
                    document.meps_language_id,
                    document.meps_document_id,
                    publication.catalog_id
                ],
            )?;
        }

        Ok(())
    }

    pub fn install_jwpub_file<T: Into<PathBuf>>(
        &mut self,
        file_path: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path: PathBuf = file_path.into();

        info!(target: TARGET, "Installing {}...", file_path.display().to_string().bright_magenta());

        let package = fs::File::open(&file_path)?;
        let reader = io::BufReader::new(package);
        let mut package = ZipArchive::new(reader)?;

        debug!(target: TARGET, "Checking if file is a valid JWPUB...");
        let manifest = get_metadata_from_archive(&mut package)?;
        let pub_pathname = manifest.name.replace(".jwpub", "");

        debug!(target: TARGET, "Checking if JWPUB doesn't match with any publication installed...");
        let mut existing_id = None;
        if let Some(publication_data) =
            self.get_publication_collection_meta(&manifest.name.replace(".jwpub", ""))?
        {
            let new_timestamp =
                NaiveDateTime::parse_from_str(&manifest.timestamp, "%Y-%m-%dT%H:%M:%SZ")?;
            let cur_timestamp =
                NaiveDateTime::parse_from_str(&publication_data.timestamp, "%Y-%m-%dT%H:%M:%SZ")?;

            existing_id = Some(publication_data.id);

            if cur_timestamp >= new_timestamp {
                error!(target: TARGET, "Publication {} is newer or the same than the installed version.", manifest.name.replace(".jwpub", ""));
                return Err(format!(
                    "Publication {} is newer or the same than the installed version.",
                    manifest.name.replace(".jwpub", "")
                )
                .into());
            }
        }

        debug!(target: TARGET, "Configuring directory...");
        let location = self.pub_path.join(pub_pathname);
        if !location.exists() {
            fs_extra::dir::create_all(&location, false)?;
        }
        info!(target: TARGET, "Installing {} at {}...", file_path.display().to_string().bright_magenta(), location.display().to_string().bright_magenta());
        let mut content_file = package.by_name("contents")?;
        let mut content_data = Vec::<u8>::new();
        content_file.read_to_end(&mut content_data)?;
        let content_package = ZipArchive::new(Cursor::new(content_data))?;

        debug!(target: TARGET, "Extracting contents...");
        unpack_zip(content_package, &location);

        debug!(target: TARGET, "Copying manifest.json...");
        let manifest_file = fs::File::create(location.join("manifest.json"))?;
        serde_json::to_writer_pretty(manifest_file, &manifest)?;

        info!(target: TARGET, "Indexing data to catalog...");
        let mut tmp_publication =
            Publication::from_database(location.join(&manifest.publication.file_name), -1)?;
        let mut first_dated_text_offset = None;
        let mut last_dated_text_offset = None;

        let dated_texts = tmp_publication.get_dated_texts()?;
        if dated_texts.len() > 0 {
            first_dated_text_offset = Some(dated_texts[0].first_date_offset);
            last_dated_text_offset = Some(dated_texts[dated_texts.len() - 1].last_date_offset);
        }

        let publication_id = if let Some(id) = existing_id {
            self.update_metadata_for_publication(
                id as i64,
                &manifest,
                first_dated_text_offset,
                last_dated_text_offset,
                location
                    .join(&manifest.publication.file_name)
                    .to_str()
                    .unwrap()
                    .to_owned(),
                None,
            )?;
            id as i64
        } else {
            self.insert_metadata_for_publication(
                &manifest,
                first_dated_text_offset,
                last_dated_text_offset,
                location
                    .join(&manifest.publication.file_name)
                    .to_str()
                    .unwrap()
                    .to_owned(),
                None,
            )?
        };

        tmp_publication.catalog_id = publication_id;

        if manifest.publication.attributes.len() > 0 {
            for attribute in manifest.publication.attributes.iter() {
                if existing_id.is_some() {
                    self.delete_attribute_for_publication(tmp_publication.catalog_id, attribute)?;
                }
                self.insert_attribute_for_publication(tmp_publication.catalog_id, attribute)?;
            }
        }

        if manifest.publication.issue_attributes.len() > 0 {
            for issue_attribute in manifest.publication.issue_attributes.iter() {
                if existing_id.is_some() {
                    self.delete_issue_attribute_for_publication(
                        tmp_publication.catalog_id,
                        issue_attribute,
                    )?;
                }
                self.insert_issue_attribute_for_publication(
                    tmp_publication.catalog_id,
                    issue_attribute,
                )?;
            }
        }

        if !manifest.publication.issue_properties.symbol.is_empty() {
            if existing_id.is_some() {
                self.delete_issue_property_for_publication(
                    tmp_publication.catalog_id,
                    &manifest.publication.issue_properties,
                )?;
            }
            self.insert_issue_property_for_publication(
                tmp_publication.catalog_id,
                &manifest.publication.issue_properties,
            )?;
        }

        if manifest.publication.images.len() > 0 {
            for image in manifest.publication.images.iter() {
                if existing_id.is_some() {
                    self.delete_image_for_publication(
                        tmp_publication.catalog_id,
                        image,
                        location.join(&image.file_name).to_str().unwrap().to_owned(),
                    )?;
                }
                self.insert_image_for_publication(
                    tmp_publication.catalog_id,
                    image,
                    location.join(&image.file_name).to_str().unwrap().to_owned(),
                )?;
            }
        }

        if dated_texts.len() > 0 {
            if existing_id.is_some() {
                self.remove_indexed_dated_texts(&mut tmp_publication)?;
            }
            self.index_dated_texts(&mut tmp_publication)?;
        }

        if existing_id.is_some() {
            self.remove_indexed_documents(&mut tmp_publication)?;
        }
        self.index_documents(&mut tmp_publication)?;

        info!(
            target: TARGET,
            "publication ID {} installed at {}!",
            tmp_publication.catalog_id.to_string().bold(),
            location.display().to_string().green()
        );

        Ok(())
    }

    pub fn get_images_of_type<'a>(
        &self,
        image_type: &'a str,
        publication_id: i64,
    ) -> Result<Vec<CollectionImage>, Box<dyn std::error::Error>> {
        let mut stmt = self.catalog_db.prepare(
            "SELECT
                    ImageId,
                    PublicationId,
                    Type,
                    Attribute,
                    Path,
                    Width,
                    Height,
                    Signature
                FROM Image
            WHERE PublicationId =?1 AND Type =?2",
        )?;
        let mut rows = stmt.query(params![publication_id, image_type])?;
        let mut images = Vec::new();
        while let Some(row) = rows.next()? {
            let image = CollectionImage {
                id: row.get(0)?,
                publication_id: row.get(1)?,
                image_type: row.get(2)?,
                attribute: row.get(3)?,
                path: row.get(4)?,
                width: row.get(5)?,
                height: row.get(6)?,
                signature: row.get(7)?,
            };

            images.push(image);
        }

        Ok(images)
    }

    pub fn get_count_of_type(
        &self,
        publication_type: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut stmt = self.catalog_db.prepare(
            "SELECT
                COUNT(PublicationId)
            FROM Publication WHERE PublicationType=?1",
        )?;

        Ok(stmt.query_row([publication_type], |row| Ok(row.get(0)?))?)
    }

    pub fn get_list_from_type(
        &self,
        publication_type: &str,
    ) -> Result<Vec<CollectionPublication>, Box<dyn std::error::Error>> {
        let mut stmt = self.catalog_db.prepare(
            "SELECT
                PublicationId,
                LanguageIndex,
                PublicationType,
                PublicationCategorySymbol,
                
                Title,
                ShortTitle,
                DisplayTitle,

                Symbol,
                UniqueEnglishSymbol,
                Year,
                VolumeNumber,
                IssueTagNumber,
                FirstDatedTextDateOffset,
                LastDatedTextDateOffset,
                
                RootSymbol,
                RootYear,
                RootMepsLanguageIndex,
                
                VersionNumber,
                SchemaVersionNumber,
                Hash,
                Timestamp,
                
                JwPub,
                DatabasePath,
                
                OnExternalStorage,
                UndatedReferenceTitle,
                
                ExpandedSize,
                MinPlatformVersion,
                KeySymbol,
                MepsBuildNumber
            FROM Publication WHERE PublicationType=?1",
        )?;
        let mut rows = stmt.query([publication_type])?;
        let mut pub_collection = Vec::new();
        while let Some(row) = rows.next()? {
            pub_collection.push(CollectionPublication {
                id: row.get(0)?,
                language_index: row.get(1)?,
                publication_type: row.get(2)?,
                publication_category_symbol: row.get(3)?,
                title: row.get(4)?,
                short_title: row.get(5)?,
                display_title: row.get(6)?,
                symbol: row.get(7)?,
                unique_english_symbol: row.get(8)?,
                year: row.get(9)?,
                volume_number: row.get(10)?,
                issue_tag_number: row.get(11)?,
                first_dated_text_date_offset: row.get(12)?,
                last_dated_text_date_offset: row.get(13)?,
                root_symbol: row.get(14)?,
                root_year: row.get(15)?,
                root_meps_language_index: row.get(16)?,
                version_number: row.get(17)?,
                schema_version_number: row.get(18)?,
                hash: row.get(19)?,
                timestamp: row.get(20)?,
                jwpub: row.get(21)?,
                database_path: row.get(22)?,
                on_external_storage: row.get(23)?,
                undated_reference_title: row.get(24)?,
                expanded_size: row.get(25)?,
                min_platform_version: row.get(26)?,
                key_symbol: row.get(27)?,
                meps_build_number: row.get(28)?,
            })
        }

        debug!(target: TARGET, "Getting list for \"{}\"... (Length: {} items)", publication_type.blue(), pub_collection.len().to_string().yellow());

        Ok(pub_collection)
    }

    pub fn get_publication_collection_meta(
        &self,
        filename_symbol: &str,
    ) -> Result<Option<CollectionPublication>, Box<dyn std::error::Error>> {
        let mut stmt = self.catalog_db.prepare(
            "SELECT
                PublicationId,
                LanguageIndex,
                PublicationType,
                PublicationCategorySymbol,
                
                Title,
                ShortTitle,
                DisplayTitle,

                Symbol,
                UniqueEnglishSymbol,
                Year,
                VolumeNumber,
                IssueTagNumber,
                FirstDatedTextDateOffset,
                LastDatedTextDateOffset,
                
                RootSymbol,
                RootYear,
                RootMepsLanguageIndex,
                
                VersionNumber,
                SchemaVersionNumber,
                Hash,
                Timestamp,
                
                JwPub,
                DatabasePath,
                
                OnExternalStorage,
                UndatedReferenceTitle,
                
                ExpandedSize,
                MinPlatformVersion,
                KeySymbol,
                MepsBuildNumber
            FROM Publication WHERE JwPub=?1",
        )?;
        let mut rows = stmt.query([format!("{}.jwpub", filename_symbol)])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(CollectionPublication {
                id: row.get(0)?,
                language_index: row.get(1)?,
                publication_type: row.get(2)?,
                publication_category_symbol: row.get(3)?,
                title: row.get(4)?,
                short_title: row.get(5)?,
                display_title: row.get(6)?,
                symbol: row.get(7)?,
                unique_english_symbol: row.get(8)?,
                year: row.get(9)?,
                volume_number: row.get(10)?,
                issue_tag_number: row.get(11)?,
                first_dated_text_date_offset: row.get(12)?,
                last_dated_text_date_offset: row.get(13)?,
                root_symbol: row.get(14)?,
                root_year: row.get(15)?,
                root_meps_language_index: row.get(16)?,
                version_number: row.get(17)?,
                schema_version_number: row.get(18)?,
                hash: row.get(19)?,
                timestamp: row.get(20)?,
                jwpub: row.get(21)?,
                database_path: row.get(22)?,
                on_external_storage: row.get(23)?,
                undated_reference_title: row.get(24)?,
                expanded_size: row.get(25)?,
                min_platform_version: row.get(26)?,
                key_symbol: row.get(27)?,
                meps_build_number: row.get(28)?,
            }));
        }

        Ok(None)
    }

    pub fn get_current_publication(&mut self) -> Option<&mut Publication> {
        self.publication_cache.get_mut(&self.current_open)
    }

    pub fn open_publication_connection(
        &mut self,
        filename_symbol: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(_publication) = self.publication_cache.get(&filename_symbol) {
            debug!(target: TARGET, "Reopening connection with \"{}\"...", filename_symbol);
            self.current_open = filename_symbol.clone();
            return Ok(());
        }

        if let Some(publication_metadata) =
            self.get_publication_collection_meta(&filename_symbol)?
        {
            debug!(target: TARGET, "Opening connection with \"{}\"...", filename_symbol);
            let publication = Publication::from_database(
                PathBuf::from(publication_metadata.database_path),
                publication_metadata.id as i64,
            )?;
            debug!(target: TARGET, "Caching connection...");
            self.publication_cache.put(filename_symbol, publication);
            return Ok(());
        }

        error!(target: TARGET, "Publication requested doesn't exist.");
        Err("Publication not found in catalog".into())
    }
}

pub fn get_metadata_from_archive(
    pub_archive: &mut ZipArchive<io::BufReader<fs::File>>,
) -> Result<Manifest, serde_json::Error> {
    let mut manifest_file = pub_archive.by_name("manifest.json").unwrap();
    let mut manifest_data = String::new();
    manifest_file.read_to_string(&mut manifest_data).unwrap();
    serde_json::from_str(manifest_data.as_str())
}
