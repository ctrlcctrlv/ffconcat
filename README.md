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

## License

```
Copyright 2023 Fredrick R. Brennan

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this software or any of the provided source code files except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied.  See the License for the
specific language governing permissions and limitations under the License.
```

**By contributing you release your contribution under the terms of the license.**
