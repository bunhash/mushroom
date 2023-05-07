Crypto
======

Mushroom crypto implementation

Handles the AES-OFB encryption used to encrypt/decrypt strings in WZ packages.

## Example

```
use crypto::{Decryptor, Encryptor, KeyStream, GMS_IV, TRIMMED_KEY};

let mut stream = KeyStream::new(&TRIMMED_KEY, &GMS_IV);

let mut input: Vec<u8> = Vec::from("smap.img".as_bytes());
stream.encrypt(&mut input);
assert_eq!(input.as_slice(), &[229, 195, 94, 212, 102, 147, 176, 247]);

stream.decrypt(&mut input);
assert_eq!(
    String::from_utf8(input).unwrap(),
    "smap.img"
);
```
