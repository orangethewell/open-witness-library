#![allow(dead_code)]

use hex::ToHex;
use inflate::inflate_bytes_zlib;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::{path::{PathBuf, Path}, fs, io::{self, Read}, collections::HashMap};
use fs_extra::dir::{move_dir, CopyOptions};
use super::extension::{Publication, Chapter}; 
use sqlite;

use openssl::sha::sha256;
use openssl::symm;

/// # Publication Manager
/// This struct is the main core of publication management. It holds a directory for
/// installation and a temp dir for publication extraction. It helps the application
/// to manage the installation of publications inside data directories and help getting
/// the publications already installed.

/// ## Example
/// 
/// Using [`tauri::State`], the application will hold a single state of `PubManager` inside
/// the app. This single state will be used through all the commands.
/// 
/// ```rust
/// // Simple example of publication extraction and installation:
/// [tauri::command]
/// fn load_publication<'r>(manager: State<'r, jwpub::PubManager>, publication_path: String){
///     let extract_info = manager.extract_publication(publication_path);
///     manager.install(extract_info);
/// }
/// ```
/// 
/// With this singleton technique, publications can be easily accessed in the configurated
/// directory, with addition, deletion and reading functions in the frontend application.
pub struct PubCatalog {
    local: PathBuf,

    pub media_location: PathBuf,

    // Cache the last key for future decoding
    cached_key: HashMap<String, Vec<u8>>,
}

#[derive(Debug)]
pub enum CatalogResponse {
    CatOk,
    CatError(String),
}

impl PubCatalog {
    pub fn new<T: Into<PathBuf>>(local: T) -> Self{
        let local = local.into();
        if !local.exists() {
            fs_extra::dir::create_all(&local, false);
        }
        Self { local, media_location: PathBuf::new(), cached_key: HashMap::new() }
    }

    /// Will try to install a publication on local directory. It will send a `CatalogResponse` to
    /// Frontend for parsing.
    pub fn install_publication<T: Into<PathBuf>>(&self, pub_path: T) -> CatalogResponse {
        let pub_path: PathBuf = pub_path.into();
        let pub_archive = fs::File::open(&pub_path).unwrap();
        let reader = io::BufReader::new(pub_archive);
        let mut pub_archive = zip::ZipArchive::new(reader).unwrap();

        let mut publication_dir = PathBuf::new();
        let manifest: Value;
        // Manifest Initialization:
        {
            let mut manifest_file = pub_archive.by_name("manifest.json").unwrap();
            let mut manifest_data = String::new();
            manifest_file.read_to_string(&mut manifest_data).unwrap();
            manifest = serde_json::from_str(manifest_data.as_str()).unwrap();
        }

        let filename = pub_path.file_name().unwrap().to_str().to_owned().unwrap();
        let lang: String = filename.split("_").collect::<Vec<_>>()[1].to_owned();
        let category: String = {
            manifest["publication"]["categories"][0].as_str().unwrap().to_string()
        };

        if !self.local.is_dir(){
            fs::create_dir(&self.local).unwrap();
        } 

        // Let check the respective directory map for /lang/category/pub: 

        if !Path::new(&self.local.join(format!("publications/{lang}/{category}/"))).exists() {
            fs::create_dir_all(&self.local.join(format!("publications/{lang}/{category}/"))).expect("Failed to create directory")
        }

        publication_dir.push(self.local.join(format!("publications/{lang}/{category}/{}", filename.replace(".jwpub", ""))));
        if publication_dir.exists() {
            println!("This publication is already installed.");
        } else {
            fs::create_dir(&publication_dir).unwrap();
        }

        // Move stuff to where it should be 
        crate::jwpub::unpack_zip(pub_archive, &publication_dir);
        let content_dir = publication_dir.join("content");
        fs::create_dir(&content_dir).unwrap_or(());
    
        let content_archive = fs::File::open(&publication_dir.join("contents")).unwrap();
        let reader = io::BufReader::new(content_archive);
        let content_archive = zip::ZipArchive::new(reader).unwrap();
    
        crate::jwpub::unpack_zip(content_archive, &content_dir);
    
        fs::remove_file(&publication_dir.join("contents")).unwrap();
        CatalogResponse::CatOk
    }

    /// Return a List of publications of certain category. (Max of 25)
    pub fn get_list_from_category(&self, lang: String, category: String, start_idx: Option<usize>, limit: Option<usize>) -> Vec<Publication> {
        let pub_list_target = self.local.join(format!("publications/{lang}/{category}"));
        // TODO: Make a parser for when category be "*".
        let mut list: Vec<Publication> = vec![];
        println!("Catalog Manager >> Category/directory: {}", pub_list_target.display());
        // BUG: Check if dir exists
        for entry in fs::read_dir(&pub_list_target).unwrap().skip(start_idx.or(Some(0)).unwrap()).take(std::cmp::min(limit.or(Some(25)).unwrap(), 25)) {
            let manifest: Value; 
            let entry = entry.unwrap();
            {
                let manifest_path = entry.path().join("manifest.json");
                let mut manifest_file = fs::File::open(manifest_path).unwrap();
                    let mut manifest_data = String::new();
                    manifest_file.read_to_string(&mut manifest_data).unwrap();
                    manifest = serde_json::from_str(manifest_data.as_str()).unwrap();
            } 

            println!("Publication detected: {}", manifest["publication"]["title"].as_str().unwrap().to_string());
            
            list.push(Publication {
                category: category.clone(),
                language: lang.clone(), 
                symbol: manifest["publication"]["undatedSymbol"].as_str().unwrap().to_string(),
                title: manifest["publication"]["title"].as_str().unwrap().to_string(),
                display_title: manifest["publication"]["shortTitle"].as_str().unwrap().to_string(), 
                cover_icon_path: PathBuf::from(
                        entry.path().join(format!("content/{}", manifest["publication"]["images"][1]["fileName"].as_str().unwrap().to_string()))
                    ),
                year: manifest["publication"]["year"].as_i64().unwrap(),
            })

        }

        list
    }

    // --------------------------------------------------------------------------------------------------------------------------------------------------------
    // URI JWPUB:///lang/category/pub scheme impl:
    // These functions will access Publication Metadata stored in their `.db` files.
    
    /// Get summary from certain publication when a request is made to
    /// `jwpub:///lang/category/pub`. 
    pub fn get_summary_from(&self, lang: String, category: String, pub_symbol: String) -> Result<Vec<Chapter>, ()> {
        let pub_directory = self.normalize_request_directory(&lang, &category, &pub_symbol); 
        println!("{:#?}", pub_directory);
        let manifest = self.populate_manifest(&pub_directory).unwrap();
        
        if let Ok(connection) = sqlite::open(&pub_directory.join(format!("content/{}", manifest["publication"]["fileName"].as_str().unwrap().to_owned()))){
           let mut cursor = connection.prepare("SELECT documentId, Class, sectionNumber, title, contextTitle, chapterNumber FROM document").unwrap().into_cursor();
           let mut summary: Vec<Chapter> = vec![];

           while let Some(Ok(row)) = cursor.next() {
               let mut new_chapter = Chapter {
                   id: row.get::<i64, _>(0),
                   class: 0,
                   section: row.get::<i64, _>(2),
                   title: row.get::<String, _>(3),
                   context_title: row.get::<Option<String>, _>(4).unwrap_or("".to_string()),
                   number: row.get::<Option<i64>, _>(5).unwrap_or(-1),
               };
               summary.push(new_chapter);
           }

            return Ok(summary)
        }

        Err(())
        
    }

    /// Get the content of a publication chapter. Request is made as query:
    /// `jwpub:///lang/category/pub?contentId={value}`
    /// Ex: jwpub://T/bk/lff_T?contentId=4
    pub fn get_chapter_content(&mut self, lang: String, category: String, pub_symbol: String, chapter_id: i64) -> String {
        let pub_directory = self.normalize_request_directory(&lang, &category, &pub_symbol);
        println!("{:#?}", pub_directory);
        let manifest = self.populate_manifest(&pub_directory).unwrap();

        if let Ok(connection) = sqlite::open(&pub_directory.join(format!("content/{}", manifest["publication"]["fileName"].as_str().unwrap().to_owned()))) {
            let mut cursor = connection.prepare("SELECT content FROM document WHERE documentId=?").unwrap()
            .into_cursor()
            .bind(&[sqlite::Value::Integer(chapter_id)])
            .unwrap();

            while let Some(Ok(row)) = cursor.next() {
                let encrypted_content = row.get::<Vec<u8>, _>(0);
                let master_key: Vec<u8>;
                if self.cached_key.contains_key(&pub_symbol) {
                    master_key = self.cached_key.get(&pub_symbol).unwrap().to_owned();
                } else {
                    master_key = self.forge_master_key(&pub_directory);
                    if self.cached_key.len() >= 12 {
                        self.cached_key.drain();
                    }
                    self.cached_key.insert(pub_symbol.clone(), master_key.clone());
                }
                let (key, iv) = master_key.split_at(master_key.len() / 2);
                let decrypted_content = inflate_bytes_zlib(&symm::decrypt(
                    symm::Cipher::aes_128_cbc(),
                    key,
                    Some(iv),
                    &encrypted_content
                ).unwrap()).unwrap();

                return std::str::from_utf8(&decrypted_content).expect("Decode Error").to_string();
            } 
        }

        String::from("")
    }

    /// Get the publication icon/cover. Request is made as query:
    /// `jwpub://localhost/lang/category/pub/cover`
    /// ### Ex:
    /// `jwpub://localhost/T/bk/lff_T/cover`
    pub fn get_cover_icon(&mut self, lang: String, category: String, pub_symbol: String) -> Vec<u8>  {
        let pub_directory = self.normalize_request_directory(&lang, &category, &pub_symbol);
        let manifest = self.populate_manifest(&pub_directory).unwrap();

        if let Ok(data) = fs::read(&pub_directory.join(format!("content/{}", manifest["publication"]["images"][0]["fileName"].as_str().unwrap().to_string()))) {
            return data;
        } else {
            return vec![];
        }
    }

    // Media Location: functions for `jwpub-media` URI.
    pub fn set_media_location(&mut self, lang: &str, category: &str, pub_symbol: &str) {
        self.media_location = self.normalize_request_directory(lang, category, pub_symbol).join("content");
    }
    

    fn normalize_request_directory(&self, lang: &str, category: &str, pub_symbol: &str) -> PathBuf {
        self.local.join(format!("publications/{lang}/{category}/{pub_symbol}"))
    } 

    fn populate_manifest(&self, pub_directory: &PathBuf) -> Result<Value, ()> {
        let mut manifest_path = pub_directory.join("manifest.json");
        if let Ok(mut manifest_file) = fs::File::open(manifest_path) {
            let mut manifest_data = String::new();
            manifest_file.read_to_string(&mut manifest_data).unwrap();
            Ok(serde_json::from_str(manifest_data.as_str()).unwrap())
        }  else {
            Err(())
        }
    }

    fn forge_master_key(&self, pub_directory: &PathBuf) -> Vec<u8> {
        let connection = sqlite::open(
            pub_directory.join(format!(
                "content/{}", 
                self.populate_manifest(pub_directory).unwrap()["publication"]["fileName"]
                .as_str()
                .unwrap()
                .to_owned()
            ))).unwrap();
            
            let mut cursor = connection.prepare("SELECT MepsLanguageIndex, Symbol, Year, IssueTagNumber FROM Publication").unwrap().into_cursor(); 
            let (mut meps_language_index, mut year): (i64, i64) = (0, 0);
            let (mut symbol, mut issue_tag_number): (String, String) = (String::new(), String::new());
            while let Some(Ok(row)) = cursor.next() {
                meps_language_index = row.get::<i64, _>(0);
                symbol = row.get::<String, _>(1);
                year = row.get::<i64, _>(2);
                issue_tag_number = row.get::<String, _>(3);
            }
            
            let key_string = 
                if issue_tag_number == "0" {
                    String::from(format!("{}_{}_{}", meps_language_index, symbol, year))
                } else {
                    String::from(format!("{}_{}_{}_{}", meps_language_index, symbol, year, issue_tag_number))
                };

            let key_part1 = sha256(key_string.as_bytes());
            let key_part2 = hex::decode("11cbb5587e32846d4c26790c633da289f66fe5842a3a585ce1bc3a294af5ada7").unwrap();

            let master_key: Vec<u8> = key_part1.iter().zip(key_part2).map(|(x, y)| x ^ y).collect();
            println!("{:#?}", master_key.len());
            master_key
    }

}
