#[derive(Debug, PartialEq, Clone)]
pub enum QuestionType {
    TrueFalse,
    Selection,
    UserInput,
    Order,
}

impl QuestionType {
    pub fn from_str(data: &str) -> Result<Self, String> {
        match data {
            "true_false" => Ok(Self::TrueFalse),
            "selection" => Ok(Self::Selection),
            "user_input" => Ok(Self::UserInput),
            "order" => Ok(Self::Order),
            x => Err(format!("\"{x}\" is not  a supported Question type")),
        }
    }
}

#[allow(dead_code)]
pub trait QuestionTrait {
    fn get_type(&self) -> &QuestionType;
    fn get_question(&self) -> Result<String, String>;
    fn get_answers(&self) -> Result<Answers, String>;
}

impl QuestionTrait for Question {
    fn get_type(&self) -> &QuestionType {
        &self.question_type
    }
    fn get_question(&self) -> Result<String, String> {
        if self.data.get("question").is_none() {
            return Err("Could not find the question field".to_string());
        }

        Ok(self
            .data
            .get("question")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string())
    }

    fn get_answers(&self) -> Result<Answers, String> {
        if self.data.get("answers").is_none() {
            return Err("Could not find the answers field".to_string());
        }

        let data = self.data.get("answers").unwrap().clone();
        let question_type = self.get_type().clone();

        Ok(Answers {
            data,
            question_type,
        })
    }
}

#[allow(dead_code)]
pub trait AnswerTrait {
    //TODO: add randomized_answers method and implement it in the trait
    fn answers(&self) -> Option<Vec<String>>;
    fn correct_answers(&self) -> Option<Vec<(String, Option<String>)>>;
    fn validate_answer(&self, input: String) -> bool;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Answers {
    data: serde_json::Value,
    question_type: QuestionType,
}

impl AnswerTrait for Answers {
    #[allow(clippy::vec_init_then_push)]
    fn answers(&self) -> Option<Vec<String>> {
        match self.question_type.clone() {
            QuestionType::TrueFalse => {
                let mut results = Vec::new();
                results.push(
                    self.data
                        .get("correct")
                        .expect("correct")
                        .get("answer")
                        .expect("answer")
                        .as_str()
                        .expect("string")
                        .to_string(),
                );
                results.push(
                    self.data
                        .get("incorrect")
                        .expect("correct")
                        .get("answer")
                        .expect("answer")
                        .as_str()
                        .expect("string")
                        .to_string(),
                );

                Some(results)
            }
            QuestionType::Order => {
                let mut results = Vec::new();
                let answers = self
                    .data
                    .get("correct")
                    .expect("correct answers")
                    .as_array()
                    .expect("valid json")
                    .clone();

                for answer in answers {
                    results.push(
                        answer
                            .get("answer")
                            .expect("answer string")
                            .as_str()
                            .expect("a string")
                            .to_string(),
                    );
                }

                Some(results)
            }
            QuestionType::UserInput => None,
            QuestionType::Selection => {
                let mut results = Vec::new();
                let correct_answers = self
                    .data
                    .get("correct")
                    .expect("correct answers")
                    .as_array()
                    .expect("valid json")
                    .clone();

                let incorrect_answers = self
                    .data
                    .get("incorrect")
                    .expect("incorrect answers")
                    .as_array()
                    .expect("valid json")
                    .clone();

                for answer in correct_answers.iter().chain(incorrect_answers.iter()) {
                    results.push(
                        answer
                            .get("answer")
                            .expect("answer string")
                            .as_str()
                            .expect("a string")
                            .to_string(),
                    );
                }

                Some(results)
            }
        }
    }
    #[allow(clippy::vec_init_then_push)]
    fn correct_answers(&self) -> Option<Vec<(String, Option<String>)>> {
        match self.question_type.clone() {
            QuestionType::TrueFalse => {
                let mut results = vec![];
                results.push((
                    self.data
                        .get("correct")
                        .expect("correct")
                        .get("answer")
                        .expect("answer")
                        .as_str()
                        .expect("string")
                        .to_string(),
                    self.data
                        .get("correct")
                        .expect("correct")
                        .get("explanation")
                        .map(|x| x.as_str().unwrap().to_string()),
                ));

                Some(results)
            }
            QuestionType::Order => {
                let mut results = vec![];
                let answers = self
                    .data
                    .get("correct")
                    .expect("correct answers")
                    .as_array()
                    .expect("valid json")
                    .clone();

                for answer in answers {
                    results.push((
                        answer
                            .get("answer")
                            .expect("answer string")
                            .as_str()
                            .expect("a string")
                            .to_string(),
                        answer
                            .get("explanation")
                            .map(|x| x.as_str().unwrap().to_string()),
                    ));
                }

                Some(results)
            }
            QuestionType::UserInput => None,
            QuestionType::Selection => {
                let mut results = Vec::new();
                let correct_answers = self
                    .data
                    .get("correct")
                    .expect("correct answers")
                    .as_array()
                    .expect("valid json")
                    .clone();

                for answer in correct_answers.iter() {
                    results.push((
                        answer
                            .get("answer")
                            .expect("answer string")
                            .as_str()
                            .expect("a string")
                            .to_string(),
                        answer
                            .get("explanation")
                            .map(|x| x.as_str().unwrap().to_string()),
                    ));
                }

                Some(results)
            }
        }
    }
    fn validate_answer(&self, _input: String) -> bool {
        todo!();
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Question {
    data: serde_json::Value,
    question_type: QuestionType,
}

impl Question {
    pub fn new(data: serde_json::Value) -> Result<Self, String> {
        if data.get("kind").is_none() {
            return Err("Json data does not have a \"kind\" field".to_string());
        }

        let kind = data
            .get("kind")
            .expect("kind field should exist")
            .as_str()
            .unwrap();

        let question_type = QuestionType::from_str(kind).expect("valid question type");

        Ok(Self {
            data,
            question_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_user_input_question_json() -> serde_json::Value {
        let raw_json = r#"{
      "answers": {
        "accepted": [
          "Lucy"
        ],
        "caseSensitive": false
      },
      "kind": "user_input",
      "question": "What is the name Luffy uses to enter the Colosseum in Dressrosa?"
    }"#;

        serde_json::from_str(raw_json).unwrap()
    }

    fn get_order_question_json() -> serde_json::Value {
        let raw_json = r#"{
      "answers": {
        "correct": [
          {
            "answer": "Amazon Lily Arc"
          },
          {
            "answer": "Impel Down Arc"
          },
          {
            "answer": "Marineford Arc"
          },
          {
            "answer": "Post-War Arc"
          }
        ]
      },
      "kind": "order",
      "question": "What is the correct order of these One Piece Arcs?"
    }"#;

        serde_json::from_str(raw_json).unwrap()
    }

    fn get_selection_question_json() -> serde_json::Value {
        let raw_json = r#"{
      "answers": {
        "correct": [
          {
            "answer": "Gum Gum Fruit"
          }
        ],
        "incorrect": [
          {
            "answer": "Stretch Stretch Fruit"
          },
          {
            "answer": "Ruber Ruber Fruit"
          },
          {
            "answer": "Hungry Hungry Fruit"
          }
        ]
      },
      "kind": "selection",
      "question": "What devil fruit did Luffy eat?"
    }"#;

        serde_json::from_str(raw_json).unwrap()
    }

    fn get_true_false_question_json() -> serde_json::Value {
        let raw_json = r#"
   {
      "answers": {
        "correct": {
          "answer": "True",
          "explanation": "Because 7 8 (ate) 9"
        },
        "incorrect": {
          "answer": "False",
          "explanation": "Because 7 8 (ate) 9"
        }
      },
      "kind": "true_false",
      "question": "Is the number 9 is afraid of the number 7?"
    }
"#;

        serde_json::from_str(raw_json).unwrap()
    }

    #[test]
    fn test_question_get_correct_answers() {
        let cases = vec![
            (
                "true_false",
                Question::new(get_true_false_question_json()).unwrap(),
                Some(vec![(
                    "True".to_string(),
                    Some("Because 7 8 (ate) 9".to_string()),
                )]),
            ),
            (
                "user_input",
                Question::new(get_user_input_question_json()).unwrap(),
                None,
            ),
            (
                "selection",
                Question::new(get_selection_question_json()).unwrap(),
                Some(vec![("Gum Gum Fruit".to_string(), None)]),
            ),
            (
                "order",
                Question::new(get_order_question_json()).unwrap(),
                Some(vec![
                    ("Amazon Lily Arc".to_string(), None),
                    ("Impel Down Arc".to_string(), None),
                    ("Marineford Arc".to_string(), None),
                    ("Post-War Arc".to_string(), None),
                ]),
            ),
        ];

        for (case_name, question, expected) in cases {
            assert_eq!(
                question.get_answers().unwrap().correct_answers(),
                expected,
                "case: {case_name} failed"
            );
        }
    }

    #[test]
    fn test_question_get_answers() {
        let cases = vec![
            (
                "true_false",
                Question::new(get_true_false_question_json()).unwrap(),
                Some(vec!["True".to_string(), "False".to_string()]),
            ),
            (
                "user_input",
                Question::new(get_user_input_question_json()).unwrap(),
                None,
            ),
            (
                "selection",
                Question::new(get_selection_question_json()).unwrap(),
                Some(vec![
                    "Gum Gum Fruit".to_string(),
                    "Stretch Stretch Fruit".to_string(),
                    "Ruber Ruber Fruit".to_string(),
                    "Hungry Hungry Fruit".to_string(),
                ]),
            ),
            (
                "order",
                Question::new(get_order_question_json()).unwrap(),
                Some(vec![
                    "Amazon Lily Arc".to_string(),
                    "Impel Down Arc".to_string(),
                    "Marineford Arc".to_string(),
                    "Post-War Arc".to_string(),
                ]),
            ),
        ];

        for (case_name, question, expected) in cases {
            assert_eq!(
                question.get_answers().unwrap().answers(),
                expected,
                "case: {case_name} failed"
            );
        }
    }

    #[test]
    fn test_question_happy_path() {
        let question = Question::new(get_true_false_question_json()).unwrap();

        assert_eq!(question.get_type(), &QuestionType::TrueFalse);
        assert_eq!(
            question.get_question().unwrap(),
            "Is the number 9 is afraid of the number 7?".to_string()
        );
        assert!(question.get_answers().is_ok());

        let answers_list = question.get_answers().unwrap().answers().unwrap();
        assert_eq!(answers_list, vec!["True", "False"]);
    }

    #[test]
    fn test_question_type_from_str() {
        let cases = vec![
            ("true_false", Ok(QuestionType::TrueFalse)),
            ("selection", Ok(QuestionType::Selection)),
            ("user_input", Ok(QuestionType::UserInput)),
            ("order", Ok(QuestionType::Order)),
            (
                "not supported",
                Err("\"not supported\" is not  a supported Question type".to_string()),
            ),
        ];

        for (index, case) in cases.into_iter().enumerate() {
            let (data, expected) = case;

            assert_eq!(
                QuestionType::from_str(data),
                expected,
                "Case {}: Failed with data == '{}'",
                index + 1,
                data,
            );
        }
    }
}
