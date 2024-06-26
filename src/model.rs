#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    String,
    Struct(())
}

#[derive(Debug)]
pub enum Value {
    StringValue(String),
    IntegerValue(i64),
    FloatValue(f64),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub ty: Type,
    pub value: Value
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration)
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub struct ImportStatement {
    pub module_name: String,
    pub import_names: Vec<String>
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub ty: Type
}

#[derive(Debug)]
pub struct File {
    pub module: String,
    pub imports: Vec<ImportStatement>,
    pub functions: Vec<Function>
}