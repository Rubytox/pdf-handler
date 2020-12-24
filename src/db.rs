use rusqlite::{params, Connection, OpenFlags, Result, Row};

#[derive(Debug)]
pub struct PDF {
    id: i32,
    company: String,
    filename: String,
    producer: Option<String>,
    creator: Option<String>,
    author: Option<String>,
    creator_tool: Option<String>,
    pdf_version: String,
    title: Option<String>,
    xmp_toolkit: Option<String>,
    create_date: Option<String>,
    modify_date: Option<String>,
}

macro_rules! res_to_opt {
    ($r:expr, $i:expr) => {
        if let Ok(v) = $r.get($i) {
            Some(v)
        } else {
            None
        }
    };
}

impl PDF {
    fn make_from_row(row: &Row) -> Result<PDF> {
        Ok(PDF {
            id: row.get(0)?,
            company: row.get(1)?,
            filename: row.get(2)?,
            producer: res_to_opt![row, 3],
            creator: res_to_opt![row, 4],
            author: res_to_opt![row, 5],
            creator_tool: res_to_opt![row, 6],
            pdf_version: row.get(7)?,
            title: res_to_opt![row, 8],
            xmp_toolkit: res_to_opt![row, 9],
            create_date: res_to_opt![row, 10],
            modify_date: res_to_opt![row, 11],
        })
    }
}

fn create_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE pdf (
            id              INTEGER PRIMARY KEY,
            company         TEXT NOT NULL,
            filename        TEXT NOT NULL,
            producer        TEXT,
            creator         TEXT,
            author          TEXT,
            creator_tool    TEXT,
            pdf_version     TEXT NOT NULL,
            title           TEXT,
            xmp_toolkit     TEXT,
            create_date     TEXT,
            modify_date     TEXT
            )",
        params![],
    )?;

    Ok(())
}

pub fn init_db() -> Result<Connection> {
    match Connection::open_with_flags("pdfs.db", OpenFlags::SQLITE_OPEN_READ_WRITE) {
        Ok(conn) => Ok(conn),
        Err(_) => {
            let conn = Connection::open("pdfs.db")?;
            create_db(&conn)?;
            Ok(conn)
        }
    }
}

pub fn insert(conn: &Connection, pdf: &PDF) -> Result<usize> {
    conn.execute(
        "INSERT INTO pdf (company,
                          filename,
                          producer,
                          creator,
                          author,
                          creator_tool,
                          pdf_version,
                          title,
                          xmp_toolkit,
                          create_date,
                          modify_date)
                VALUES (?1,
                        ?2,
                        ?3,
                        ?4,
                        ?5,
                        ?6,
                        ?7,
                        ?8,
                        ?9,
                        ?10,
                        ?11)",
        params![
            pdf.company,
            pdf.filename,
            pdf.producer,
            pdf.creator,
            pdf.author,
            pdf.creator_tool,
            pdf.pdf_version,
            pdf.title,
            pdf.xmp_toolkit,
            pdf.create_date,
            pdf.modify_date
        ],
    )
}

pub fn get_by_id(conn: &Connection, id: i32) -> Result<PDF> {
    let mut stmt = conn.prepare(
        "SELECT * 
                                 FROM pdf
                                 WHERE id = :id",
    )?;
    let mut pdf_iter = stmt.query_map_named(&[(":id", &id)], PDF::make_from_row)?;

    // There should be only ONE PDF file with this id
    let t = pdf_iter.next().unwrap();
    Ok(t.unwrap())
}

pub fn get_all(conn: &Connection) -> Result<Vec<PDF>> {
    let mut stmt = conn.prepare("SELECT * FROM pdf")?;
    let pdf_iter = stmt.query_map(params![], PDF::make_from_row)?;

    let mut pdfs = Vec::new();
    for pdf in pdf_iter {
        pdfs.push(pdf?);
    }

    Ok(pdfs)
}

// fn main() -> Result<()> {
//     let conn = init_db()?;
//
//     // let pdf = PDF {
//     //     id: 0,
//     //     company: "LVMH".into(),
//     //     filename: "Lettre_actionnaires".into(),
//     //     producer: None,
//     //     creator: None,
//     //     author: None,
//     //     creator_tool: None,
//     //     pdf_version: "1.0".into(),
//     //     title: None,
//     //     xmp_toolkit: None,
//     //     create_date: None,
//     //     modify_date: None
//     // };
//
//     // insert(&conn, &pdf)?;
//
//     let all_pdfs = get_all(&conn)?;
//
//     all_pdfs.iter().for_each(|pdf| println!("PDF: {:?}", pdf));
//
//     Ok(())
// }
