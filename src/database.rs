use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::Queryable;
use diesel::Identifiable;
use diesel::Associations;
use diesel::Insertable;
use crate::schema::{manga, chapter};
use crate::ChapterItem;
use crate::LibraryItem;

// Function to establish a database connection
fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_library() -> Result<Vec<LibraryItem>, String> {
    let connection = &mut establish_connection();
    let all_manga = manga::table.load::<Manga>(connection).map_err(|err| err.to_string())?;
    let mut result = Vec::new();
    for manga in all_manga {
        result.push(LibraryItem {
            id: manga.id,
            title: manga.title,
            img: manga.img,
            extension: manga.extension,
            authors: manga.authors,
            artists: manga.artists,
            description: Some(manga.description),
            chapters: Vec::new()
        })
    }
    Ok(result)
}

pub fn create_manga(item: LibraryItem) -> Result<(), String> {
    let connection = &mut establish_connection();

    let desc = match item.description {
        Some(desc) => desc,
        None => "".to_string()
    };
    let new_manga = NewManga {
        id: &item.id,
        title: &item.title,
        img: &item.img,
        extension: &item.extension,
        authors: &item.authors,
        artists: &item.artists,
        description: &desc,
    };

    for chap in item.chapters {
        let new_chapter = NewChapter {
            id: &chap.id,
            manga_id: &item.id,
            number: chap.number,
            title: &chap.title,
            page: chap.page,
            completed: chap.completed,
        };

        diesel::insert_into(chapter::table)
            .values(&new_chapter)
            .execute(connection)
            .map_err(|err| err.to_string())?;
    }

    diesel::insert_into(manga::table)
        .values(&new_manga)
        .execute(connection)
        .map_err(|err| err.to_string())?;

    Ok(())
}

fn find_manga(id: String) -> Result<LibraryItem, String> {
    let connection = &mut establish_connection();

    // Find the manga by ID
    let manga_result: Manga = manga::table
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
#[derive(Queryable, Identifiable)]
#[table_name = "manga"]
pub struct Manga {
    pub id: String,
    pub title: String,
    pub img: String,
    pub extension: String,
    pub authors: String,
    pub artists: String,
    pub description: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Manga, foreign_key = "manga_id")]
#[table_name = "chapter"]
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
#[table_name = "manga"]
pub struct NewManga<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub img: &'a str,
    pub extension: &'a str,
    pub authors: &'a str,
    pub artists: &'a str,
    pub description: &'a str,
}

#[derive(Insertable)]
#[table_name = "chapter"]
pub struct NewChapter<'a> {
    pub id: &'a str,
    pub manga_id: &'a str,
    pub number: f32,
    pub title: &'a str,
    pub page: i32,
    pub completed: bool,
}
