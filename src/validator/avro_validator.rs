extern crate avro_rs;

use super::super::models;
use avro_rs::Schema;

pub fn is_valid(definition: &String) -> bool {
    let schema = Schema::parse_str(definition);
    if schema.is_ok() {
        return true;
    } else {
        return false;
    }
}

pub fn get_matching_schema_from_def<'a>(
    schemas: Vec<&'a models::Schema>,
    schema_string: &String,
) -> Option<&'a models::Schema> {
    get_matching_schema(schemas, &Schema::parse_str(schema_string).unwrap())
}

pub fn get_matching_schema<'a>(
    schemas: Vec<&'a models::Schema>,
    schema: &Schema,
) -> Option<&'a models::Schema> {
    let mut ret: Option<&'a models::Schema> = None;

    for s in schemas {
        let cur_schema = match Schema::parse_str(&s.definition) {
            Ok(cur_schema) => cur_schema,
            Err(e) => panic!(e),
        };

        if cur_schema == *schema {
            ret = Some(s);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SCHEMA_1: &str = r#"
        {
            "type": "record",
            "name": "test1",
            "fields": [
                {"name": "a", "type": "long", "default": 42},
                {"name": "b", "type": "string"}
            ]
        }
    "#;

    const TEST_SCHEMA_2: &str = r#"
        {
            "type": "record",
            "name": "test2",
            "fields": [
                {"name": "a", "type": "long", "default": 42},
                {"name": "b", "type": "long"},
                {"name": "c", "type": "long"}
            ]
        }
    "#;

    const TEST_SCHEMA_3: &str = r#"
        {
            "type": "record",
            "name": "test3",
            "fields": [
                {"name": "aa", "type": "string", "default": "foobar"},
                {"name": "bbb", "type": "string"},
                {"name": "ccc", "type": "string"}
            ]
        }
    "#;

    const TEST_BAD_SCHEMA_1: &str = r#"
        {
            "type": "record",
            "name": "testfdfs3",
            "fields": [
                {"name": "aa", "type": "string", "default": "foobar"},
                {"name": "bbb", "type": "strsing"},
                {"name": "ccc", "type": "string"}
            ]
        }
    "#;

    #[test]
    fn test_is_valid() {
        let valid = is_valid(&TEST_SCHEMA_1.to_string());
        assert_eq!(valid, true);
    }

    #[test]
    fn test_is_not_valid() {
        let valid = is_valid(&TEST_BAD_SCHEMA_1.to_string());
        assert_eq!(valid, false);
    }

    #[test]
    fn test_match() {
        let to_find = Schema::parse_str(TEST_SCHEMA_2).unwrap();

        let schema_model_1 = models::Schema {
            id: 1,
            version: 1,
            format: "avro".to_string(),
            subject: "test1".to_string(),
            definition: TEST_SCHEMA_1.to_string(),
        };
        let schema_model_2 = models::Schema {
            id: 2,
            version: 1,
            format: "avro".to_string(),
            subject: "test2".to_string(),
            definition: TEST_SCHEMA_2.to_string(),
        };
        let schema_model_3 = models::Schema {
            id: 3,
            version: 1,
            format: "avro".to_string(),
            subject: "test3".to_string(),
            definition: TEST_SCHEMA_3.to_string(),
        };

        let mut schemas = Vec::new();
        schemas.push(&schema_model_1);
        schemas.push(&schema_model_2);
        schemas.push(&schema_model_3);

        let schema = get_matching_schema(schemas, &to_find).unwrap();
        assert_eq!(schema, &schema_model_2);
    }
}
