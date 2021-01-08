use std::{fs::read_to_string, path::PathBuf};

use harmonizer::{harmonize, Result::*, ServiceDefinition};

fn main() {
  let composed =
    harmonize(std::env::args().skip(2)
      .map(|file| {
        let src = read_to_string(&file)
          .expect("reading source file");
        ServiceDefinition::new(
          PathBuf::from(&file).file_stem()
            .expect("path must point to a schema file")
            .to_str()
            .expect("os string decoding"),
          &file,
          src
        )
      })
      .collect());
  
  match composed {
    Ok(schema) => { println!("{}", schema); },
    Err(errors) => {
      for err in &errors {
        eprintln!("{}", err.message);
      }

      eprintln!("{} {} during composition",
        errors.len(),
        if errors.len() == 1 { "error" } else { "errors" });
    }
  }
}