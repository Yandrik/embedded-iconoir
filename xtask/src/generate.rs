use resvg::tiny_skia::{Pixmap, Transform};
use std::collections::BTreeMap as MapType;
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

type Icons = MapType<u32, MapType<IconSet, MapType<String, Vec<String>>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum IconSet {
    Regular,
    Solid,
}

impl IconSet {
    const ALL: &[Self] = &[Self::Regular, Self::Solid];

    fn feature_name(&self) -> Option<&'static str> {
        match self {
            IconSet::Regular => None,
            IconSet::Solid => Some("icon-set-solid"),
        }
    }
}

impl core::fmt::Display for IconSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IconSet::Regular => write!(f, "regular"),
            IconSet::Solid => write!(f, "solid"),
        }
    }
}

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

fn get_categories() -> anyhow::Result<MapType<String, String>> {
    let regex =
        Regex::new("\"(?P<icon>[^\"]+)\",\"(?P<category>[^\"]+)\"(?:,\"(?P<tags>[^\"]+)\",?)?")
            .unwrap();

    let mut map = MapType::new();
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
    categories: &MapType<String, String>,
    size: u32,
    target_dir: &Path,
) -> anyhow::Result<MapType<String, Vec<String>>> {
    // input: icon -> categories, output: categories -> icons (as written)
    fs::create_dir_all(target_dir)?;
    let mut out_map: MapType<String, Vec<String>> = MapType::new();

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
    icons: &MapType<IconSet, MapType<String, Vec<String>>>,
) -> anyhow::Result<()> {
    println!(
        "Generating module for {} icon categories of size {}px",
        icons.len(),
        size
    );
    /*
    sample:
        make_icon_category!(actions, 24, "regular", "Actions", [
            (AddCircle, "add-circle"),
            (Cancel, "cancel"),
            (Check, "check"),
            (DeleteCircle, "delete-circle"),
        ]);
     */

    assert!(
        !icons.is_empty(),
        "BUG: Cannot generate module for empty icon set"
    );

    writeln!(code, "#[cfg(feature = \"{size}px\")]")?;

    writeln!(code, "pub mod size{size}px {{\n")?;

    for (icon_set, icons) in icons {
        if let Some(feature_name) = icon_set.feature_name() {
            writeln!(code, "#[cfg(feature = \"{feature_name}\")]")?;
        }
        writeln!(code, "pub mod {icon_set} {{\n\nuse super::super::*;\n")?;
        for (cat, icon_list) in icons {
            println!(
                "{size}px-{icon_set}: making category {cat} for {} icons...",
                icon_list.len()
            );
            writeln!(
                code,
                "make_icon_category!({}, {size}, \"{icon_set}\", \"{cat}\", [",
                denumber(cat).to_snake_case(),
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
        writeln!(code, "}} // end of {icon_set} module")?;
    }

    // Keep the original `sizeXXpx::category` API as shorthand for the regular
    // icon set. Icon-set module names take precedence over flattened category
    // names, so colliding categories remain available through `regular::...`.
    let regular_icons = icons
        .get(&IconSet::Regular)
        .expect("BUG: regular icon set is missing");
    for category in regular_icons.keys() {
        let module_name = denumber(category).to_snake_case();
        if !IconSet::ALL
            .iter()
            .any(|icon_set| icon_set.to_string() == module_name)
        {
            writeln!(code, "pub use regular::{module_name};")?;
        }
    }

    writeln!(code, "}} // end of size{size}px module\n\n")?;

    Ok(())
}

fn gen_code(target_file: &Path, icons: Icons) -> anyhow::Result<()> {
    println!("generating code...");

    let mut code = String::new();

    code.push_str(
        "\
// This file is auto-generated. It is regenerated with each run of the build script.
// Please do not edit this file manually.
// Any changes you make will disappear when the file is overwritten the next build.
\n\n",
    );

    for (size, sets) in icons {
        gen_module(&mut code, size, &sets)?;
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
    const SIZES: &[u32] = &[12, 16, 18, 24, 32, 48, 96, 144];
    let categories = get_categories().unwrap();

    // MapType::<size, MapType<icon set, MapType<category, Vec<icon_name>>>>
    let mut icons = Icons::new();

    for icon_set in IconSet::ALL {
        let mut svgs: Vec<_> = WalkDir::new(PathBuf::from(ICONS_DIR).join(icon_set.to_string())) // WalkDir for potential future folders
            .max_depth(1)
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| {
                f.path()
                    .extension()
                    .map(|ext| ext.eq_ignore_ascii_case("svg"))
                    .unwrap_or(false)
            })
            .collect();
        svgs.sort_by_key(|entry| entry.file_name().to_owned());

        for size in SIZES {
            let rendered_icons = render_icons(
                &svgs,
                &categories,
                *size,
                PathBuf::from(TARGET_DIR)
                    .join(format!("{size}px-{icon_set}"))
                    .as_path(),
            )
            .unwrap_or_else(|e| panic!("Couldn't render {size}px-{icon_set} icons:\n{e}"));

            *icons
                .entry(*size)
                .or_default()
                .entry(*icon_set)
                .or_default() = rendered_icons;
        }
    }

    gen_code(&PathBuf::from(ICON_CODEGEN_TARGET_FILE), icons).unwrap();
}
