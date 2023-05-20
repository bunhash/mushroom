WZ
==

Library for reading and writing WZ archives.

## WZ Archives

Denoted with `.wz` extension. These files contain binary files \(`images`\) organized in a directory-like structure.

Each image within the WZ archives should be treated as independent binary blobs.

Example:

```rust no_run
use wz::archive::{self, reader::Node};
use wz::image;
use wz::io::WzImageReader;

// The archive::Reader is used to map the WZ archive
let mut archive = archive::Reader::unencrypted("Character.wz").unwrap();
let map = archive.map("Character.wz").unwrap();

// Once mapped, we can consume the Reader and take the underlying WzRead
let mut reader = archive.into_inner();

println!("{:?}", map.debug_pretty_print());

// The WzRead can be wrapped to read an image embedded in the archive
let img_offset = match map.get("Character.wz/Weapon/01472030.img").unwrap() {
    Node::Image { offset, .. } => *offset,
    _ => panic!("01472030.img should be an Image"),
};

// Now we can read and map the image
let img_reader = WzImageReader::with_offset(&mut reader, img_offset);
let mut image = image::Reader::new(img_reader);
let img_map = image.map("01472030.img").unwrap();

println!("{:?}", img_map.debug_pretty_print());
```
