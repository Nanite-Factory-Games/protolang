use std::fs::File;


/// Takes in a list of files with module definitions and merges any files with the same module together
pub fn merge_modules(files: Vec<File>) -> Vec<File> {
    // TODO merge source for any files that have the same module as others
    return files;
}

pub fn snake_case_to_camel_case(name: &str) -> String {
    let mut result = String::new();
    let mut capitalize = false;
    for c in name.chars() {
        if c == '_' {
            capitalize = true;
        } else {
            if capitalize {
                result.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                result.push(c);
            }
        }
    }
    return result; 
}

pub fn snake_case_to_title_case(name: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;
    for c in name.chars() {
        if c == '_' {
            capitalize = true;
        } else {
            if capitalize {
                result.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                result.push(c);
            }
        }
    }
    return result; 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_snake_case_to_camel_case() {
        assert_eq!(snake_case_to_camel_case("hello_world"), "helloWorld");
        assert_eq!(snake_case_to_camel_case("hello_world_2"), "helloWorld2");
        assert_eq!(snake_case_to_camel_case("hello_world_2_hello_world_3"), "helloWorld2HelloWorld3");
        assert_eq!(snake_case_to_camel_case("hello_world_hello_world"), "helloWorldHelloWorld");
        assert_eq!(snake_case_to_camel_case("hello_world_hello_world_2"), "helloWorldHelloWorld2");
        assert_eq!(snake_case_to_camel_case("hello_world_hello_world_2_hello_world_3"), "helloWorldHelloWorld2HelloWorld3");
    }

    #[test]
    fn test_snake_case_to_title_case() {
        assert_eq!(snake_case_to_title_case("hello_world"), "HelloWorld");
        assert_eq!(snake_case_to_title_case("hello_world_2"), "HelloWorld2");
        assert_eq!(snake_case_to_title_case("hello_world_2_hello_world_3"), "HelloWorld2HelloWorld3");
        assert_eq!(snake_case_to_title_case("hello_world_hello_world"), "HelloWorldHelloWorld");
        assert_eq!(snake_case_to_title_case("hello_world_hello_world_2"), "HelloWorldHelloWorld2");
        assert_eq!(snake_case_to_title_case("hello_world_hello_world_2_hello_world_3"), "HelloWorldHelloWorld2HelloWorld3");
    }
}