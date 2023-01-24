use std::path::PathBuf;

pub fn directory_listing(base: PathBuf) -> String {
    let children = base.read_dir().unwrap();

    let mut body = String::new();

    for entry in children {
            let entry = entry.unwrap();
            let path = entry.path();
            let p = match path.strip_prefix(&base) {
                Ok(p) if cfg!(windows) => base.join(p).to_string_lossy().replace('\\', "/"),
                Ok(p) => base.join(p).to_string_lossy().into_owned(),
                Err(_) => continue,
            };

            if path.is_dir() {
                body += &format!(
                    "<li><a href=\"{}\">{}/</a></li>",
                    p,
                    entry.file_name().to_string_lossy(),
                ).to_string();
            } else {
                body += &format!(
                    "<li><a href=\"{}\">{}</a></li>",
                    p,
                    entry.file_name().to_string_lossy(),
                ).to_string();
            }
        }

    format!(
        "<html>\
         <head><title>{}</title></head>\
         <body><h1>{}</h1>\
         <ul>\
         {}\
         </ul></body>\n</html>",
        "", "", body
    )
}