use std::collections::BTreeMap;

use ndc_sdk::models::{
    ArgumentInfo, FunctionInfo, ObjectField, ObjectType, ProcedureInfo, ScalarType, SchemaResponse,
    Type,
};
use serde::Deserialize;

use super::sendgrid_api;

pub fn make_schema_response() -> SchemaResponse {
    SchemaResponse {
        scalar_types: BTreeMap::from([
            (String::from("String"), STRING_SCALAR_TYPE),
            (String::from("Int"), INT_SCALAR_TYPE),
            (String::from("Bool"), BOOL_SCALAR_TYPE),
        ]),
        object_types: BTreeMap::from([
            (String::from("list_template_item"), list_template_item()),
            (
                String::from("list_template_version"),
                list_template_version(),
            ),
            (String::from("send_mail_request"), send_mail_request()),
            (String::from("mail_personalization"), mail_personalization()),
            (String::from("mail_address"), mail_address()),
            (String::from("header"), header()),
            (String::from("substitution"), substitution()),
            (String::from("template_variable"), template_variable()),
            (String::from("mail_content"), mail_content()),
            (String::from("mail_attachment"), mail_attachment()),
            (
                String::from("unsubscription_settings"),
                unsubscription_settings(),
            ),
            (String::from("send_mail_response"), send_mail_response()),
        ]),
        collections: vec![],
        functions: vec![list_function_templates()],
        procedures: vec![send_mail()],
    }
}

const STRING_SCALAR_TYPE: ScalarType = ScalarType {
    aggregate_functions: BTreeMap::new(),
    comparison_operators: BTreeMap::new(),
};

const INT_SCALAR_TYPE: ScalarType = ScalarType {
    aggregate_functions: BTreeMap::new(),
    comparison_operators: BTreeMap::new(),
};

const BOOL_SCALAR_TYPE: ScalarType = ScalarType {
    aggregate_functions: BTreeMap::new(),
    comparison_operators: BTreeMap::new(),
};

fn list_template_item() -> ObjectType {
    ObjectType {
        description: Some(String::from(
            "The response properties for listing transactional templates",
        )),
        fields: BTreeMap::from([
            (
                String::from("id"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from("The ID of the transactional template.")),
                },
            ),
            (
                String::from("name"),
                ObjectField {
                    r#type: Type::Named {
                        name: String::from("String"),
                    },
                    description: Some(String::from("The name for the transactional template.")),
                },
            ),
            (
                String::from("generation"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from("Defines the generation of the template.")),
                },
            ),
            (
                String::from("updated_at"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from(
                        "The date and time that this transactional template version was updated",
                    )),
                },
            ),
            (
                String::from("versions"),
                ObjectField {
                    r#type: array_of(named("list_template_version")),
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
                r#type: named("String"),
                description: Some(String::from("ID of the transactional template version."))
            }),
            (String::from("template_id"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("ID of the transactional template."))
            }),
            (String::from("active"), ObjectField {
                r#type: named("Int"),
                description: Some(String::from("Set the version as the active version associated with the template. Only one version of a template can be active. The first version created for a template will automatically be set to Active."))
            }),
            (String::from("name"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("Name of the transactional template version."))
            }),
            (String::from("subject"), ObjectField {
                r#type: nullable(named("String")),
                description: Some(String::from("Subject of the new transactional template version."))
            }),
            (String::from("updated_at"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("The date and time that this transactional template version was updated."))
            }),
            (String::from("generate_plain_content"), ObjectField {
                r#type: named("Bool"),
                description: Some(String::from("If true, plain_content is always generated from html_content. If false, plain_content is not altered."))
            }),
            (String::from("editor"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("The editor used in the UI. Allowed Values: code, design"))
            }),
            (String::from("thumbnail_url"), ObjectField {
                r#type: named("String"),
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
            String::from("generations"),
            ArgumentInfo {
                description: Some(String::from("Comma-delimited list specifying which generations of templates to return. Options are legacy, dynamic or legacy,dynamic")),
                argument_type: nullable(named("String"))
            }),
            (
            String::from("page_size"),
            ArgumentInfo {
                description: Some(String::from("The number of templates to be returned in each page of results")),
                argument_type: named("Int")
            }),
            (
            String::from("page_token"),
            ArgumentInfo {
                description: Some(String::from("A token corresponding to a specific page of results, as provided by metadata")),
                argument_type: nullable(named("String"))
            }),
        ]),
        result_type: array_of(named("list_template_item")),
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct SendMailRequest {
    pub personalizations: Vec<MailPersonalization>,
    pub from: sendgrid_api::MailAddress,
    pub reply_to_list: Vec<sendgrid_api::MailAddress>,
    pub subject: String,
    pub content: Vec<sendgrid_api::MailContent>,
    pub attachments: Option<Vec<sendgrid_api::MailAttachment>>,
    pub template_id: Option<String>,
    pub headers: Option<Vec<Header>>,
    pub send_at: Option<u32>,
    pub batch_id: Option<String>,
    pub asm: Option<sendgrid_api::UnsubscriptionSettings>,
}

fn send_mail_request() -> ObjectType {
    ObjectType {
        description: Some(String::from("The request properties for sending mail")),
        fields: BTreeMap::from([
            (String::from("personalizations"), ObjectField {
                r#type: array_of(named("mail_personalization")),
                description: Some(String::from("An array of messages and their metadata. Each object within personalizations can be thought of as an envelope - it defines who should receive an individual message and how that message should be handled."))
            }),
            (String::from("from"), ObjectField {
                r#type: named("mail_address"),
                description: Some(String::from("The 'From' email address used to deliver the message. This address should be a verified sender in your Twilio SendGrid account."))
            }),
            (String::from("reply_to_list"), ObjectField {
                r#type: array_of(named("mail_address")),
                description: Some(String::from("An array of recipients who will receive replies."))
            }),
            (String::from("subject"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("The global or 'message level' subject of your email. This may be overridden by subject lines set in personalizations."))
            }),
            (String::from("content"), ObjectField {
                r#type: array_of(named("mail_content")),
                description: Some(String::from("An array where you can specify the content of your email. You can include multiple MIME types of content, but you must specify at least one MIME type."))
            }),
            (String::from("attachments"), ObjectField {
                r#type: nullable(array_of(named("mail_attachment"))),
                description: Some(String::from("An array of objects where you can specify any attachments you want to include."))
            }),
            (String::from("template_id"), ObjectField {
                r#type: nullable(named("Bool")),
                description: Some(String::from("An email template ID. A template that contains a subject and content — either text or html — will override any subject and content values specified at the personalizations or message level."))
            }),
            (String::from("headers"), ObjectField {
                r#type: nullable(array_of(named("header"))),
                description: Some(String::from("The headers to put on the mail. You must ensure these are properly encoded if they contain unicode characters. These headers cannot be one of the reserved headers."))
            }),
            (String::from("send_at"), ObjectField {
                r#type: nullable(named("Int")),
                description: Some(String::from("A unix timestamp allowing you to specify when you want your email to be delivered. This may be overridden by the send_at parameter set at the personalizations level. Delivery cannot be scheduled more than 72 hours in advance."))
            }),
            (String::from("batch_id"), ObjectField {
                r#type: nullable(named("String")),
                description: Some(String::from("An ID representing a batch of emails to be sent at the same time. Including a batch_id in your request allows you include this email in that batch. It also enables you to cancel or pause the delivery of that batch."))
            }),
            (String::from("asm"), ObjectField {
                r#type: nullable(named("unsubscription_settings")),
                description: Some(String::from("An object allowing you to specify how to handle unsubscribes."))
            }),
        ]),
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct MailPersonalization {
    pub from: Option<sendgrid_api::MailAddress>,
    pub to: Vec<sendgrid_api::MailAddress>,
    pub cc: Option<Vec<sendgrid_api::MailAddress>>,
    pub bcc: Option<Vec<sendgrid_api::MailAddress>>,
    pub subject: Option<String>,
    pub headers: Option<Vec<Header>>,
    pub substitutions: Option<Vec<Substitution>>,
    pub dynamic_template_data: Option<Vec<TemplateVariable>>,
    pub send_at: Option<u32>,
}

fn mail_personalization() -> ObjectType {
    ObjectType {
        description: Some(String::from("A personalization can be thought of as an envelope - it defines who should receive an individual message and how that message should be handled.")),
        fields: BTreeMap::from([
            (String::from("from"), ObjectField {
                r#type: nullable(named("mail_address")),
                description: Some(String::from("The 'From' email address used to deliver the message. This address should be a verified sender in your Twilio SendGrid account."))
            }),
            (String::from("to"), ObjectField {
                r#type: nullable(array_of(named("mail_address"))),
                description: Some(String::from("An array of addresses that will be sent the email."))
            }),
            (String::from("cc"), ObjectField {
                r#type: array_of(named("mail_address")),
                description: Some(String::from("An array of addresses that will be cced the email."))
            }),
            (String::from("bcc"), ObjectField {
                r#type: nullable(array_of(named("mail_address"))),
                description: Some(String::from("An array of addresses that will be bcced the email."))
            }),
            (String::from("subject"), ObjectField {
                r#type: nullable(named("String")),
                description: Some(String::from("The subject of your email. See character length requirements according to RFC 2822."))
            }),
            (String::from("headers"), ObjectField {
                r#type: nullable(array_of(named("header"))),
                description: Some(String::from("A collection of headers allowing you to specify handling instructions for your email. You may not overwrite the following headers: x-sg-id, x-sg-eid, received, dkim-signature, Content-Type, Content-Transfer-Encoding, To, From, Subject, Reply-To, CC, BCC."))
            }),
            (String::from("substitutions"), ObjectField {
                r#type: nullable(array_of(named("substitution"))),
                description: Some(String::from("Substitutions allow you to insert data without using Dynamic Transactional Templates."))
            }),
            (String::from("dynamic_template_data"), ObjectField {
                r#type: nullable(array_of(named("template_variable"))),
                description: Some(String::from("A collection of dynamic template variable name/value to insert pairs."))
            }),
            (String::from("send_at"), ObjectField {
                r#type: nullable(named("Int")),
                description: Some(String::from("A unix timestamp allowing you to specify when your email should be delivered. Scheduling delivery more than 72 hours in advance is forbidden."))
            }),
        ]),
    }
}

fn mail_address() -> ObjectType {
    ObjectType {
        description: Some(String::from("An email address")),
        fields: BTreeMap::from([
            (
                String::from("email"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from("The recipient's email address")),
                },
            ),
            (
                String::from("name"),
                ObjectField {
                    r#type: nullable(named("String")),
                    description: Some(String::from("The recipient's name")),
                },
            ),
        ]),
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Header {
    pub name: String,
    pub value: String,
}

fn header() -> ObjectType {
    ObjectType {
        description: Some(String::from("A header in an email")),
        fields: BTreeMap::from([
            (
                String::from("name"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from("The name of the header")),
                },
            ),
            (
                String::from("value"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from("The value of the header")),
                },
            ),
        ]),
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Substitution {
    pub tag: String,
    pub value: String,
}

fn substitution() -> ObjectType {
    ObjectType {
        description: Some(String::from("A substitution value")),
        fields: BTreeMap::from([
            (
                String::from("tag"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from("The substitution tag")),
                },
            ),
            (
                String::from("value"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from("The value to substitute for the tag")),
                },
            ),
        ]),
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct TemplateVariable {
    pub variable: String,
    pub value: String,
}

fn template_variable() -> ObjectType {
    ObjectType {
        description: Some(String::from("A dynamic template variable value")),
        fields: BTreeMap::from([
            (
                String::from("variable"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from("The dynamic template variable name")),
                },
            ),
            (
                String::from("value"),
                ObjectField {
                    r#type: named("String"),
                    description: Some(String::from("The value to substitute for the variable")),
                },
            ),
        ]),
    }
}

fn mail_content() -> ObjectType {
    ObjectType {
        description: Some(String::from("Content of an email in a particular MIME format")),
        fields: BTreeMap::from([
            (String::from("type"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("The MIME type of the content you are including in your email"))
            }),
            (String::from("name"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("The actual content of the specified MIME type that you are including in your email."))
            }),
        ]),
    }
}

fn mail_attachment() -> ObjectType {
    ObjectType {
        description: Some(String::from("An attachment to an email")),
        fields: BTreeMap::from([
            (String::from("content"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("The Base64 encoded content of the attachment."))
            }),
            (String::from("type"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("The MIME type of the content you are attaching."))
            }),
            (String::from("filename"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("The attachment's filename."))
            }),
            (String::from("disposition"), ObjectField {
                r#type: nullable(named("String")),
                description: Some(String::from("The attachment's content-disposition, specifying how you would like the attachment to be displayed. For example, “inline” results in the attached file are displayed automatically within the message while “attachment” results in the attached file require some action to be taken before it is displayed, such as opening or downloading the file."))
            }),
            (String::from("content_id"), ObjectField {
                r#type: nullable(named("String")),
                description: Some(String::from("The attachment's content ID. This is used when the disposition is set to “inline” and the attachment is an image, allowing the file to be displayed within the body of your email."))
            }),
        ]),
    }
}

fn unsubscription_settings() -> ObjectType {
    ObjectType {
        description: Some(String::from("An object allowing you to specify how to handle unsubscribes.")),
        fields: BTreeMap::from([
            (String::from("group_id"), ObjectField {
                r#type: named("String"),
                description: Some(String::from("The unsubscribe group to associate with this email."))
            }),
            (String::from("groups_to_display"), ObjectField {
                r#type: nullable(array_of(named("group_id"))),
                description: Some(String::from("An array containing the unsubscribe groups that you would like to be displayed on the unsubscribe preferences page."))
            }),
        ]),
    }
}

fn send_mail_response() -> ObjectType {
    ObjectType {
        description: Some(String::from("The response from a mail send request.")),
        fields: BTreeMap::from([(
            String::from("batch_id"),
            ObjectField {
                r#type: nullable(named("String")),
                description: Some(String::from(
                    "The batch ID used with the send mail request.",
                )),
            },
        )]),
    }
}

pub const SEND_MAIL: &str = "send_mail";

fn send_mail() -> ProcedureInfo {
    ProcedureInfo {
        name: String::from(SEND_MAIL),
        description: Some(String::from("Allows you to send email")),
        arguments: BTreeMap::from([
            (String::from("from"),
                ArgumentInfo {
                    description: Some(String::from("An array of messages and their metadata. Each object within personalizations can be thought of as an envelope - it defines who should receive an individual message and how that message should be handled.")),
                    argument_type: named("String") }),
            (String::from("to"),
                ArgumentInfo {
                    description: Some(String::from("An address that will be sent the email.")),
                    argument_type: named("String") }),
            (String::from("cc"),
                ArgumentInfo {
                    description: Some(String::from("An address that will be cced the email.")),
                    argument_type: nullable(named("String")) }),
            (String::from("bcc"),
                ArgumentInfo {
                    description: Some(String::from("An address that will be bcced the email.")),
                    argument_type: nullable(named("String")) }),
            (String::from("reply_to"),
                ArgumentInfo {
                    description: Some(String::from("An array of recipients who will receive replies.")),
                    argument_type: nullable(named("String")) }),
            (String::from("subject"),
                ArgumentInfo {
                    description: Some(String::from("The subject of your email. See character length requirements according to RFC 2822.")),
                    argument_type: named("String") }),
            // (String::from("content"),
            //     ArgumentInfo {
            //         description: Some(String::from("An array where you can specify the content of your email. You can include multiple MIME types of content, but you must specify at least one MIME type.")),
            //         argument_type: named("mail_content") }),
            (String::from("content_type"),
                ArgumentInfo {
                    description: Some(String::from("The MIME type of the content you are including in your email")),
                    argument_type: named("String") }),
            (String::from("content"),
                ArgumentInfo {
                    description: Some(String::from("The actual content of the specified MIME type that you are including in your email.")),
                    argument_type: named("String") }),
                // (String::from("type"), ObjectField {
                //     r#type: named("String"),
                //     description: Some(String::from("The MIME type of the content you are including in your email"))
                // }),
                // (String::from("name"), ObjectField {
                //     r#type: named("String"),
                //     description: Some(String::from("The actual content of the specified MIME type that you are including in your email."))
                // }),
            // (String::from("attachment"),
            //     ArgumentInfo {
            //         description: Some(String::from("An object where you can specify an attachment you want to include.")),
            //         argument_type: nullable(named("mail_attachment")) }),
            (String::from("template_id"),
                ArgumentInfo {
                    description: Some(String::from("An email template ID. A template that contains a subject and content — either text or html — will override any subject and content values specified at the personalizations or message level.")),
                    argument_type: nullable(named("String")) }),
            (String::from("send_at"),
                ArgumentInfo {
                    description: Some(String::from("A unix timestamp allowing you to specify when you want your email to be delivered. This may be overridden by the send_at parameter set at the personalizations level. Delivery cannot be scheduled more than 72 hours in advance.")),
                    argument_type: nullable(named("Int")) }),
            (String::from("batch_id"),
                ArgumentInfo {
                    description: Some(String::from("An ID representing a batch of emails to be sent at the same time. Including a batch_id in your request allows you include this email in that batch. It also enables you to cancel or pause the delivery of that batch.")),
                    argument_type: nullable(named("String")) }) //,
            // (String::from("asm"),
            //     ArgumentInfo {
            //         description: Some(String::from("An object allowing you to specify how to handle unsubscribes.")),
            //         argument_type: nullable(named("unsubscription_settings")) })
        ]),
        result_type: named("send_mail_response"),
    }
}

fn named(name: &str) -> Type {
    Type::Named {
        name: String::from(name),
    }
}

fn nullable(underlying: Type) -> Type {
    Type::Nullable {
        underlying_type: Box::new(underlying),
    }
}

fn array_of(element: Type) -> Type {
    Type::Array {
        element_type: Box::new(element),
    }
}
