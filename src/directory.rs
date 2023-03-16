use std::path::PathBuf;

pub fn directory_listing(base: PathBuf, root_dir: PathBuf) -> String {
    let root = root_dir.to_string_lossy().to_string();
    let children = base.read_dir().expect("Failed to read the directory!");

    let mut body = String::new();

    for entry in children {
        let entry = entry.unwrap();
        let path = entry.path();

        let href = path.to_string_lossy().replacen(&root, "", 1);
        if path.is_dir() {
            body += &format!(
                "<li><a href=\"{}\">{}/</a></li>\n",
                href,
                entry.file_name().to_string_lossy(),
            )
            .to_string();
        } else {
            body += &format!(
                "<li><a href=\"{}\">{}</a></li>\n",
                href,
                entry.file_name().to_string_lossy(),
            )
            .to_string();
        }
    }

    let title = base.to_string_lossy();

    format!(
        "<!DOCTYPE HTML>
<html>
<head><title>{}</title></head>
<body>
<h3>{}</h3>
<ul>
{}
</ul>
</body>
</html>\n
<style>\n{}</style>",
        title,
        title,
        body,
        include_str!("../assets/style.css")
    )
}
