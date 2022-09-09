// Poor code, I know
use std::fs;
use std::fs::File;

const HTML_DATA: &str = "<!DOCTYPE html>
<html lang=\"en\">

<head>
    <meta charset=\"UTF-8\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <link rel=\"icon\" href=\"favicon.ico\" type=\"image/x-icon\">
    <link rel=\"stylesheet\" href=\"./style.css\" />
    <title>Title</title>
</head>

<body>
    <script src=\"./script.js\"></script>
</body>

</html>";

const CSS_DATA: &str = "body {
    margin: 0;
    background-color: black;
}";

fn main() {
    File::create("index.html").expect("Unable to write file");
    fs::write("index.html", HTML_DATA).expect("Unable to write file");
    File::create("style.css").expect("Unable to write file");
    fs::write("style.css", CSS_DATA).expect("Unable to write file");
    File::create("script.js").expect("Unable to write file");
    fs::write("script.js", "").expect("Unable to write file");
}
