mod quizlit;
mod requests;
mod schema;
mod template;

use std::io::Write;

use askama::Template;

use crate::quizlit::{AnswerTrait, QuestionTrait, QuestionType};

/*
Thoughts on the display:

I think I can build the display content from wrapped double ended iterators.
The inner iterator is used for question and answer text and display count (they know the total and index).
The next layer of iterator handles the user input interface layer (commands and accepting answers).

Anyway, this will make everything disconnected, testable in isolation and
the main function will only have to deal with a single iterator.
*/

// TODO: Make this into a double ended iterator
struct QuestionListDisplay {
    questions: Vec<quizlit::Question>,
    index: usize,
}

impl QuestionListDisplay {
    fn new(questions: Vec<quizlit::Question>) -> Self {
        Self {
            questions,
            index: 0,
        }
    }

    fn previous(&mut self) -> Option<String> {
        match self.index > 0 {
            true => {
                self.index -= 1;
                Some(self.current_question())
            }
            false => None,
        }
    }

    fn next(&mut self) -> Option<String> {
        match self.index < self.questions.len() - 1 {
            true => {
                self.index += 1;
                Some(self.current_question())
            }
            false => None,
        }
    }
    fn current_question(&self) -> String {
        let question = &self.questions[self.index];
        let question_type = question.get_type();
        let question_str = question.get_question().unwrap();

        let question_text = match question_type {
            QuestionType::TrueFalse => {
                let answers = question.get_answers().unwrap().answers().unwrap();
                let question_template = template::TrueFalseQuestionTemplate::new(
                    &question_str,
                    &answers[0],
                    &answers[1],
                );
                question_template.render().unwrap()
            }
            QuestionType::Selection => {
                let answers = question.get_answers().unwrap().answers().unwrap();
                let mut answers_str = Vec::new();
                for x in &answers {
                    answers_str.push(x.as_str());
                }

                let question_template =
                    template::SelectionQuestionTemplate::new(&question_str, answers_str);
                question_template.render().unwrap()
            }
            QuestionType::UserInput => {
                let question_template = template::UserInputQuestionTemplate::new(&question_str);
                question_template.render().unwrap()
            }
            QuestionType::Order => {
                let answers = question.get_answers().unwrap().answers().unwrap();
                let mut answers_str = Vec::new();
                for x in &answers {
                    answers_str.push(x.as_str());
                }

                let question_template =
                    template::SelectionQuestionTemplate::new(&question_str, answers_str);
                question_template.render().unwrap()
            }
        };
        question_text
    }
}

#[derive(PartialEq)]
enum Command {
    Next,
    Previous,
    Quit,
    Unknown,
}

fn parse_command(input: &str) -> Command {
    match input {
        "n" | "next" => Command::Next,
        "p" | "previous" => Command::Previous,
        "q" | "Quit" => Command::Quit,
        _ => Command::Unknown,
    }
}

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

    let mut question_list_display = QuestionListDisplay::new(list);

    let mut current_command = Command::Unknown;
    while current_command != Command::Quit {
        clearscreen::clear().expect("failed to clear screen");

        let question_text = question_list_display.current_question();
        // Print stuff to screen
        print!("{question_text}");
        //print!(">>> ");
        std::io::stdout().flush().unwrap();

        // Get user input
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;

        current_command = parse_command(user_input.trim());

        match current_command {
            Command::Next => match question_list_display.next() {
                Some(_) => {}
                None => {
                    println!("No next question")
                }
            },
            Command::Previous => match question_list_display.previous() {
                Some(_) => {}
                None => {
                    println!("No previous question")
                }
            },
            Command::Quit => {}
            Command::Unknown => {}
        }
    }

    Ok(())
}
