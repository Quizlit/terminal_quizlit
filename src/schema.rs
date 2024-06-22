use jsonschema::{Draft, JSONSchema, ValidationError};

pub fn validate_json<'a>(
    compiled_schema: &'a JSONSchema,
    json_data: &'a serde_json::Value,
) -> Result<(), jsonschema::ErrorIterator<'a>> {
    let _ = compiled_schema.validate(json_data)?;
    Ok(())
}

pub fn compile_json_schema(schema: &serde_json::Value) -> Result<JSONSchema, ValidationError> {
    JSONSchema::options()
        .with_draft(Draft::Draft202012)
        .compile(schema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_json_errors() {
        let schema = json!({"maxLength": 5});

        let cases = vec![(
            "larger than max length",
            json!("long foo"),
            vec!["\"long foo\" is longer than 5 characters"],
        )];

        let compiled = compile_json_schema(&schema).expect("A valid schema");

        for (i, case) in cases.into_iter().enumerate() {
            let count = i + 1;
            let (name, instance, expected_error) = case;

            let result = validate_json(&compiled, &instance);

            assert!(result.is_err());
            if let Err(errors) = result {
                for (index, error) in errors.enumerate() {
                    assert_eq!(
                        expected_error[index],
                        error.to_string(),
                        "case {count}: {name} failed"
                    );
                }
            }
        }
    }

    #[test]
    fn test_validate_json_happy_path() {
        let schema = json!({"maxLength": 5});
        let cases = vec![("foo", json!("foo")), ("empty string", json!(""))];

        let compiled = compile_json_schema(&schema).expect("A valid schema");

        for (i, case) in cases.into_iter().enumerate() {
            let count = i + 1;
            let (name, instance) = case;
            let result = validate_json(&compiled, &instance);

            assert!(result.is_ok(), "case {count}: {name} failed");
        }
    }
}
