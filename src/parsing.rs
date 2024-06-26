
use crate::model::*;

peg::parser!{
    pub grammar program() for str {
        // Types
        rule int() -> Type
            = "Int" { Type::Int } / expected!("Integer type")

        rule float() -> Type
            = "Float" { Type::Float } / expected!("Float type")

        rule string() -> Type
            = "String" { Type::String } / expected!("String type")
        
        rule ty() -> Type
            = t:(int() / float() / string()) { t } / expected!("type")
        
        // Values
        rule string_value() -> Value
            = value:string_literal() { Value::StringValue(value) } / expected!("String value")
        
        rule integer_value() -> Value
            = i:integer_literal() {Value::IntegerValue(i)} / expected!("Integer value")
        
        rule value() -> Value
            = (string_value() / integer_value()) / expected!("value")

        // Naming schemes
        rule underscore_delimited() -> String
            = n:$(['a'..='z' | '_']+) { n.to_string() } / expected!("underscore delimited name")

        rule module_name() -> String
            = quiet!{underscore_delimited()} / expected!("module name")

        rule function_name() -> String
            = quiet!{underscore_delimited()} / expected!("function name")

        rule import_name() -> String
            = quiet!{underscore_delimited()} / expected!("import name")

        rule variable_name() -> String
            = quiet!{underscore_delimited()} / expected!("variable name")

        // Helpers

        rule whitespace() = quiet!{ [' ' | '\t']+ } / expected!("whitespace")

        rule newline() = quiet!{ ['\n' | '\r']+ } / expected!("newline")

        rule any_whitespace() = quiet!{(whitespace() / newline())*} / expected!("any whitespace")
        
        rule optional_whitespace() = quiet!{ [' ' | '\t']* } / expected!("optional whitespace")

        rule optional_newline() = quiet!{ ['\n' | '\r']* } / expected!("optional newline")

        rule string_literal() -> String
            = "\"" s:$((!"\"" [_])* ) "\"" { s.to_string() } / expected!("string literal")
        
        rule integer_literal() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule parameter() -> Parameter
            = name:variable_name() ":" optional_whitespace() ty:ty() {Parameter{name, ty}} / expected!("parameter")
        
        rule ret() -> Type
            = "->" optional_whitespace() ty:ty() {ty} / expected!("function return")
        
        // Definitions
        rule module_def() -> String
            = "module" whitespace() n:module_name() newline() { n } / expected!("module definition")

        rule function_def() -> Function
            = "fn" whitespace() name:function_name() optional_whitespace() "(" optional_whitespace() parameters:parameter() ** ("," optional_whitespace()) optional_whitespace() ")" optional_whitespace() return_type:ret()? optional_whitespace() "{" any_whitespace() statements:statement()* any_whitespace() "}"{
                Function{
                    name,
                    parameters,
                    return_type,
                    statements
                }
            } / expected!("function definition")

        // Base context statements

        rule import_statement() -> ImportStatement
            = "from" whitespace() module_name:module_name() whitespace() "import" whitespace() import_names:import_name() ++ "," newline() {
                ImportStatement { module_name, import_names} 
            } / expected!("import statement")
        
        rule comment() -> String
            = "#" c:$((!['\n'][_])*) ['\n']? { c.to_string() }
        
        // Function statements

        rule variable_declaration() -> Statement
            = "let" name:variable_name() ":" ty:ty() "=" value:value() {
                Statement::VariableDeclaration(VariableDeclaration{
                    name,
                    ty,
                    value
                })
            }
        
        rule statement() -> Statement
            = variable_declaration()
        
        // Definition of a file in a program

        pub rule protolang_file() -> File
            = module:module_def() imports:import_statement()* functions:function_def()* {
                File {
                    module,
                    imports,
                    functions
                }
            }
    }
}