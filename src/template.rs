use askama::Template;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "true_false_question.txt")]
pub struct TrueFalseQuestionTemplate<'a> {
    question: &'a str,
    answer_1: &'a str,
    answer_2: &'a str,
}

#[allow(dead_code)]
impl<'a> TrueFalseQuestionTemplate<'a> {
    pub fn new(question: &'a str, answer_1: &'a str, answer_2: &'a str) -> Self {
        Self {
            question,
            answer_1,
            answer_2,
        }
    }
}

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "selection_question.txt")]
pub struct SelectionQuestionTemplate<'a> {
    question: &'a str,
    answers: Vec<String>,
}

#[allow(dead_code)]
impl<'a> SelectionQuestionTemplate<'a> {
    pub fn new(question: &'a str, answers: Vec<&'a str>) -> Self {
        let answers: Vec<String> = answers
            .iter()
            .zip("abcdefghijklmnop".chars())
            .map(|(v, i)| format!("{i}. {v}"))
            .collect();

        Self { question, answers }
    }
}

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "user_input_question.txt")]
pub struct UserInputQuestionTemplate<'a> {
    question: &'a str,
}

#[allow(dead_code)]
impl<'a> UserInputQuestionTemplate<'a> {
    pub fn new(question: &'a str) -> Self {
        Self { question }
    }
}

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "order_question.txt")]
pub struct OrderQuestionTemplate<'a> {
    question: &'a str,
    answers: Vec<String>,
}

#[allow(dead_code)]
impl<'a> OrderQuestionTemplate<'a> {
    pub fn new(question: &'a str, answers: Vec<&'a str>) -> Self {
        let answers: Vec<String> = answers
            .iter()
            .zip("abcdefghijklmnop".chars())
            .map(|(v, i)| format!("{i}. {v}"))
            .collect();

        Self { question, answers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_question() {
        let expected = r#"Question: question?

a. answer_1
b. answer_2
c. answer_3
d. answer_4"#;

        let question = OrderQuestionTemplate::new(
            "question?",
            vec!["answer_1", "answer_2", "answer_3", "answer_4"],
        );
        let text = question.render().unwrap();
        for (line, wanted_line) in text.lines().zip(expected.lines()) {
            assert!(line.contains(wanted_line));
        }
    }

    #[test]
    fn test_user_input_question() {
        let expected = r#"Question: question?"#;

        let question = UserInputQuestionTemplate::new("question?");
        let text = question.render().unwrap();
        for (line, wanted_line) in text.lines().zip(expected.lines()) {
            assert!(line.contains(wanted_line));
        }
    }

    #[test]
    fn test_selection_question() {
        let expected = r#"Question: question?

a. answer_1
b. answer_2
c. answer_3
d. answer_4"#;

        let question = SelectionQuestionTemplate::new(
            "question?",
            vec!["answer_1", "answer_2", "answer_3", "answer_4"],
        );
        let text = question.render().unwrap();
        for (line, wanted_line) in text.lines().zip(expected.lines()) {
            assert!(line.contains(wanted_line));
        }
    }

    #[test]
    fn test_true_false_question() {
        let expected = r#"Question: question?

answer_1 | answer_2"#;

        let question = TrueFalseQuestionTemplate::new("question?", "answer_1", "answer_2");
        let text = question.render().unwrap();
        for (line, wanted_line) in text.lines().zip(expected.lines()) {
            assert!(line.contains(wanted_line));
        }
    }
}
