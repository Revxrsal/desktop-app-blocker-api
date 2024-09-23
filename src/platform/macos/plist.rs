use io::{Error, ErrorKind};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use regex_macro::regex;

pub fn bundle_id_from_app(app_path: &Path) -> Result<String, Error> {
    let plist_path = app_path.join("Contents/Info.plist");
    let mut file = File::open(&plist_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    bundle_id_from_xml(&content)
}

pub fn bundle_id_from_xml(content: &str) -> Result<String, Error> {
    let bundle_id_matcher = regex!(r#"<key>CFBundleIdentifier</key>\s*<string>(.*?)</string>"#);

    match bundle_id_matcher.captures(content) {
        Some(caps) => Ok(caps[1].to_string()),
        None => Err(Error::new(ErrorKind::Other, "Failed to parse Info.plist")),
    }
}
