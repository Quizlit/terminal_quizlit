#[derive(Debug, PartialEq)]
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

    #[test]
    fn test_question_new_happy_path() {
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

        let json_value = serde_json::from_str(raw_json).unwrap();

        let question = Question::new(json_value);

        assert!(question.is_ok());
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
