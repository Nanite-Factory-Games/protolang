use std::{fs, path::Path};
use crate::model::{File, Function, Parameter, Statement, Type};

pub fn write_files(output_folder: &Path, files: &Vec<File>) -> anyhow::Result<()> {
    for file in files {
        fs::write(
            output_folder.join(format!("{}.cs",file.module.clone())),
            generate_file_contents(file)
        )?;
    }
    Ok(())
}

pub fn generate_file_contents(file: &File) -> String {
    "".to_string()
}