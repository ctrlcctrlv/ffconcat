# ffconcat

## What is it?

ffconcat.rs is a tiny program that outputs ffmpeg concat filter compatible files from the current directory to stdout. If args provided, reads those, else reads current directory.

## Why?

I wanted to make a simple program that could output a list of files in a directory in a format that ffmpeg could use.

## Using library

```no_run
use std::io::{BufWriter, Write};
use std::fs::File;

let mut writer = BufWriter::new(File::create("output.txt").unwrap());
ffconcat(writer).unwrap();
```
