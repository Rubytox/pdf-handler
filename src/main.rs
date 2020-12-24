use rusqlite::Result;

mod db;
mod exif;

fn main() -> Result<()> {
    let conn = db::init_db()?;

    // let pdf = PDF {
    //     id: 0,
    //     company: "LVMH".into(),
    //     filename: "Lettre_actionnaires".into(),
    //     producer: None,
    //     creator: None,
    //     author: None,
    //     creator_tool: None,
    //     pdf_version: "1.0".into(),
    //     title: None,
    //     xmp_toolkit: None,
    //     create_date: None,
    //     modify_date: None
    // };

    // insert(&conn, &pdf)?;

    let all_pdfs = db::get_all(&conn)?;

    all_pdfs.iter().for_each(|pdf| println!("PDF: {:?}", pdf));

    Ok(())
}
