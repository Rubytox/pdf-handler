use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Version {
    primary: u64,
    secondary: u64,
}

impl Version {
    pub fn as_str(&self) -> String {
        format!("{}.{}", self.primary, self.secondary)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.primary, self.secondary)
    }
}

#[derive(Debug)]
pub struct Metadata {
    pub filename: String,
    pub producer: Option<String>,
	pub creator: Option<String>,
	pub author: Option<String>,
	pub creator_tool: Option<String>,
	pub pdf_version: Version,
	pub title: Option<String>,
	pub xmp_toolkit: Option<String>,
	pub create_date: Option<String>,
	pub modify_date: Option<String>,
}

pub fn get_os(meta: &Metadata) -> Option<String> {
    // First we check on producer
    if let Some(producer) = &meta.producer {
        let re = Regex::new(r".*\((.*)\).*").unwrap();
        for cap in re.captures_iter(producer) {
            println!("In-use OS: {}", &cap[1]);

            return Some(String::from(&cap[1]));
        }
    }
    if let Some(creator_tool) = &meta.creator_tool {
        let re = Regex::new(r"PScript.*\.dll").unwrap();
        if re.is_match(creator_tool) {
            return Some("Windows".to_string());
        }
    }
    None
}

fn update_metadata(meta: &mut Metadata, line: String) {
    let parts: Vec<&str> = line.split(" : ").collect();
    let id: String = parts[0].split_whitespace().collect();

    match &id[..] {
        "Producer" => meta.producer = Some(parts[1].into()),
        "Creator" => meta.creator = Some(parts[1].into()),
        "Author" => meta.author = Some(parts[1].into()),
        "CreatorTool" => meta.creator_tool = Some(parts[1].into()),
        "PDFVersion" => {
            let version = parts[1].parse::<f64>().unwrap();
            meta.pdf_version = Version {
                primary: version.trunc() as u64,
                secondary: (10.0 * version.fract()) as u64,
            };
        }
        "Title" => meta.title = Some(parts[1].into()),
        "XMPToolkit" => meta.xmp_toolkit = Some(parts[1].into()),
        "CreateDate" => meta.create_date = Some(parts[1].into()),
        "ModifyDate" => meta.modify_date = Some(parts[1].into()),
        _ => (),
    }
}

pub fn read_metadata(filename: &str, child: &mut Child) -> Option<Metadata> {
    if let Some(ref mut stdout) = child.stdout {
        let lines = BufReader::new(stdout).lines().enumerate();

        let mut meta = Metadata {
            filename: filename.into(),
            producer: None,
            creator: None,
            author: None,
            creator_tool: None,
            pdf_version: Version {
                primary: 1,
                secondary: 0,
            },
            title: None,
            xmp_toolkit: None,
            create_date: None,
            modify_date: None,
        };

        for (_, line) in lines {
            let content = line.unwrap();
            update_metadata(&mut meta, content);
        }

        Some(meta)
    } else {
        None
    }
}

fn main() {
    let mut metas: Vec<Metadata> = Vec::new();

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

        let meta = read_metadata(filename, &mut output.unwrap());
        match meta {
            Some(meta) => metas.push(meta),
            None => println!("Could not read metadata for {}", pathname.display()),
        }
    }

    let mut versions: HashMap<Version, u64> = HashMap::new();

    metas.iter().for_each(|m| {
        let count = versions.entry(m.pdf_version).or_insert(0);
        *count += 1;
    });

    metas.iter().for_each(|m| {
        match &m.create_date {
            Some(date) => println!("Creation date: {}", date),
            None => println!("No date"),
        };
    });

    metas.iter().for_each(|m| {
        get_os(m);
    });

    let mut all_meta: HashMap<&str, HashSet<String>> = HashMap::new();
    vec!["Producer", "Creator", "Author", "Creator Tool"]
        .iter()
        .for_each(|s| {
            all_meta.insert(s, HashSet::new());
        });

    metas.iter_mut().for_each(|m| {
        match &m.producer {
            Some(v) => update_hashset(&mut all_meta, "Producer", v),
            None => (),
        };
        match &m.creator {
            Some(v) => update_hashset(&mut all_meta, "Creator", v),
            None => (),
        };
        match &m.author {
            Some(v) => update_hashset(&mut all_meta, "Author", v),
            None => (),
        };
        match &m.creator_tool {
            Some(v) => update_hashset(&mut all_meta, "Creator Tool", v),
            None => (),
        };
    });

    all_meta.iter().for_each(|(id, set)| {
        println!("{}: {:?}", id, set);
    });

    // versions.iter().for_each(|(version, counter)| {
    //     println!("Version {} has been used {} times", version, counter);
    // });
}

fn update_hashset(h: &mut HashMap<&str, HashSet<String>>, k: &str, v: &String) {
    h.get_mut(k).unwrap().insert(v.clone());
}
