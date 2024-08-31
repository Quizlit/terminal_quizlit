mod quizlit;
mod requests;
mod schema;
mod template;

fn get_questions(json: &serde_json::Value) -> Result<Vec<quizlit::Question>, String> {
    let mut result = Vec::new();
    let json_questions = json["questions"].as_array().expect("valid json");
    for data in json_questions {
        match quizlit::Question::new(data.clone()) {
            Ok(question) => result.push(question),
            Err(_) => return Err(format!("Unable to create question from {:?}", data)),
        }
    }

    Ok(result)
}

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

    let list = get_questions(&instance).unwrap();
    println!("{:#?}", list);

    Ok(())
}
