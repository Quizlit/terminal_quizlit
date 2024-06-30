mod quizlit;
mod requests;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = requests::get_json(
        "https://raw.githubusercontent.com/Quizlit/schemas/main/src/schemas/v1/quizlit.json",
    )
    .await?;

    let instance = requests::get_json(
        "https://raw.githubusercontent.com/Quizlit/schemas/main/src/schemas/v1/examples/20_questions.json",
    )
    .await?;

    println!("Hello, World!");

    let compiled =
        schema::compile_json_schema(&schema).expect("Compiling the schema should not fail");
    let result = schema::validate_json(&compiled, &instance);

    if let Err(errors) = result {
        for error in errors {
            println!("Validatation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
    }

    let mut list = Vec::new();
    let my_json = instance["questions"].as_array().expect("valid json");
    for data in my_json {
        list.push(quizlit::Question::new(data.clone()));
    }
    println!("{:#?}", list);

    Ok(())
}
