use std::collections::BTreeMap;

use ndc_sdk::models::{
    ArgumentInfo, FunctionInfo, ObjectField, ObjectType, ScalarType, SchemaResponse, Type,
};
use serde::Deserialize;

pub fn make_schema_response() -> SchemaResponse {
    SchemaResponse {
        scalar_types: BTreeMap::from([
            (String::from("String"), STRING_SCALAR_TYPE),
            (String::from("Int"), INT_SCALAR_TYPE),
            (String::from("Bool"), BOOL_SCALAR_TYPE),
        ]),
        object_types: BTreeMap::from([
            (
                String::from("list_template_params"),
                list_template_params(),
            ),
            (String::from("list_template_item"), list_template_item()),
            (
                String::from("list_template_version"),
                list_template_version(),
            ),
        ]),
        collections: vec![],
        functions: vec![list_function_templates()],
        procedures: vec![],
    }
}

const STRING_SCALAR_TYPE: ScalarType = ScalarType {
    aggregate_functions: BTreeMap::new(),
    comparison_operators: BTreeMap::new(),
    update_operators: BTreeMap::new(),
};

const INT_SCALAR_TYPE: ScalarType = ScalarType {
    aggregate_functions: BTreeMap::new(),
    comparison_operators: BTreeMap::new(),
    update_operators: BTreeMap::new(),
};

const BOOL_SCALAR_TYPE: ScalarType = ScalarType {
    aggregate_functions: BTreeMap::new(),
    comparison_operators: BTreeMap::new(),
    update_operators: BTreeMap::new(),
};

#[derive(Clone, Debug, Deserialize)]
pub struct ListTemplateRequest {
    pub generations: Option<String>,
    pub page_size: u32,
    pub page_token: Option<String>,
}

fn list_template_params() -> ObjectType {
    ObjectType {
        description: Some(String::from("The request parameters for listing transactional templates")),
        fields: BTreeMap::from([
            (String::from("generations"), ObjectField {
                r#type: Type::Nullable { underlying_type: Box::new(Type::Named {name: String::from("String")}) },
                arguments: BTreeMap::new(),
                description: Some(String::from("Comma-delimited list specifying which generations of templates to return. Options are legacy, dynamic or legacy,dynamic"))
            }),
            (String::from("page_size"), ObjectField {
                r#type: Type::Named {name: String::from("Int")},
                arguments: BTreeMap::new(),
                description: Some(String::from("The number of templates to be returned in each page of results"))
            }),
            (String::from("page_token"), ObjectField {
                r#type: Type::Nullable { underlying_type: Box::new(Type::Named {name: String::from("String")}) },
                arguments: BTreeMap::new(),
                description: Some(String::from("A token corresponding to a specific page of results, as provided by metadata"))
            }),
        ]),
    }
}

fn list_template_item() -> ObjectType {
    ObjectType {
        description: Some(String::from(
            "The response properties for listing transactional templates",
        )),
        fields: BTreeMap::from([
            (
                String::from("id"),
                ObjectField {
                    r#type: Type::Named {
                        name: String::from("String"),
                    },
                    arguments: BTreeMap::new(),
                    description: Some(String::from("The ID of the transactional template.")),
                },
            ),
            (
                String::from("name"),
                ObjectField {
                    r#type: Type::Named {
                        name: String::from("String"),
                    },
                    arguments: BTreeMap::new(),
                    description: Some(String::from("The name for the transactional template.")),
                },
            ),
            (
                String::from("generation"),
                ObjectField {
                    r#type: Type::Named {
                        name: String::from("String"),
                    },
                    arguments: BTreeMap::new(),
                    description: Some(String::from("Defines the generation of the template.")),
                },
            ),
            (
                String::from("updated_at"),
                ObjectField {
                    r#type: Type::Named {
                        name: String::from("String"),
                    },
                    arguments: BTreeMap::new(),
                    description: Some(String::from(
                        "The date and time that this transactional template version was updated",
                    )),
                },
            ),
            (
                String::from("versions"),
                ObjectField {
                    r#type: Type::Array {
                        element_type: Box::new(Type::Named {
                            name: String::from("list_template_version"),
                        }),
                    },
                    arguments: BTreeMap::new(),
                    description: Some(String::from(
                        "The date and time that this transactional template version was updated",
                    )),
                },
            ),
        ]),
    }
}

fn list_template_version() -> ObjectType {
    ObjectType {
        description: Some(String::from("The response properties for listing transactional templates")),
        fields: BTreeMap::from([
            (String::from("id"), ObjectField {
                r#type: Type::Named {name: String::from("String") },
                arguments: BTreeMap::new(),
                description: Some(String::from("ID of the transactional template version."))
            }),
            (String::from("template_id"), ObjectField {
                r#type: Type::Named {name: String::from("String")},
                arguments: BTreeMap::new(),
                description: Some(String::from("ID of the transactional template."))
            }),
            (String::from("active"), ObjectField {
                r#type: Type::Named {name: String::from("Int")},
                arguments: BTreeMap::new(),
                description: Some(String::from("Set the version as the active version associated with the template. Only one version of a template can be active. The first version created for a template will automatically be set to Active."))
            }),
            (String::from("name"), ObjectField {
                r#type: Type::Named {name: String::from("String")},
                arguments: BTreeMap::new(),
                description: Some(String::from("Name of the transactional template version."))
            }),
            (String::from("subject"), ObjectField {
                r#type: Type::Nullable { underlying_type: Box::new(Type::Named {name: String::from("String")}) },
                arguments: BTreeMap::new(),
                description: Some(String::from("Subject of the new transactional template version."))
            }),
            (String::from("updated_at"), ObjectField {
                r#type: Type::Named {name: String::from("String")},
                arguments: BTreeMap::new(),
                description: Some(String::from("The date and time that this transactional template version was updated."))
            }),
            (String::from("generate_plain_content"), ObjectField {
                r#type: Type::Named {name: String::from("Bool")},
                arguments: BTreeMap::new(),
                description: Some(String::from("If true, plain_content is always generated from html_content. If false, plain_content is not altered."))
            }),
            (String::from("editor"), ObjectField {
                r#type: Type::Named {name: String::from("String")},
                arguments: BTreeMap::new(),
                description: Some(String::from("The editor used in the UI. Allowed Values: code, design"))
            }),
            (String::from("thumbnail_url"), ObjectField {
                r#type: Type::Named {name: String::from("String")},
                arguments: BTreeMap::new(),
                description: Some(String::from("A Thumbnail preview of the template's html content."))
            }),
        ]),
    }
}

pub const LIST_TEMPLATES_FUNCTION_NAME: &str = "list_templates";

fn list_function_templates() -> FunctionInfo {
    FunctionInfo {
        name: String::from(LIST_TEMPLATES_FUNCTION_NAME),
        description: Some(String::from(
            "allows you to retrieve all transactional templates",
        )),
        arguments: BTreeMap::from([(
            String::from("params"),
            ArgumentInfo {
                description: Some(String::from("Request parameters")),
                argument_type: Type::Named {
                    name: String::from("list_template_params"),
                },
            },
        )]),
        result_type: Type::Array {
            element_type: Box::new(Type::Named {
                name: String::from("list_template_item"),
            }),
        },
    }
}
