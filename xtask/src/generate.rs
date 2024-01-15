use resvg::tiny_skia::{Pixmap, Transform};
use std::collections::HashMap;
use std::fs;
use usvg::{FitTo, Tree};

use anyhow::Error;
use bit_vec::BitVec;
use heck::{ToSnakeCase, ToUpperCamelCase};
use regex::Regex;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

const TARGET_DIR: &str = "./embedded-iconoir/rendered";
const ICONS_DIR: &str = "./iconoir/icons";
const CATEGORIES_FILE_PATH: &str = "iconoir/iconoir.com/icons.csv";
const EXTENSION: &str = "bits";

const ICON_CODEGEN_TARGET_FILE: &str = "./embedded-iconoir/src/icons.gen.rs";

const fn alpha_cutoff(size: u32) -> u8 {
    match size {
        0..=16 => 0x40,
        _ => 0x60,
    }
}

fn render_icon_to_bits(path: &Path, size: u32) -> anyhow::Result<BitVec<u8>> {
    assert!(size > 0, "BUG: Cannot render icon for size 0");

    if !path.exists() {
        return Err(Error::msg(format!("No file at path {:?}", path)));
    }

    let mut pixmap = Pixmap::new(size, size).unwrap();

    resvg::render(
        &Tree::from_str(&fs::read_to_string(path)?, &Default::default())?,
        FitTo::Size(size, size),
        Transform::default(),
        pixmap.as_mut(),
    );

    let result: BitVec<u8> = pixmap
        .data()
        .iter()
        .enumerate()
        .filter(|(a, _)| a % 4 == 3 /* select alpha channel */)
        .map(|(_, b)| *b) // discard index
        .map(|alpha| alpha > alpha_cutoff(size))
        .collect();

    Ok(result)
}

#[allow(unused)]
fn panic_render_debug(icon: &BitVec, size: u32) -> ! {
    let mut out = String::new();

    let size = size as usize;

    writeln!(out, "raw: {:?}", icon).unwrap();

    writeln!(out, "\npoints:\n\n").unwrap();

    let points: String = icon.iter().map(|d| if d { ".+." } else { "   " }).collect();

    for i in 0..size {
        writeln!(out, "{:?}", &points[size * 3 * i..size * 3 * (i + 1)]).unwrap();
    }

    panic!("{}", out);
}

fn get_categories() -> anyhow::Result<HashMap<String, String>> {
    let regex =
        Regex::new("\"(?P<icon>[^\"]+)\",\"(?P<category>[^\"]+)\"(?:,\"(?P<tags>[^\"]+)\",?)?")
            .unwrap();

    let mut map = HashMap::new();
    let text = fs::read_to_string(CATEGORIES_FILE_PATH)?;

    for icon_meta in regex.captures_iter(&text) {
        map.insert(
            String::from(icon_meta.name("icon").unwrap().as_str()),
            String::from(icon_meta.name("category").unwrap().as_str()),
        );
    }

    Ok(map)
}

fn render_icons(
    files: &[DirEntry],
    categories: &HashMap<String, String>,
    size: u32,
    target_dir: &Path,
) -> anyhow::Result<HashMap<String, Vec<String>>> {
    // input: icon -> categories, output: categories -> icons (as written)
    fs::create_dir_all(target_dir)?;
    let mut out_map: HashMap<String, Vec<String>> = HashMap::new();

    for file in files {
        assert!(
            file.path().file_stem().is_some(),
            "File Path {:?} is invalid!",
            file
        );

        let category: PathBuf = categories
            .get::<String>(&String::from(
                file.path().file_stem().unwrap().to_str().unwrap(),
            ))
            .map(|s| s.as_str())
            .unwrap_or("NoCategory")
            .into();

        if !target_dir.join(&category).exists() {
            fs::create_dir(target_dir.join(&category))?;
        }

        let bits = render_icon_to_bits(file.path(), size)?;
        println!(
            "writing {:?}... ({:?})",
            file.path(),
            file.path().file_stem().unwrap().to_str().unwrap()
        );
        let mut target_file = target_dir
            .join(&category)
            .join(file.path().file_stem().unwrap().to_str().unwrap());
        target_file.set_extension(EXTENSION);

        println!("target-file: {:?}", target_file);

        match fs::write(&target_file, bits.blocks().collect::<Vec<_>>()) {
            Ok(_) => {
                let file_id = target_file.file_stem().unwrap().to_str().unwrap();
                let cat = String::from(category.to_str().unwrap());
                if let Some(vec) = out_map.get_mut(&cat) {
                    vec.push(String::from(file_id));
                } else {
                    out_map.insert(cat, vec![String::from(file_id)]);
                }
            }
            Err(e) => return Err(Error::new(e)),
        }
    }

    Ok(out_map)
}

fn denumber(s: &str) -> String {
    s.replace('1', "One") // to fix 2x2 cell icon
        .replace('2', "Two")
        .replace('3', "Three")
        .replace('4', "Four")
        .replace('5', "Five")
        .replace('6', "Six")
        .replace('7', "Seven")
        .replace('8', "Eight")
        .replace('9', "Nine")
        .replace('0', "Zero")
}

fn gen_module(
    code: &mut String,
    size: u32,
    icons: &HashMap<String, Vec<String>>,
) -> anyhow::Result<()> {
    println!(
        "Generating module for {} icon categories of size {}px",
        icons.len(),
        size
    );
    /*
    sample:
        make_icon_category!(actions, 24, "Actions", [
            (AddCircle, "add-circle"),
            (Cancel, "cancel"),
            (Check, "check"),
            (DeleteCircle, "delete-circle"),
        ]);
     */

    writeln!(code, "#[cfg(feature = \"{}px\")]", size)?;
    writeln!(code, "pub mod size{}px {{ \nuse super::*; \n", size)?;
    for (cat, icon_list) in icons {
        println!(
            "{}px: making category {} for {} icons...",
            size,
            cat,
            icon_list.len()
        );
        writeln!(
            code,
            "make_icon_category!({}, {}, \"{}\", [",
            denumber(cat).to_snake_case(),
            size,
            cat
        )?;
        for icon_name in icon_list {
            writeln!(
                code,
                "      ({}, \"{}\"),",
                denumber(icon_name).to_upper_camel_case(),
                icon_name.replace(".bits", "")
            )?;
        }
        writeln!(code, "]);")?;
    }
    writeln!(code, "//end of {}px module\n}}\n", size)?;
    Ok(())
}

fn gen_code(
    target_file: &Path,
    icons: Vec<(u32, HashMap<String, Vec<String>>)>,
) -> anyhow::Result<()> {
    println!("generating code...");

    let mut code = String::new();

    code.push_str(
        "\
// This file is auto-generated. It is regenerated with each run of the build script.
// Please do not edit this file manually.
// Any changes you make will disappear when the file is overwritten the next build.
\n\n",
    );

    for (size, these_icons) in icons {
        gen_module(&mut code, size, &these_icons)?;
    }

    if !target_file
        .parent()
        .expect("codegen target file has no parent")
        .exists()
    {
        return Err(Error::msg(format!(
            "The file's parent directory ({:?}) doesn't exist. Please make sure it does.",
            target_file.parent().unwrap()
        )));
    }

    fs::write(target_file, code)?;

    Ok(())
}

pub fn main() {
    let sizes = vec![12, 16, 18, 24, 32, 48, 96, 144];

    // panic!("{:#?}", get_categories());

    let svgs: Vec<_> = WalkDir::new(ICONS_DIR) // WalkDir for potential future folders
        .max_depth(1)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| {
            f.path()
                .extension()
                .map(|ext| ext.to_ascii_lowercase() == "svg")
                .unwrap_or(false)
        })
        .collect();

    let mut maps = vec![];

    for size in sizes {
        let icons = render_icons(
            &svgs,
            &get_categories().unwrap(),
            size,
            PathBuf::from(TARGET_DIR)
                .join(format!("{}px", size))
                .as_path(),
        )
        .expect("Couldn't render 24px icons");

        maps.push((size, icons));
    }

    gen_code(&PathBuf::from(ICON_CODEGEN_TARGET_FILE), maps).unwrap();

    // panic!("svgs: {:#?}", svgs);

    // let icon = icon_to_bits(PathBuf::from("./iconoir/icons/3d-select-face.svg"), size).unwrap();

    // panic_render_debug(&icon, size);
}
