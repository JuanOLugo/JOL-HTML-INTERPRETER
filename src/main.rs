use std::{
    env,
    fs::{ self, File },
    io::{ BufRead, BufReader, BufWriter, Write },
    ops::Index,
    path::{ PathBuf },
};

fn main() {
    let mut commands: Vec<String> = vec!["-c".to_string()];
    let args: Vec<String> = env::args().collect();
    if args[1] == "run" {
        for (i, a) in args.iter().enumerate() {
            if a != "run" && i > 0 {
                let _ = match verify_argument(&mut commands, args.clone(), i, a) {
                    Ok(item) => {
                        let path: PathBuf = get_path_save(&item, "index.html");
                        let path_str = path.to_str().unwrap_or("index.html");
                        read_file(&item, path_str);

                        break;
                    }
                    Err(err) => panic!("Error to verify argument, {}", err),
                };
            }
        }
    }
}

fn get_path_save(path: &str, new_path: &str) -> PathBuf {
    let mut original_path = PathBuf::from(path);

    original_path.pop();
    original_path.push(new_path);

    return original_path;
}

fn write_on_file(file: &mut File, item_write: String) {
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", item_write).unwrap();
}

fn read_file(path: &str, save_path: &str) {
    let items_language: Vec<&str> = vec![
        "@title",
        "@subtitle",
        "@paragraph",
        "@-",
        "@*",
        "@link",
        "@list",
        "@endlist",
        "@item",
        "@code",
        "@endcode",
        "@warn",
        "@info",
        "@image",
        "@quote",
        "@small"
    ];

    let file = fs::File::open(path).expect("Cannot read file");
    let reader = BufReader::new(file);
    let mut file_to_write = File::create(save_path).expect("Error creating file");
    making_styles(&mut file_to_write);
    for line in reader.lines() {
        let line = line.expect("Error reading line").trim().to_string();
        let mut matched = false;

        for token in &items_language {
            if line.starts_with(token) {
                let content = line.strip_prefix(token).unwrap_or("").trim();
                matched = true;

                match *token {
                    "@title" => write_on_file(&mut file_to_write, format!("<h1>{}</h1>", content)),
                    "@subtitle" =>
                        write_on_file(&mut file_to_write, format!("<h3>{}</h3>", content)),
                    "@paragraph" =>
                        write_on_file(&mut file_to_write, format!("<p>{}</p>", content)),
                    "@-" => write_on_file(&mut file_to_write, "<hr/>".to_string()),
                    "@*" => write_on_file(&mut file_to_write, format!("<!-- {} -->", content)),
                    "@list" => write_on_file(&mut file_to_write, "<ul>".to_string()),
                    "@endlist" => write_on_file(&mut file_to_write, "</ul>".to_string()),
                    "@item" => write_on_file(&mut file_to_write, format!("<li>{}</li>", content)),
                    "@code" => write_on_file(&mut file_to_write, "<pre><code>".to_string()),
                    "@endcode" => write_on_file(&mut file_to_write, "</code></pre>".to_string()),
                    "@info" =>
                        write_on_file(
                            &mut file_to_write,
                            format!("<div class='info-box'>{}</div>", content)
                        ),
                    "@warn" =>
                        write_on_file(
                            &mut file_to_write,
                            format!("<div class='warn-box'>{}</div>", content)
                        ),
                    "@link" =>
                        write_on_file(
                            &mut file_to_write,
                            format!("<a href='{}' target='_blank'>{}</a>", content, content)
                        ),
                    "@image" =>
                        write_on_file(
                            &mut file_to_write,
                            format!("<img src='{}' alt='image'/>", content)
                        ),
                    "@quote" =>
                        write_on_file(
                            &mut file_to_write,
                            format!("<blockquote>{}</blockquote>", content)
                        ),
                    "@small" =>
                        write_on_file(&mut file_to_write, format!("<small>{}</small>", content)),
                    _ => {}
                }

                break; // ✅ ya coincidió, no sigas buscando
            }
        }

        // Si no coincidió con ningún decorador, escribe la línea tal cual
        if !matched && !line.is_empty() {
            write_on_file(&mut file_to_write, line);
        }
    }
}

fn verify_argument(
    commands: &mut Vec<String>,
    args: Vec<String>,
    i: usize,
    a: &String
) -> Result<String, bool> {
    if commands.contains(a) {
        let item_arg: &String = args.index(i + 1);
        if item_arg.trim() == "" {
            panic!("Error: el valor esta mal escrito");
        }
        return Ok(String::from(item_arg));
    } else {
        println!("Command not exist: {}", a);
        Err(false)
    }
}

fn making_styles(file_to_write: &mut File) {
    write_on_file(file_to_write, "<!DOCTYPE html>".to_string());
    write_on_file(file_to_write, "<html lang='es'>".to_string());
    write_on_file(file_to_write, "<head>".to_string());
    write_on_file(file_to_write, "<meta charset='UTF-8'>".to_string());
    write_on_file(
        file_to_write,
        "<meta name='viewport' content='width=device-width, initial-scale=1.0'>".to_string()
    );
    write_on_file(file_to_write, "<title>Documento Generado</title>".to_string());
    write_on_file(
        file_to_write,
        r#"
    <style>
        body {
            font-family: Arial, sans-serif;
            background-color: #f8f9fa;
            color: #333;
            margin: 40px;
            line-height: 1.6;
        }
        h1, h3 {
            color: #2c3e50;
        }
        p {
            margin-bottom: 1em;
        }
        .warn-box {
            background-color: #ffe5e5;
            border-left: 5px solid #e74c3c;
            padding: 10px;
            margin: 10px 0;
        }
        .info-box {
            background-color: #e8f4ff;
            border-left: 5px solid #3498db;
            padding: 10px;
            margin: 10px 0;
        }
        code {
            background: #f4f4f4;
            border-radius: 4px;
            padding: 5px;
            display: block;
            white-space: pre-wrap;
            font-family: Consolas, monospace;
        }
        blockquote {
            border-left: 4px solid #aaa;
            padding-left: 10px;
            font-style: italic;
            color: #555;
            background: #f9f9f9;
        }
        img {
            max-width: 100%;
            border-radius: 6px;
            margin: 10px 0;
        }
        small {
            color: #888;
        }
        ul {
            margin-left: 20px;
        }
        hr {
            margin: 20px 0;
        }
    </style>
    "#.to_string()
    );
    write_on_file(file_to_write, "</head><body>".to_string());
}
