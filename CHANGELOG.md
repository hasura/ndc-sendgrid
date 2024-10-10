# SendGrid Connector Changelog
This changelog documents changes between release tags.


## [Unreleased]
Upcoming changes for the next versioned release.

## [0.6.0] - 2024-10-10
* Update NDC SDK to v0.4.0 (NDC Spec v0.1.6) ([#10](https://github.com/hasura/ndc-sendgrid/pull/10))

## [0.5.0] - 2024-02-28
* Fix incorrect types used on unsubscription_settings object type ([#9](https://github.com/hasura/ndc-sendgrid/pull/9))

## [0.4] - 2024-02-21
* Updated with the latest NDC SDK version that supports NDC Spec v0.1.0-rc.16
* send_mail procedure now takes the full send mail request type and uses nested objects

## [0.3] - 2023-10-26
* Simplified send_mail inputs to work around v3-engine missing argument object type support

## [0.2] - 2023-09-15
Updates include:

* Updating the Spec and SDK dependencies
* Auth Support via `SERVICE_TOKEN_SECRET`

## [0.1] - 2023-09-14
Initial releaase of SendGrid Connector.

Supports:

* Listing email templates
* Sending emails
