use serde::{Serialize, Deserialize};

pub struct DatedText {
    // Primary key
    pub id: i32,

    // Foreign key to `Document` table
    pub document_id: i32,

    // MEPS helper
    pub link: String,

    pub first_date_offset: i32,
    pub last_date_offset: i32,

    // Foreign keys to `Footnote` table
    pub first_footnote_id: Option<i32>,
    pub last_footnote_id: Option<i32>,

    pub first_bible_citation_id: i32,
    pub last_bible_citation_id: i32,

    pub begin_paragraph_ordinal: i32,
    pub end_paragraph_ordinal: i32,

    pub caption: String,
    pub caption_rich: Option<String>,

    pub content: Vec<u8>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Document {
    // Primary key
    pub id: i32,

    // Foreign key to `PublicationMeta` table
    pub publication_id: i32,

    // MEPS helpers
    pub meps_document_id: i32,
    pub meps_language_id: i32,

    pub class: String,
    pub type_id: i32,
    pub section_number: i32,
    pub chapter_number: Option<i32>,

    // -------------------------------------------
    pub title: String,
    pub title_rich: Option<String>,

    pub toc_title: String,
    pub toc_title_rich: Option<String>,

    pub context_title: Option<String>,
    pub context_title_rich: Option<String>,

    pub feature_title: Option<String>,
    pub feature_title_rich: Option<String>,

    pub subtitle: Option<String>,
    pub subtitle_rich: Option<String>,

    pub feature_subtitle: Option<String>,
    pub feature_subtitle_rich: Option<String>,
    // -------------------------------------------
    // Content is a symmetric encrypted inflated file
    pub content: Vec<u8>,

    // -------------------------------------------
    pub first_footnote_id: Option<i32>,
    pub last_footnote_id: Option<i32>,
    pub first_bible_citation_id: Option<i32>,
    pub last_bible_citation_id: Option<i32>,
    // -------------------------------------------
    // Flags, can be used for conditional rendering.
    // For example, if `has_media_links` or `has_links`
    // are set to true, the document will be rendered
    // and all links will pass through `convertFileSrc()`
    // function and other hooks would be executed. If
    // false, it will have less overhead.
    pub paragraph_count: i32,

    pub has_media_links: bool,
    pub has_links: bool,

    pub first_page_number: Option<i32>,
    pub last_page_number: Option<i32>,

    pub content_length: i32,

    pub preferred_presentation: Option<String>, // TODO: Check other publications, probably refers to full page or html display
    pub content_reworked_date: Option<String>,

    pub has_pronunciation_guide: bool,
}

pub struct RefPublication {
    id: i32,

    version_number: i32,
    type_id: i32,

    title: String,
    title_rich: Option<String>,

    root_symbol: String,
    root_year: i32,
    root_meps_language_index: i32,

    short_title: String,
    short_title_rich: Option<String>,

    display_title: String,
    display_title_rich: Option<String>,

    reference_title: String,
    reference_title_rich: Option<String>,

    undated_reference_title: String,
    undated_reference_title_rich: Option<String>,

    symbol: String,
    undated_symbol: String,
    unique_symbol: String,
    english_symbol: String,
    unique_english_symbol: String,

    issue_tag_number: i32,
    issue_number: i32,

    variation: String,
    year: i32,

    volume_number: i32,

    meps_language_index: i32,

    publication_type: String,
    publication_category_symbol: String,

    bible_version_for_citations: String,

    has_publication_chapter_numbers: bool,
    has_publication_section_numbers: bool,

    first_dated_text_date_offset: Option<i32>,
    last_dated_text_date_offset: Option<i32>,

    meps_build_number: i32,
}

pub struct Extract {
    id: i32,

    link: String,
    caption: String,
    caption_rich: Option<String>,

    content: Vec<u8>,

    // Foreign key to `RefPublication` table
    ref_publication_id: i32,

    ref_meps_document_id: i32,
    ref_document_class: i32,
    ref_begin_paragraph_ordinal: Option<i32>,
    ref_end_paragraph_ordinal: Option<i32>,
}

pub struct Hyperlink {
    id: i32,

    link: String,
    major_type: Option<i32>,
    key_symbol: Option<String>,
    track: Option<i32>,

    meps_document_id: Option<i32>,
    meps_language_index: i32,
    issue_tag_number: i32,
}

pub struct DocumentExtract {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    // Foreign key to `Extract` table
    extract_id: i32,

    begin_paragraph_ordinal: i32,
    end_paragraph_ordinal: i32,

    sort_position: i32,

    // Foreign key to `Hyperlink` table
    hyperlink_id: i32,
}

pub struct DocumentHyperlink {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    // Foreign key to `Hyperlink` table
    hyperlink_id: i32,

    begin_paragraph_ordinal: i32,
    end_paragraph_ordinal: i32,

    sort_position: i32,
}

pub struct InternalLink {
    id: i32,

    link: String,
    caption: String,
    caption_rich: Option<String>,

    meps_document_id: i32,

    begin_paragraph_ordinal: Option<i32>,
    end_paragraph_ordinal: Option<i32>,
}

pub struct DocumentInternalLink {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    // Foreign key to `InternalLink` table
    internal_link_id: i32,

    begin_paragraph_ordinal: i32,
    end_paragraph_ordinal: i32,

    sort_position: i32,

    // Foreign key to `Hyperlink` table
    hyperlink_id: i32,
}

pub struct Multimedia {
    id: i32,

    link_multimedia_id: i32,
    data_type: i32,
    major_type: i32,
    minor_type: i32,
    width: i32,
    height: i32,

    mime_type: String,
    label: String,
    label_rich: Option<String>,

    caption: String,
    caption_rich: Option<String>,
    caption_content: Vec<u8>,

    credit_line: String,
    credit_line_rich: Option<String>,
    credit_line_content: Vec<u8>,

    category_type: i32,

    file_path: String,
    key_symbol: Option<String>,

    track: Option<i32>,
    meps_document_id: Option<i32>,
    meps_language_index: Option<i32>,
    issue_tag_number: i32,
    suppress_zoom: bool,
    size_constraint: Option<String>,
}

pub struct DocumentMultimedia {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    // Foreign key to `Multimedia` table
    multimedia_id: i32,

    begin_paragraph_ordinal: Option<i32>,
    end_paragraph_ordinal: Option<i32>,
}

pub struct DocumentParagraph {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    paragraph_index: i32,
    paragraph_number_label: Option<i32>,

    begin_position: Option<i32>,
    end_position: Option<i32>,
}

pub struct Endnote {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    text_id: i32,

    label: String,
    label_rich: Option<String>,

    content: Vec<u8>,
}

pub struct ExtractMultimedia {
    id: i32,

    // Foreign key to `Extract` table
    extract_id: i32,

    ref_meps_document_id: i32,
    ref_meps_document_class: i32,

    data_type: i32,

    major_type: i32,
    minor_type: i32,

    width: i32,
    height: i32,

    mime_type: String,
    label: String,
    label_rich: Option<String>,

    caption: String,
    caption_rich: Option<String>,
    caption_content: Vec<u8>,

    credit_line: String,
    credit_line_rich: Option<String>,
    credit_line_content: Vec<u8>,

    category_type: i32,

    file_path: String,
    key_symbol: Option<String>,

    track: Option<i32>,
    meps_document_id: Option<i32>,
    meps_language_index: Option<i32>,
    issue_tag_number: i32,
    suppress_zoom: bool,

    size_constraint: Option<String>,
}

pub struct ExtractVideoMarker {
    id: i32,

    // Foreign key to `Extract` table
    extract_id: i32,

    label: String,
    label_rich: Option<String>,

    caption: String,
    caption_rich: Option<String>,

    style: String,
    segment_format: i32,

    start_time_ticks: i32,
    duration_ticks: i32,

    start_frame: i32,
    frame_count: i32,

    begin_transition_duration_ticks: i32,
    end_transition_duration_ticks: i32,

    begin_transition_frame_count: i32,
    end_transition_frame_count: i32,
}

pub struct ExtractVideoMarkerRange {
    id: i32,

    // Foreign key to `Extract` table
    extract_id: i32,

    // Foreign key to `ExtractVideoMarker` table
    first_extract_video_marker_id: i32,

    // Foreign key to `ExtractVideoMarker` table
    last_extract_video_marker_id: i32,
}

pub struct BibleCitation {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    block_number: i32,
    element_number: i32,

    // Foreign key to `BibleVerse` table (Table doesn't exist)
    first_bible_verse_id: i32,

    // Foreign key to `BibleVerse` table (Table doesn't exist)
    last_bible_verse_id: i32,

    // Foreign key to `BibleVerse` table (Table doesn't exist)
    bible_verse_id: Option<i32>,

    paragraph_ordinal: i32,
    marginal_classification: i32,

    sort_position: i32,

    // Foreign key to `Hyperlink` table
    hyperlink_id: i32,
}

pub struct Footnote {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    footnote_index: i32,

    type_id: i32,

    content: Vec<u8>,

    // Foreign key to `BibleVerse` table (Table doesn't exist)
    bible_verse_id: Option<i32>,

    paragraph_ordinal: i32,
}

pub struct ParagraphCommentary {
    id: i32,

    commentary_type: i32,
    label: String,
    content: Vec<u8>,
}

pub struct ParagraphCommentaryMap {
    id: i32,

    meps_document_id: i32,

    begin_paragraph_ordinal: i32,
    end_paragraph_ordinal: i32,

    // Foreign key to `ParagraphCommentary` table
    paragraph_commentary_id: i32,
}

pub struct PublicationMeta {
    id: i32,

    version_number: i32,
    type_id: i32,

    title: String,
    title_rich: Option<String>,

    root_symbol: String,
    root_year: i32,
    root_meps_language_index: i32,

    short_title: String,
    short_title_rich: Option<String>,

    display_title: String,
    display_title_rich: Option<String>,

    reference_title: String,
    reference_title_rich: Option<String>,

    undated_reference_title: String,
    undated_reference_title_rich: Option<String>,

    symbol: String,
    undated_symbol: String,
    unique_symbol: String,
    english_symbol: String,
    unique_english_symbol: String,

    issue_tag_number: String,
}

pub struct PublicationAttribute {
    id: i32,

    // Foreign key to `PublicationMeta` table
    publication_id: i32,

    attribute: String,
}

pub struct PublicationCategory {
    id: i32,

    // Foreign key to `PublicationMeta` table
    publication_id: i32,

    category: String,
}

pub struct PublicationIssueAttribute {
    id: i32,

    // Foreign key to `PublicationIssue` table
    publication_issue_id: i32,

    attribute: String,
}

pub struct PublicationIssueProperty {
    id: i32,

    // Foreign key to `PublicationMeta` table
    publication_id: i32,

    title: String,
    title_rich: Option<String>,

    undated_title: String,
    undated_title_rich: Option<String>,

    cover_title: String,
    cover_title_rich: Option<String>,

    symbol: String,
    undated_symbol: String,
}

pub struct PublicationView {
    id: i32,
    name: String,
    symbol: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublicationViewItem {
    pub id: i32,

    // Foreign key to `PublicationView` table
    pub publication_view_id: i32,

    pub parent_publication_view_item_id: i32,

    pub title: String,
    pub title_rich: Option<String>,

    pub schema_type: i32,
    pub child_template_schema_type: Option<i32>,

    pub default_document_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PublicationViewItemDocument {
    pub id: i32,

    // Foreign key to `PublicationViewItem` table
    pub publication_view_item_id: i32,

    // Foreign key to `Document` table
    pub document_id: i32,
}

pub struct PublicationViewItemField {
    id: i32,

    // Foreign key to `PublicationViewItem` table
    publication_view_item_id: i32,

    value: String,
    value_rich: Option<String>,

    type_name: String,
}

pub struct PublicationViewSchema {
    id: i32,
    schema_type: i32,
    data_type: String,
}

pub struct PublicationYear {
    id: i32,

    // Foreign key to `PublicationMeta` table
    publication_id: i32,

    year: i32,
}

pub struct Question {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    question_index: i32,

    content: Vec<u8>,

    paragraph_ordinal: i32,
    target_paragraph_ordinal: i32,
    target_paragraph_number_label: i32,
}

pub struct RelatedDocument {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,
    meps_document_id: i32,

    relationship_type: i32,
}

pub struct SearchIndexBibleVerse {
    id: i32,

    // Foreign key to `Word` table
    word_id: i32,

    text_unit_count: i32,
    word_occurrence_count: i32,

    text_unit_indices: Vec<u8>,
    positional_list: Vec<u8>,
    positional_list_index: Vec<u8>,
}

pub struct SearchIndexDocument {
    id: i32,

    // Foreign key to `Word` table
    word_id: i32,

    text_unit_count: i32,
    word_occurrence_count: i32,

    text_unit_indices: Vec<u8>,
    positional_list: Vec<u8>,
    positional_list_index: Vec<u8>,
}

pub struct SearchTextRangeBibleVerse {
    id: i32,

    text_positions: Vec<u8>,
    text_lengths: Vec<u8>,
}

pub struct SearchTextRangeDocument {
    id: i32,

    text_positions: Vec<u8>,
    text_lengths: Vec<u8>,

    scope_paragraph_data: Vec<u8>,
}

pub struct TextUnit {
    id: i32,

    type_name: String,
    unit_id: i32,
}

pub struct Topic {
    id: i32,

    topic: String,
    display_topic: String,
    display_topic_rich: Option<String>,
}

pub struct TopicDocument {
    id: i32,

    // Foreign key to `Document` table
    document_id: i32,

    // Foreign key to `Topic` table
    topic_id: i32,
}

pub struct VerseCommentary {
    id: i32,

    commentary_type: i32,
    label: String,
    content: Vec<u8>,
}

pub struct VerseCommentaryMap {
    id: i32,

    // Foreign Key to `BibleVerse` table
    bible_verse_id: i32,

    // Foreign Key to `VerseCommentary` table
    verse_commentary_id: i32,
}

pub struct VerseMultimediaMap {
    id: i32,

    // Foreign Key to `BibleVerse` table
    bible_verse_id: i32,

    // Foreign Key to `Multimedia` table
    multimedia_id: i32,
}

pub struct VideoMarker {
    id: i32,

    // Foreign key to `Multimedia` table
    multimedia_id: i32,

    label: String,
    label_rich: Option<String>,

    caption: String,
    caption_rich: Option<String>,

    style: String,
    segment_format: i32,

    start_time_ticks: i32,
    duration_ticks: i32,

    start_frame: i32,
    frame_count: i32,

    begin_transition_duration_ticks: i32,
    end_transition_duration_ticks: i32,

    begin_transition_frame_count: i32,
    end_transition_frame_count: i32,
}

pub struct VideoMarkerBibleVerseLocation {
    id: i32,

    // Foreign Key to `BibleVerse` table
    bible_verse_id: i32,

    // Foreign Key to `VideoMarker` table
    video_marker_id: i32,
}

pub struct VideoMarkerParagraphLocation {
    id: i32,

    // Foreign Key to `DocumentParagraph` table
    document_paragraph_id: i32,

    // Foreign Key to `VideoMarker` table
    video_marker_id: i32,
}

pub struct Word {
    id: i32,

    word: String,
    reading: String,
}
