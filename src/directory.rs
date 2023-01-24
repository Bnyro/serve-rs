use std::path::PathBuf;

pub fn directory_listing(base: PathBuf, root_dir: PathBuf) -> String {
    
    let root = root_dir.to_string_lossy().to_string();
    let children = base.read_dir().unwrap();

    let mut body = String::new();

    for entry in children {
            let entry = entry.unwrap();
            let path = entry.path();

            let href = path.to_string_lossy().replacen(&root, "", 1);
            if path.is_dir() {
                body += &format!(
                    "<li><a href=\"{}\">{}/</a></li>",
                    href,
                    entry.file_name().to_string_lossy(),
                ).to_string();
            } else {
                body += &format!(
                    "<li><a href=\"{}\">{}</a></li>",
                    href,
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