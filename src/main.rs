#![doc = include_str!("../README.md")]
use std::borrow::Cow;
use std::ffi::OsStr;
use std::io::{BufWriter, Write};
use std::os::unix::prelude::OsStrExt;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    ffconcat(BufWriter::new(io::stdout()))
}

/// Writes the ffconcat output to a writer.
///
/// # Examples
///
/// ```no_run
/// use std::io::{BufWriter, Write};
/// use std::fs::File;
///
/// let mut writer = BufWriter::new(File::create("output.txt").unwrap());
/// ffconcat(writer).unwrap();
/// ```
pub fn ffconcat<W: Write>(mut writer: BufWriter<W>) -> io::Result<()> {
    writer.write(b"ffconcat version 1.0\n")?;

    for _ in env::args_os()
    //list of arguments passed in
    {
        let mut files: Vec<Cow<'_, OsStr>> =
            get_file_vec(env::args_os().map(|e| Cow::Owned(e)).collect());
        files.sort_unstable(); //sort by filename

        for f in files
        //go through all the files
        {
            let ft: &[u8] = f.as_bytes(); //get bytes
            let ffilename: String = ffmpeg_escape_filepath(ft); //escape filepath

            writer.write(b"file '")?;
            writer.write(ffilename.as_bytes())?;
            writer.write(b"'\n")?;
        }
    }
    Ok(())
}

/// Escapes a filepath for ffmpeg concat filter.
///
/// # Examples
///
/// ```
/// assert_eq!(ffmpeg_escape_filepath(b"hello world.txt"), "hello\\ world.txt");
/// ```
fn ffmpeg_escape_filepath(filepath: &[u8]) -> String {
    let escaped_str: String = filepath
        .iter()
        .map(|&c| match c {
            //0x20 – 0x7E are the valid printable characters.
            //Single quotes need escaping.
            b' ' | b'\'' => format!("\\{}", c as char),
            //if character not within above range
            z if z < 0x20 || z > 0x7E => format!("\\{:o}", z),
            //if within range, just return character
            z => (z as char).into(),
        })
        .collect();

    return escaped_str;
}

/// Returns a vector of filepaths. If no args, returns files in current directory.
///
/// # Examples
///
/// ```
/// let mut files = get_file_vec(0, Vec::new());
/// assert!(files.len() > 0);
/// ```
fn get_file_vec(args: Vec<Cow<'_, OsStr>>) -> Vec<Cow<'_, OsStr>> {
    //if no args, take with current directory
    if args.len() <= 1 {
        get_dir(Cow::Borrowed(OsStr::new("."))).into_iter().collect()
    } else {
        args.into_iter().map(|dir|get_dir(dir)).flatten().collect()
    }
}

fn get_dir<'a>(dir: Cow<'a, OsStr>) -> Vec<Cow<'a, OsStr>> {
    fs::read_dir(dir)
        .into_iter()
        .flat_map(|d|d.into_iter())
        .filter_map(Result::ok)
        .flat_map(|dd|{
            let d = Cow::Owned(dd.path().as_os_str().to_owned());
            if dd.path().is_dir() {
                get_dir(d)
            } else {
                vec![d]
            }
        })
        .map(|d|d.into())
        .map(|e|Cow::Owned(e))
        .collect::<Vec<_>>()
}
