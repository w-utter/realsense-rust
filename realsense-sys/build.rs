extern crate bindgen;

use failure::Fallible;
use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
};

fn main() {
    // Tell cargo to tell rustc to link the system shared library
    println!("cargo:rustc-link-lib=realsense2");

    // Probe libary
    let library = probe_library("realsense2").unwrap();

    // Verify version
    let (include_dir, version) = library
        .include_paths
        .iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .filter_map(|path| {
            let dir = Path::new(path).join("librealsense2");
            if dir.is_dir() {
                match get_version_from_header_dir(&dir) {
                    Some(version) => Some((dir, version)),
                    None => None,
                }
            } else {
                None
            }
        })
        .next()
        .expect("fail to detect librealsense2 version");

    assert_eq!(
        &version.major,
        "2",
        "librealsense2 version {} is not supported",
        version.to_string()
    );

    let bindings = bindgen::Builder::default()
        .header(include_dir.join("rs.h").to_str().unwrap())
        .header(
            include_dir
                .join("h")
                .join("rs_pipeline.h")
                .to_str()
                .unwrap(),
        )
        .header(
            include_dir
                .join("h")
                .join("rs_advanced_mode_command.h")
                .to_str()
                .unwrap(),
        )
        .header(include_dir.join("h").join("rs_config.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn get_version_from_header_dir<P>(dir: P) -> Option<Version>
where
    P: AsRef<Path>,
{
    let header_path = dir.as_ref().join("rs.h");

    let mut major_opt: Option<String> = None;
    let mut minor_opt: Option<String> = None;
    let mut patch_opt: Option<String> = None;
    let mut build_opt: Option<String> = None;

    let mut reader = BufReader::new(File::open(header_path).ok()?);
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => return None,
            _ => (),
        }

        const PREFIX: &str = "#define RS2_API_";
        if line.starts_with(PREFIX) {
            let mut tokens = line[PREFIX.len()..].split_whitespace();
            let name_opt = tokens.next();
            let version_opt = tokens.next();

            if let (Some(name), Some(version)) = (name_opt, version_opt) {
                let version_owned = version.to_owned();
                match name {
                    "MAJOR_VERSION" => major_opt = Some(version_owned),
                    "MINOR_VERSION" => minor_opt = Some(version_owned),
                    "PATCH_VERSION" => patch_opt = Some(version_owned),
                    "BUILD_VERSION" => build_opt = Some(version_owned),
                    _ => (),
                }
            }
        }

        if major_opt.is_some() && minor_opt.is_some() && patch_opt.is_some() && build_opt.is_some()
        {
            let version = Version {
                major: major_opt.unwrap(),
                minor: minor_opt.unwrap(),
                patch: patch_opt.unwrap(),
                build: build_opt.unwrap(),
            };

            return Some(version);
        }
    }
}

fn probe_library(pkg_name: &str) -> Fallible<Library> {
    let package = pkg_config::probe_library(pkg_name)?;
    let lib = Library {
        pkg_name: pkg_name.to_owned(),
        libs: package.libs,
        link_paths: package.link_paths,
        framework_paths: package.framework_paths,
        include_paths: package.include_paths,
        version: package.version,
        prefix: PathBuf::from(pkg_config::get_variable(pkg_name, "prefix")?),
        libdir: PathBuf::from(pkg_config::get_variable(pkg_name, "libdir")?),
    };
    Ok(lib)
}

#[derive(Debug, Clone)]
struct Version {
    major: String,
    minor: String,
    patch: String,
    build: String,
}

impl Version {
    pub fn to_string(&self) -> String {
        let Self {
            major,
            minor,
            patch,
            build,
        } = self;
        format!("{}.{}.{}.{}", major, minor, patch, build)
    }
}

#[derive(Debug)]
struct Library {
    pub pkg_name: String,
    pub libs: Vec<String>,
    pub link_paths: Vec<PathBuf>,
    pub framework_paths: Vec<PathBuf>,
    pub include_paths: Vec<PathBuf>,
    pub version: String,
    pub prefix: PathBuf,
    pub libdir: PathBuf,
}