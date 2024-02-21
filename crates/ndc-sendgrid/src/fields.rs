use thiserror::Error;
use indexmap::IndexMap;
use ndc_sdk::{connector, models};

#[derive(Debug, Error)]
pub enum FieldsError {
    /// The request was invalid or did not match the
    /// requirements of the specification. This indicates
    /// an error with the client.
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("unsupported operation: {0}")]
    UnsupportedOperation(String),
}

impl From<FieldsError> for connector::QueryError {
    fn from(value: FieldsError) -> Self {
        match value {
            FieldsError::InvalidRequest(err) => connector::QueryError::InvalidRequest(err),
            FieldsError::UnsupportedOperation(err) => connector::QueryError::UnsupportedOperation(err)
        }
    }
}

impl From<FieldsError> for connector::MutationError {
    fn from(value: FieldsError) -> Self {
        match value {
            FieldsError::InvalidRequest(err) => connector::MutationError::InvalidRequest(err),
            FieldsError::UnsupportedOperation(err) => connector::MutationError::UnsupportedOperation(err)
        }
    }
}

pub fn eval_row(
    fields: &IndexMap<String, models::Field>,
    item: &IndexMap<String, serde_json::Value>,
) -> Result<IndexMap<String, models::RowFieldValue>, FieldsError> {
    let mut row = IndexMap::new();
    for (field_name, field) in fields.iter() {
        row.insert(
            field_name.clone(),
            eval_field(field, item)?,
        );
    }
    Ok(row)
}

fn eval_field(
    field: &models::Field,
    item: &IndexMap<String, serde_json::Value>,
) -> Result<models::RowFieldValue, FieldsError> {
    match field {
        models::Field::Column { column, fields } => {
            let col_val = eval_column(item, column.as_str())?;
            match fields {
                None => Ok(models::RowFieldValue(col_val)),
                Some(nested_field) => eval_nested_field(
                    col_val,
                    nested_field,
                ),
            }
        }
        models::Field::Relationship { .. } => {
            Err(FieldsError::UnsupportedOperation("Relationship fields are not supported".into()))
        }
    }
}

fn eval_column(row: &IndexMap<String, serde_json::Value>, column_name: &str) -> Result<serde_json::Value, FieldsError> {
    row.get(column_name).cloned()
        .ok_or(FieldsError::InvalidRequest(format!("invalid column name: {}", column_name)))
}

pub fn eval_nested_field(
    value: serde_json::Value,
    nested_field: &models::NestedField,
) -> Result<models::RowFieldValue, FieldsError> {
    match nested_field {
        models::NestedField::Object(models::NestedObject { fields }) => {
            let full_row: IndexMap<String, serde_json::Value> = serde_json::from_value(value).map_err(|_| FieldsError::InvalidRequest("Object expected".into()))?;
            let row = eval_row(fields, &full_row)?;
            Ok(models::RowFieldValue(serde_json::to_value(row).map_err(|_| FieldsError::InvalidRequest("Cannot encode rowset".into()))?))
        }
        models::NestedField::Array(models::NestedArray { fields }) => {
            let array: Vec<serde_json::Value> = serde_json::from_value(value).map_err(|_| FieldsError::InvalidRequest("Array expected".into()))?;
            let result_array = array
                .into_iter()
                .map(|value| eval_nested_field(value, fields))
                .collect::<Result<Vec<_>, FieldsError>>()?;
            Ok(models::RowFieldValue(serde_json::to_value(result_array).map_err(|_| FieldsError::InvalidRequest("Cannot encode rowset".into()))?))
        }
    }
}
