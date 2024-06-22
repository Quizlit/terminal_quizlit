use serde_json::json;

mod schema;

fn main() {
    println!("Hello, world!");

    let schema = json!({"maxLength": 5});
    let instance = json!("foo failing");

    let compiled = schema::compile_json_schema(&schema).expect("A valid schema");
    let result = schema::validate_json(&compiled, &instance);
    if let Err(errors) = result {
        for error in errors {
            println!("Validatation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
    }
}
