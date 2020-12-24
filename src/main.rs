use std::fs;
use std::process::Command;
use std::process::Stdio;
use rusqlite::{Connection, Result};

mod db;
mod exif;

fn insert_db(conn: &Connection, meta: exif::Metadata, company: &str) -> Result<usize> {
    let pdf = db::PDF {
        id: -1,  // It is assigned by the database
        company: company.into(),
        filename: meta.filename,
        producer: meta.producer,
        creator: meta.creator,
        author: meta.author,
        creator_tool: meta.creator_tool,
        pdf_version: meta.pdf_version.as_str(),
        title: meta.title,
        xmp_toolkit: meta.xmp_toolkit,
        create_date: meta.create_date,
        modify_date: meta.modify_date,
    };

    db::insert(&conn, &pdf)
}

fn update_exif_data(conn: &Connection) -> Result<()> {
    let path = "pdf/lvmh/";
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let pathname = path.unwrap().path();
        let string_path = pathname.clone().into_os_string().into_string().unwrap();
        let filename = string_path.split('/').next_back().unwrap();

        if !filename.ends_with(".pdf") {
            continue;
        }

        let output = Command::new("exiftool")
            .arg(&pathname)
            .stdout(Stdio::piped())
            .spawn();

        let meta = exif::read_metadata(filename, &mut output.unwrap());
        match meta {
            Some(meta) => { 
                insert_db(&conn, meta, "LVMH")?;
            },
            None => println!("Could not read metadata for {}", pathname.display()),
        }
    }


    Ok(())
}

fn main() -> Result<()> {
    let conn = db::init_db()?;

    update_exif_data(&conn)?;

    let all_pdfs = db::get_all(&conn)?;

    all_pdfs.iter().for_each(|pdf| println!("PDF: {:?}", pdf));

    Ok(())
}
