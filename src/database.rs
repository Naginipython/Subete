use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::Queryable;
use diesel::Identifiable;
use diesel::Associations;
use diesel::Insertable;
use crate::schema::{library, chapters};
use crate::ChapterItem;
use crate::LibraryItem;

// Function to establish a database connection
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_library() -> Result<Vec<LibraryItem>, String> {
    let connection = &mut establish_connection();
    let all_manga = library::table.load::<Manga>(connection).map_err(|err| err.to_string())?;
    let all_chapters = Chapter::belonging_to(&all_manga)
        .load::<Chapter>(connection)
        .map_err(|err| err.to_string())?
        .grouped_by(&all_manga);

    let library_items: Vec<LibraryItem> = all_manga
        .into_iter()
        .zip(all_chapters)
        .map(|(manga, chapters)| LibraryItem {
            id: manga.id,
            title: manga.title,
            img: manga.img,
            extension: manga.extension,
            authors: manga.authors,
            artists: manga.artists,
            description: Some(manga.description),
            chapters: chapters
                .into_iter()
                .map(|chapter| ChapterItem {
                    id: chapter.id,
                    number: chapter.number,
                    title: chapter.title,
                    page: chapter.page,
                    completed: chapter.completed,
                })
                .collect(),
        })
        .collect();

    Ok(library_items)
}

pub fn create_manga(item: LibraryItem) -> Result<(), String> {
    let connection = &mut establish_connection();

    let (new_manga, chapters) = item.to_new_manga();

    for chap in chapters {
        let new_chapter = NewChapter {
            id: &chap.id,
            manga_id: &item.id,
            number: chap.number,
            title: &chap.title,
            page: chap.page,
            completed: chap.completed,
        };

        diesel::insert_into(chapters::table)
            .values(&new_chapter)
            .execute(connection)
            .map_err(|err| err.to_string())?;
    }

    diesel::insert_into(library::table)
        .values(&new_manga)
        .execute(connection)
        .map_err(|err| err.to_string())?;

    Ok(())
}

pub fn update_manga(item: LibraryItem) -> Result<(), String> {
    let connection = &mut establish_connection();

    let (new_manga, new_chapters) = item.to_manga();

    diesel::update(library::table.filter(library::id.eq(&new_manga.id)))
        .set(&new_manga)
        .execute(connection)
        .map_err(|err| err.to_string())?;

    // Delete existing chapters for the manga
    diesel::delete(chapters::table.filter(chapters::manga_id.eq(&new_manga.id)))
        .execute(connection)
        .map_err(|err| err.to_string())?;

    let insert_chap: Vec<NewChapter> = new_chapters.iter()
        .map(|chapter| chapter.to_new_chapter())
        .collect();

    for c in insert_chap {
        // Insert new chapters
        diesel::update(chapters::table.filter(chapters::manga_id.eq(&new_manga.id)))
            .set(&c)
            .execute(connection)
            .map_err(|err| err.to_string())?;
    }
    Ok(())
}

pub fn find_manga(id: String) -> Result<LibraryItem, String> {
    let connection = &mut establish_connection();

    // Find the manga by ID
    let manga_result: Manga = library::table
        .find(id.clone())
        .first::<Manga>(connection)
        .map_err(|err| err.to_string())?;
    
    // Find the chapters associated with the manga
    let chapters_result: Vec<Chapter> = Chapter::belonging_to(&manga_result)
        .load::<Chapter>(connection)
        .map_err(|err| err.to_string())?;

    let mut chp = Vec::new();
    for c in chapters_result {
        chp.push(ChapterItem {
            id: c.id,
            number: c.number,
            title: c.title,
            page: c.page,
            completed: c.completed
        })
    }

    Ok(LibraryItem {
        id: manga_result.id,
        title: manga_result.title,
        img: manga_result.img,
        extension: manga_result.extension,
        authors: manga_result.authors,
        artists: manga_result.artists,
        description: Some(manga_result.description),
        chapters: chp
    })
}

// ----- DATA MODELS -----
#[derive(Queryable, Identifiable, AsChangeset)]
#[table_name = "library"]
pub struct Manga {
    pub id: String,
    pub title: String,
    pub img: String,
    pub extension: String,
    pub authors: String,
    pub artists: String,
    pub description: String,
}

#[derive(Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(Manga, foreign_key = "manga_id")]
#[table_name = "chapters"]
pub struct Chapter {
    pub id: String,
    pub manga_id: String,
    pub number: f32,
    pub title: String,
    pub page: i32,
    pub completed: bool,
}

// ----- INSERT MODELS -----
#[derive(Insertable)]
#[table_name = "library"]
pub struct NewManga<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub img: &'a str,
    pub extension: &'a str,
    pub authors: &'a str,
    pub artists: &'a str,
    pub description: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "chapters"]
pub struct NewChapter<'a> {
    pub id: &'a str,
    pub manga_id: &'a str,
    pub number: f32,
    pub title: &'a str,
    pub page: i32,
    pub completed: bool,
}
impl Chapter {
    fn to_new_chapter(&self) -> NewChapter<'_> {
        NewChapter {
            id: &self.id,
            manga_id: &self.manga_id,
            number: self.number,
            title: &self.title,
            page: self.page,
            completed: self.completed,
        }
    }
}