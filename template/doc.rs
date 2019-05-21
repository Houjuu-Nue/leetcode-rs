
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;


// Command: cargo r --bin doc-gen p00xx 17 generic

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let doc_module  = args[1].clone();
    let doc_problem = args[2].clone();
    let doc_type    = args[3].clone();

    // write src/pxxx/pxxx.rs
    let file_path = format!("./src/{}/p{}.rs", doc_module, doc_problem);
    let mut file_problem = match OpenOptions::new()
        .read(true).write(true)
        .open(file_path.clone()) {
        
        | Ok(file) => translate_doc(file, file_path.clone())?,
        | Err(_) => File::create(file_path)?,
    };
    match doc_type.as_ref() {
        | "generic"  => file_problem.write_all(&include_str!("./template_p_generic.txt" ).to_string().into_bytes())?,
        | "linklist" => file_problem.write_all(&include_str!("./template_p_linklist.txt").to_string().into_bytes())?,
        | _ => panic!("unknown problem type"),
    }
    file_problem.flush()?;

    // write src/pxxx/mod.rs
    let p_mod_path    = format!("./src/{}/mod.rs", doc_module);
    let p_mod_content = format!("pub mod p{};\n", doc_problem);
    write_mod_info(p_mod_path, p_mod_content)?;

    // write test/pxxx/txxx.rs
    let file_path = format!("./tests/{}/t{}.rs", doc_module, doc_problem);
    let mut file_test = File::create(&file_path)?;
    let mut test_content = match doc_type.as_ref() {
        | "generic"  => include_str!("./template_t_generic.txt" ).to_string(),
        | "linklist" => include_str!("./template_t_linklist.txt").to_string(),
        | _ => panic!("unknown problem type"),
    };
    test_content = test_content.replace("pxxx", &format!("p{}", doc_problem));
    test_content = test_content.replace("txxx", &format!("t{}", doc_problem));
    file_test.write_all(&test_content.into_bytes())?;
    file_test.flush()?;

    // write test/pxxx/mod.rs
    let t_mod_path    = format!("./tests/{}/mod.rs", doc_module);
    let t_mod_content = format!("#[cfg(test)] pub mod t{};\n", doc_problem);
    write_mod_info(t_mod_path, t_mod_content)?;

    Ok(())
}


fn translate_doc(mut file: File, path: String) -> std::io::Result<File> {

    const LINE_PREFIX_V1: &str = "//! ";
    const LINE_PREFIX_V2: &str = "//!";
    
    let mut content = String::new();
    let mut translation = String::new();

    file.read_to_string(&mut content)?;
    drop(file);
    fs::remove_file(path.clone())?;

    let mut lines = content.lines();
    let problem_title = lines.next()
        .expect("Failed to location problem title!");
    let problem_url = lines.next()
        .expect("Failed to location problem URL!");
    
    translation.push_str(LINE_PREFIX_V2);
    translation.push('\n');
    translation.push_str(LINE_PREFIX_V1);
    translation.push_str(problem_title);
    translation.push('\n');
    translation.push_str(LINE_PREFIX_V2);
    translation.push('\n');
    translation.push_str(LINE_PREFIX_V1);
    translation.push_str(problem_url);
    translation.push('\n');
    translation.push_str(LINE_PREFIX_V2);
    translation.push('\n');


    let mut text_flag = false;

    while let Some(line) = lines.next() {

        if text_flag {
            if line.is_empty() {
                translation.push_str(LINE_PREFIX_V1);
                translation.push_str("```");
                translation.push('\n');
                translation.push_str(LINE_PREFIX_V2);
                translation.push('\n');
                text_flag = false;
            } else {
                translation.push_str(LINE_PREFIX_V1);
                translation.push_str(line);
                translation.push('\n');
            }
        } else if line.starts_with("Example") {
            translation.push_str(LINE_PREFIX_V1);
            translation.push_str("**");
            translation.push_str(line);
            translation.push_str("**");
            translation.push('\n');
            translation.push_str(LINE_PREFIX_V1);
            translation.push_str("```text");
            translation.push('\n');
            lines.next();

            text_flag = true;
        } else if line.starts_with("Note") || line.starts_with("Clarification") {
            translation.push_str(LINE_PREFIX_V1);
            translation.push_str("**");
            translation.push_str(line);
            translation.push_str("**");
            translation.push('\n');
        } else {
            if text_flag {
                translation.push_str(LINE_PREFIX_V1);
                translation.push_str(line);
                translation.push('\n');
            } else {
                let sentences: Vec<&str> = line.split('.')
                    .filter(|s| !s.is_empty())
                    .collect();
                for sentence in sentences {
                    if sentence.starts_with(' ') {
                        translation.push_str(LINE_PREFIX_V2);
                    } else {
                        translation.push_str(LINE_PREFIX_V1);
                    }
                    
                    translation.push_str(sentence);
                    if sentence.ends_with(':') == false && sentence.ends_with('?') == false {
                        translation.push('.');
                    }
                    translation.push_str("\n");
                    translation.push_str(LINE_PREFIX_V2);
                    translation.push_str("\n");
                }
            }
        }
    }

    translation.push_str(LINE_PREFIX_V2);
    translation.push_str("\n\n");

    let mut file = File::create(&path)?;
    file.write_all(&translation.into_bytes())?;
    Ok(file)
}

fn write_mod_info(file_path: String, content: String) -> std::io::Result<()> {

    let mut file_mod = OpenOptions::new()
        .write(true).read(true).append(true)
        .open(&file_path)?;
    let mut file_content = String::new();
    file_mod.read_to_string(&mut file_content)?;

    if file_content.contains(&content) == false {
        write!(file_mod, "{}", content)?;
        file_mod.flush()?;
    }

    Ok(())
}
