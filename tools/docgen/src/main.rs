#[macro_use]
extern crate clap;

use clap::{Arg, App};

use std::cmp;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::slice::Iter;

// TODO: header support

#[derive(Clone,PartialEq)]
enum DocMode {
    Cli,
    Gui,
}

struct Data {
    workdir: String,
    mode: DocMode,
    img_before_path: String,
    img_after_path: String,
}

fn main() {
    let m = App::new("docgen")
        .version("0.1.0")
        .arg(Arg::with_name("docdir")
            .long("docdir")
            .help("Sets path to documentation directory")
            .value_name("DIR")
            .required(true))
        .arg(Arg::with_name("for-gui")
            .long("for-gui")
            .help("Generate documentation for GUI")
            .requires_all(&["outdir"]))
        .arg(Arg::with_name("outdir")
            .long("outdir")
            .help("Sets path to working directory")
            .value_name("DIR"))
        .get_matches();

    let workdir = m.value_of("docdir").unwrap();
    let srcdir = Path::new(workdir).join("src").to_str().unwrap().to_owned();

    let mode;
    if m.is_present("for-gui") {
        mode = DocMode::Gui;
    } else {
        mode = DocMode::Cli;
    }

    let outdir;
    if m.is_present("outdir") {
        outdir = m.value_of("outdir").unwrap().to_owned();
    } else {
        outdir = String::new();
    }

    // init folders
    // doc/images/before
    // doc/images/after
    let imgs_path = Path::new(workdir).join("images");
    let before_dir = Path::new(&imgs_path).join("before");
    let after_dir = Path::new(&imgs_path).join("after");
    if mode == DocMode::Cli {
        create_dir(imgs_path.to_str().unwrap());
        create_dir(before_dir.to_str().unwrap());
        create_dir(after_dir.to_str().unwrap());
    }

    let out_path = Path::new(workdir).join("svgcleaner.rst");

    if mode == DocMode::Cli {
        let mut out_buf = BufWriter::new(File::create(&out_path).unwrap());
        out_buf.write(b".. This file is autogenerated. Do not edit it!\n\n").unwrap();
    }

    // process rst files
    let lines = load_lines(Path::new(&srcdir).join("order.txt").to_str().unwrap());
    for line in lines {
        let path = Path::new(&srcdir).join(&line);
        let path_str = path.to_str().unwrap();

        println!("{:?}", path_str);

        let doc_basename = basename(path_str);
        let data = Data {
            workdir: workdir.to_owned(),
            mode: mode.clone(),
            img_before_path: gen_svg_path(&before_dir, doc_basename),
            img_after_path: gen_svg_path(&after_dir, doc_basename),
        };

        if mode == DocMode::Cli {
            let file = OpenOptions::new().append(true).open(&out_path).unwrap();
            let mut out_buf = BufWriter::new(file);
            prepare_page(path_str, &data, &mut out_buf);
        } else {
            let path = Path::new(&outdir).join(&line);
            let mut out_buf = BufWriter::new(File::create(path).unwrap());

            prepare_page(path_str, &data, &mut out_buf);
        }
    }
}

pub fn is_rst_file(p: &Result<fs::DirEntry, io::Error>) -> bool {
    match p.as_ref().unwrap().path().extension() {
        Some(ext) => ext.to_str().unwrap() == "rst",
        None => false,
    }
}

fn gen_svg_path(dir: &PathBuf, basename: &str) -> String {
    let filename = String::from(basename) + ".svg";
    Path::new(&dir).join(filename).to_str().unwrap().to_owned()
}

fn prepare_page(page_path: &str, data: &Data, out_buf: &mut BufWriter<File>) {
    let lines = load_lines(page_path);
    let mut lines_iter = lines.iter();

    if data.mode == DocMode::Gui {
        // skip title
        lines_iter.next();
        lines_iter.next();
    }

    while let Some(line) = lines_iter.next() {
        if line == ".. GEN_TABLE" {
            if data.mode == DocMode::Cli {
                // write CLI arg before table
                write!(out_buf, "CLI argument: ``--{}``\n\n", basename(page_path)).unwrap();
            }

            let table = gen_table(&mut lines_iter, &data);
            out_buf.write(table.as_bytes()).unwrap();
            out_buf.write(b"\n").unwrap();
        } else {
            out_buf.write(line.as_bytes()).unwrap();
            out_buf.write(b"\n").unwrap();
        }
    }

    out_buf.write(b"\n").unwrap();
}

fn load_lines(path: &str) -> Vec<String> {
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    let mut lines = Vec::new();
    for line in f.lines() {
        lines.push(line.unwrap());
    }

    lines
}

#[derive(PartialEq)]
enum Column {
    Before,
    After,
}

fn gen_table(lines: &mut Iter<String>, data: &Data) -> String {
    let mut svg1: Vec<String> = Vec::new();
    let mut svg2: Vec<String> = Vec::new();

    let mut column = Column::Before;
    let mut insert_xmlns_xlink = true;

    while let Some(line) = lines.next() {
        if line == ".. BEFORE" {
            continue;
        } else if line == ".. NO_XMLNS_XLINK" {
            insert_xmlns_xlink = false;
            continue;
        } else if line == ".. AFTER" {
            column = Column::After;
            continue;
        } else if line == ".. END" {
            break;
        }

        let l = String::from_utf8_lossy(&line.as_bytes()[3..]).into_owned();

        if column == Column::Before {
            svg1.push(l);
        } else {
            svg2.push(l);
        }
    }

    let svg1_bytes = gen_svg_file(&svg1, insert_xmlns_xlink, Column::Before, data);
    let svg2_bytes = gen_svg_file(&svg2, insert_xmlns_xlink, Column::After, data);

    // vectors should have equal size for zip iter
    while svg1.len() > svg2.len() {
        svg2.push(String::new());
    }
    while svg2.len() > svg1.len() {
        svg1.push(String::new());
    }

    let mut svg1_max_len = svg1.iter().fold(0, |acc, ref x| cmp::max(acc, x.len())) + 2;
    let mut svg2_max_len = svg2.iter().fold(0, |acc, ref x| cmp::max(acc, x.len())) + 2;

    fn gen_img_link(path: &str, workdir: &str) -> String {
        let relative = Path::new(path).strip_prefix(workdir).unwrap().to_str().unwrap();
        format!(".. image:: https://razrfalcon.github.io/svgcleaner/{}", relative)
    }

    let img_link_before = gen_img_link(&data.img_before_path, &data.workdir);
    let img_link_after = gen_img_link(&data.img_after_path, &data.workdir);

    svg1_max_len = cmp::max(svg1_max_len, img_link_before.len());
    svg2_max_len = cmp::max(svg2_max_len, img_link_after.len());

    let mut hl = String::new();
    hl.push('+');
    fill_str(&mut hl, '-', svg1_max_len + 2);
    hl.push('+');
    fill_str(&mut hl, '-', svg2_max_len + 2);
    hl.push('+');

    let mut table = String::new();

    let add_line = |table: &mut String, leftpad, col1: &str, col2: &str| {
        table.push_str("| ");
        fill_str(table, ' ', leftpad);
        table.push_str(col1);
        fill_str(table, ' ', svg1_max_len - col1.len() + 1 - leftpad);
        table.push_str("| ");
        fill_str(table, ' ', leftpad);
        table.push_str(col2);
        fill_str(table, ' ', svg2_max_len - col2.len() + 1 - leftpad);
        table.push('|');
        table.push('\n');
    };

    table.push_str(&hl);
    table.push('\n');

    add_line(&mut table, 0, &format!("Before ({}b)", svg1_bytes),
                            &format!("After ({}b)", svg2_bytes));

    table.push_str(&hl);
    table.push('\n');

    if data.mode == DocMode::Cli {
        add_line(&mut table, 0, ".. code-block:: XML", ".. code-block:: XML");
    } else {
        add_line(&mut table, 0, "::", "::");
    }
    add_line(&mut table, 0, "", "");

    assert_eq!(svg1.len(), svg2.len());

    for (l1, l2) in svg1.iter().zip(svg2.iter()) {
        add_line(&mut table, 2, &l1, &l2);
    }

    if data.mode == DocMode::Cli {
        table.push_str(&hl);
        table.push('\n');

        add_line(&mut table, 0, &img_link_before, &img_link_after);
    }

    table.push_str(&hl);
    table.push('\n');

    table
}

fn fill_str(text: &mut String, c: char, len: usize) {
    for _ in 0..len {
        text.push(c);
    }
}

fn basename<'a>(path: &'a str) -> &str {
    Path::new(path).file_stem().unwrap().to_str().unwrap()
}

fn gen_svg_file(lines: &Vec<String>, insert_xmlns_xlink: bool, col: Column, data: &Data) -> usize {
    let svg_attrs = if insert_xmlns_xlink {
        "<svg xmlns=\"http://www.w3.org/2000/svg\" \
                    xmlns:xlink=\"http://www.w3.org/1999/xlink\" \
                    width=\"200\" height=\"100\""
    } else {
        "<svg xmlns=\"http://www.w3.org/000/svg\" \
                    width=\"200\" height=\"100\""
    };

    let mut s = String::new();
    for l in lines {
        if l.starts_with("<svg") {
            let line = l.replace("<svg", svg_attrs);
            s.push_str(&line);
        } else {
            s.push_str(l);
        }
        s.push('\n');
    }

    let mut f = match col {
        Column::Before => File::create(&data.img_before_path).unwrap(),
        Column::After => File::create(&data.img_after_path).unwrap(),
    };

    f.write(s.as_bytes()).unwrap();

    s.as_bytes().len()
}

fn create_dir(path: &str) {
    match fs::create_dir(path) {
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => {},
                _ => {
                    println!("{:?}", e.kind());
                    return;
                },
            }
        },
        Ok(_) => {},
    }
}