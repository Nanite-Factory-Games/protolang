use std::{fs, path::Path};
use crate::model::{File, Function, Parameter, Statement, Type};

pub fn write_files(output_folder: &Path, files: &Vec<File>) -> anyhow::Result<()> {
    for file in files {
        fs::write(
            output_folder.join(format!("{}.py",file.module.clone())),
            generate_file_contents(file)
        )?;
    }
    Ok(())
}

pub fn generate_file_contents(file: &File) -> String {
    let mut functions = Vec::new();
    for function in &file.functions {
        functions.push(generate_function(function));
    }
    return functions.join("\n");
}

pub fn generate_statements(params: &Vec<Statement>) -> String {
    return "".to_string();
}

pub fn generate_parameters(params: &Vec<Parameter>) -> String {
    return params
        .iter()
        .map(|p| format!("{}: {}", p.name, generate_type(&p.ty)))
        .collect::<Vec<String>>()
        .join(", ");
}

pub fn generate_return_type(ty: &Option<Type>) -> String {
    return match ty {
        Some(ty) => format!("-> {}", generate_type(ty)),
        None => "".to_string()
    };
}

pub fn generate_type(ty: &Type) -> String {
    match ty {
        Type::Int => "int".to_string(),
        Type::Float => "float".to_string(),
        Type::String => "str".to_string(),
        Type::Struct(_) => "struct".to_string()
    }
}

pub fn generate_function(func: &Function) -> String {
    return format!(
        "def {}({}) {}:\n{}",
        func.name,
        generate_parameters(&func.parameters),
        generate_return_type(&func.return_type),
        generate_statements(&func.statements),
    );
}