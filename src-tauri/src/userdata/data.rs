use std::path::PathBuf;
use colored::Colorize;
use rusqlite::{params, Connection};

use super::tables::Location;

const TARGET: &'static str = "userdata";

pub struct UserData {
    data_path: PathBuf,
    userdata_db: Connection,
}

impl UserData {
    pub fn init<T: Into<PathBuf>>(location: T) -> Result<Self, Box<dyn std::error::Error>> {
        debug!(target: TARGET, "Initializing user data...");
        let location: PathBuf = location.into();
        if !location.exists() {
            fs_extra::dir::create_all(&location, false)?;
            info!(target: TARGET, "Creating user data location...");
        }

        let db = Connection::open(location.join("userdata.db"))?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "Location".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS Location (
                LocationId INTEGER PRIMARY KEY AUTOINCREMENT,
                
                BookNumber INTEGER,
                ChapterNumber INTEGER,
                DocumentId INTEGER,
                
                Track INTEGER,
                IssueTagNumber INTEGER NOT NULL,
                KeySymbol TEXT NOT NULL,
                MepsLanguage INTEGER,
                Type INTEGER NOT NULL,
                Title TEXT NOT NULL
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "UserMark".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS UserMark (
                UserMarkId INTEGER PRIMARY KEY AUTOINCREMENT,
                
                ColorIndex INTEGER NOT NULL,
                LocationId INTEGER NOT NULL,
                StyleIndex INTEGER NOT NULL,
                
                UserMarkGuid TEXT NOT NULL,
                Version INTEGER NOT NULL,
                
                FOREIGN KEY (LocationId) REFERENCES Location(LocationId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "BlockRange".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS BlockRange (
                BlockRangeId INTEGER PRIMARY KEY AUTOINCREMENT,
                
                BlockType INTEGER NOT NULL,
                Identifier INTEGER NOT NULL,
                StartToken INTEGER NOT NULL,
                
                EndToken INTEGER NOT NULL,
                UserMarkId TEXT NOT NULL,
                DisplayTitle TEXT NOT NULL,
                
                FOREIGN KEY (UserMarkId) REFERENCES UserMark(UserMarkId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "Bookmark".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS Bookmark (
                BookmarkId INTEGER PRIMARY KEY AUTOINCREMENT,
                
                LocationId INTEGER NOT NULL,
                PublicationLocationId INTEGER NOT NULL,
                Slot INTEGER NOT NULL,
                
                Title TEXT NOT NULL,
                Snippet TEXT NOT NULL,
                BlockType INTEGER NOT NULL,
                BlockIdentifier INTEGER NOT NULL,

                FOREIGN KEY (LocationId) REFERENCES Location(LocationId),
                FOREIGN KEY (PublicationLocationId) REFERENCES Location(LocationId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "IndependentMedia".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS IndependentMedia (
                IndependentMediaId INTEGER PRIMARY KEY AUTOINCREMENT,
                
                OriginalFilename TEXT NOT NULL,
                FilePath TEXT NOT NULL,
                MimeType TEXT NOT NULL,
                Hash TEXT NOT NULL,
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "InputField".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS InputField (
                LocationId INTEGER PRIMARY KEY,
                TextTag TEXT NOT NULL PRIMARY KEY,
                Value TEXT NOT NULL,

                FOREIGN KEY (LocationId) REFERENCES Location(LocationId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "LastModified".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS LastModified (
                LastModified TEXT NOT NULL
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "Note".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS Note (
                NoteId INTEGER PRIMARY KEY AUTOINCREMENT,
                Guid TEXT NOT NULL,

                UserMarkId INTEGER,
                LocationId INTEGER,
                Title TEXT NOT NULL,
                Content TEXT NOT NULL,
                LastModified TEXT NOT NULL,
                Created TEXT NOT NULL,

                BlockType INTEGER NOT NULL,
                BlockIdentifier INTEGER NOT NULL,

                FOREIGN KEY (UserMarkId) REFERENCES UserMark(UserMarkId),
                FOREIGN KEY (LocationId) REFERENCES Location(LocationId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PlaylistItemAccuracy".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PlaylistItemAccuracy (
                PlaylistItemAccuracyId INTEGER PRIMARY KEY AUTOINCREMENT,
                Description TEXT NOT NULL
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PlaylistItem".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PlaylistItem (
                PlaylistItemId INTEGER PRIMARY KEY AUTOINCREMENT,
                Label TEXT NOT NULL,

                StartTrimOffsetTicks INTEGER,
                StartTrimOffsetTicks INTEGER,
                Accuracy INTEGER NOT NULL,
                EndAction INTEGER NOT NULL,
                ThumbnailFilePath TEXT NOT NULL,
                Created TEXT NOT NULL,

                BlockType INTEGER NOT NULL,
                BlockIdentifier INTEGER NOT NULL,

                FOREIGN KEY (Accuracy) REFERENCES PlaylistItemAccuracy(PlaylistItemAccuracyId),
                FOREIGN KEY (ThumbnailFilePath) REFERENCES IndependentMedia(FilePath)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PlaylistItemIndependentMediaMap".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PlaylistItemIndependentMediaMap (
                PlaylistItemId INTEGER PRIMARY KEY,
                IndependentMediaId INTEGER PRIMARY KEY,

                DurationTicks INTEGER NOT NULL,

                FOREIGN KEY (PlaylistItemId) REFERENCES PlaylistItem(PlaylistItemId),
                FOREIGN KEY (IndependentMediaId) REFERENCES IndependentMedia(IndependentMediaId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PlaylistItemLocationMap".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PlaylistItemLocationMap (
                PlaylistItemId INTEGER PRIMARY KEY,
                LocationId INTEGER PRIMARY KEY,

                MajorMultimediaType INTEGER NOT NULL,
                BaseDurationTicks INTEGER NOT NULL,

                FOREIGN KEY (PlaylistItemId) REFERENCES PlaylistItem(PlaylistItemId),
                FOREIGN KEY (LocationId) REFERENCES Location(LocationId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PlaylistItemMarker".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PlaylistItemLocationMap (
                PlaylistItemMarkerId INTEGER PRIMARY KEY AUTOINCREMENT,
                PlaylistItemId INTEGER NOT NULL,

                Label TEXT NOT NULL,
                StartTimeTicks INTEGER NOT NULL,
                DurationTicks INTEGER NOT NULL,
                EndTransitionDurationTicks INTEGER NOT NULL,

                FOREIGN KEY (PlaylistItemId) REFERENCES PlaylistItem(PlaylistItemId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PlaylistItemMarkerBibleVerseMap".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PlaylistItemMarkerBibleVerseMap (
                PlaylistItemMarkerId INTEGER PRIMARY KEY,
                VerseId INTEGER PRIMARY KEY,

                FOREIGN KEY (PlaylistItemMarkerId) REFERENCES PlaylistItemMarker(PlaylistItemMarkerId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "PlaylistItemMarkerParagraphMap".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS PlaylistItemMarkerParagraphMap (
                PlaylistItemMarkerId INTEGER PRIMARY KEY,
                MepsDocumentId INTEGER PRIMARY KEY,
                ParagraphIndex INTEGER PRIMARY KEY,
                MarkerIndexWithinParagraph INTEGER PRIMARY KEY,

                FOREIGN KEY (PlaylistItemMarkerId) REFERENCES PlaylistItemMarker(PlaylistItemMarkerId)
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "Tag".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS Tag (
                TagId INTEGER PRIMARY KEY AUTOINCREMENT,
                Type INTEGER NOT NULL,
                Name TEXT NOT NULL,
            )",
            (),
        )?;

        debug!(target: TARGET,  "initializing \"{}\" table...", "TagMap".magenta());
        db.execute(
            "CREATE TABLE IF NOT EXISTS Tag (
                TagMapId INTEGER PRIMARY KEY AUTOINCREMENT,
                PlaylistItemId INTEGER,
                LocationId INTEGER,
                NoteId INTEGER,
                TagId INTEGER,
                Position INTEGER,

                FOREIGN KEY (PlaylistItemId) REFERENCES PlaylistItem(PlaylistItemId),
                FOREIGN KEY (LocationId) REFERENCES Location(LocationId),
                FOREIGN KEY (NoteId) REFERENCES Note(NoteId),
                FOREIGN KEY (TagId) REFERENCES Tag(TagId)
            )",
            (),
        )?;

        Ok(Self {
            data_path: location,
            userdata_db: db
        })
    }

    pub fn get_locations(
        &self,
    ) -> Result<Vec<Location>, Box<dyn std::error::Error>> {
        let mut stmt = self.userdata_db.prepare(
            "SELECT 
                LocationId,
                BookNumber,
                ChapterNumber,
                DocumentId,
                Track,
                IssueTagNumber,
                KeySymbol,
                MepsLanguage,
                Type,
                Title
        FROM PublicationViewItemDocument",
        )?;
        let mut rows = stmt.query([])?;

        let mut locations = vec![];

        while let Some(row) = rows.next()? {
            let location = Location {
                id: row.get(0)?,
                book_number: row.get(1)?,
                chapter_number: row.get(2)?,
                document_id: row.get(3)?,
                track: row.get(4)?,
                issue_tag_number: row.get(5)?,
                key_symbol: row.get(6)?,
                meps_language: row.get(7)?,
                type_id: row.get(8)?,
                title: row.get(9)?,
            };

            locations.push(location);
        }

        Ok(locations)
    }

    pub fn get_location_by_document_id(
        &self,
        document_id: i32,
    ) -> Result<Option<Location>, Box<dyn std::error::Error>> {
        let mut stmt = self.userdata_db.prepare(
            "SELECT 
                LocationId,
                BookNumber,
                ChapterNumber,
                DocumentId,
                Track,
                IssueTagNumber,
                KeySymbol,
                MepsLanguage,
                Type,
                Title
        FROM Location WHERE DocumentId=?1",
        )?;
        let mut rows = stmt.query([document_id])?;

        if let Some(row) = rows.next()? {
            let location = Location {
                id: row.get(0)?,
                book_number: row.get(1)?,
                chapter_number: row.get(2)?,
                document_id: row.get(3)?,
                track: row.get(4)?,
                issue_tag_number: row.get(5)?,
                key_symbol: row.get(6)?,
                meps_language: row.get(7)?,
                type_id: row.get(8)?,
                title: row.get(9)?,
            };

            return Ok(Some(location))
        }

        Ok(None)
    }

    pub fn insert_location(
        &self,
        book_number: Option<i32>,
        chapter_number: Option<i32>,
        document_id: i32,
        track: Option<i32>,
        issue_tag_number: i32,
        key_symbol: String,
        meps_language: i32,
        type_id: i32,
        title: String,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        self.userdata_db.execute(
            "INSERT INTO Location (
                BookNumber,
                ChapterNumber,
                DocumentId,
                Track,
                IssueTagNumber,
                KeySymbol,
                MepsLanguage,
                Type,
                Title
        ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
         params![
            book_number,
            chapter_number,
            document_id,
            track,
            issue_tag_number,
            key_symbol,
            meps_language,
            type_id,
            title,
         ]
        )?;

        let location_id = self.userdata_db.last_insert_rowid();

        debug!(target: TARGET, "Location inserted to userdata database for ID {}!", location_id);

        Ok(location_id)
    }
}