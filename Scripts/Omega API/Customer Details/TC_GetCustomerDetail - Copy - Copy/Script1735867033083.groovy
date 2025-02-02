import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import com.github.fge.jsonschema.main.JsonSchemaFactory

// Send the API request
def response =WS.sendRequest(findTestObject('Omega API/Postman/Admin-API/API Admin - customer details - schema', [('api_url') : GlobalVariable.api_url
            , ('app_key') : 'vm99fEaPs7cb9DhkzLnHehht', ('version') : '2', ('username') : 'rusidi', ('action') : 'customer_getdetails'
            , ('customer_id') : '6079']))
// Get the JSON response
def jsonResponse = response.getResponseText()

// Load and parse the schema
def schemaContent = '''
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "result": {
      "type": "boolean"
    },
    "customer_id": {
      "type": "string"
    },
    "type": {
      "type": "string"
    },
    "disc_level": {
      "type": "string"
    },
    "discount": {
      "type": "string"
    },
    "company_id": {
      "type": "string"
    },
    "whmcs_clientid": {
      "type": "null"
    },
    "sales_person_id": {
      "type": "string"
    },
    "sales_person_name": {
      "type": "string"
    },
    "account_manager_id": {
      "type": "string"
    },
    "account_manager_name": {
      "type": "string"
    },
    "salutation": {
      "type": "string"
    },
    "first_name": {
      "type": "string"
    },
    "middle_name": {
      "type": "null"
    },
    "last_name": {
      "type": "string"
    },
    "company_name": {
      "type": "string"
    },
    "address": {
      "type": "string"
    },
    "address_cont": {
      "type": "null"
    },
    "city": {
      "type": "string"
    },
    "state": {
      "type": "string"
    },
    "zip": {
      "type": "string"
    },
    "province": {
      "type": "string"
    },
    "country_code": {
      "type": "string"
    },
    "country": {
      "type": "string"
    },
    "phone_full": {
      "type": "string"
    },
    "phone": {
      "type": "string"
    },
    "phone_country_code": {
      "type": "string"
    },
    "phone_ext": {
      "type": "string"
    },
    "fax": {
      "type": "string"
    },
    "primary_email_address": {
      "type": "string"
    },
    "secondary_email_address": {
      "type": "string"
    },
    "website": {
      "type": "string"
    },
    "im_address": {
      "type": "null"
    },
    "im_type": {
      "type": "null"
    },
    "join_date": {
      "type": "string"
    },
    "sharepoint_url": {
      "type": "null"
    },
    "receive_nagios_alert_opt": {
      "type": "boolean"
    },
    "receive_newsletter_opt": {
      "type": "boolean"
    },
    "last_login": {
      "type": "string"
    },
    "additional_contacts": {
      "type": "array",
      "items": [
        {
          "type": "object",
          "properties": {
            "contact_id": {
              "type": "string"
            },
            "contact_name": {
              "type": "string"
            },
            "contact_name_cont": {
              "type": "string"
            },
            "email_address": {
              "type": "string"
            },
            "secondary_email_address": {
              "type": "string"
            },
            "phone": {
              "type": "string"
            },
            "contact_time": {
              "type": "string"
            },
            "category": {
              "type": "string"
            },
            "keycard": {
              "type": "string"
            },
            "is_active": {
              "type": "string"
            },
            "is_has_portal_account": {
              "type": "boolean"
            },
            "contact_can_add_contacts_opt": {
              "type": "boolean"
            },
            "contact_receive_notification_opt": {
              "type": "boolean"
            },
            "contact_receive_nagios_alert_opt": {
              "type": "boolean"
            },
            "contact_notif_problem_opt": {
              "type": "boolean"
            },
            "contact_dc_access_opt": {
              "type": "boolean"
            },
            "contact_can_add_equipment_opt": {
              "type": "boolean"
            },
            "contact_can_create_ticket_opt": {
              "type": "boolean"
            },
            "contact_notif_billing_opt": {
              "type": "boolean"
            },
            "contact_notif_neworder_opt": {
              "type": "boolean"
            },
            "contact_can_add_contract_opt": {
              "type": "boolean"
            }
          },
          "required": [
            "contact_id",
            "contact_name",
            "contact_name_cont",
            "email_address",
            "secondary_email_address",
            "phone",
            "contact_time",
            "category",
            "keycard",
            "is_active",
            "is_has_portal_account",
            "contact_can_add_contacts_opt",
            "contact_receive_notification_opt",
            "contact_receive_nagios_alert_opt",
            "contact_notif_problem_opt",
            "contact_dc_access_opt",
            "contact_can_add_equipment_opt",
            "contact_can_create_ticket_opt",
            "contact_notif_billing_opt",
            "contact_notif_neworder_opt",
            "contact_can_add_contract_opt"
          ]
        }
      ]
    },
    "reseller_contacts": {
      "type": "array",
      "items": {}
    },
    "billing_info": {
      "type": "array",
      "items": {}
    },
    "confidential_files": {
      "type": "object",
      "properties": {
        "63": {
          "type": "object",
          "properties": {
            "file_name": {
              "type": "string"
            },
            "file_type": {
              "type": "string"
            },
            "note": {
              "type": "string"
            }
          },
          "required": [
            "file_name",
            "file_type",
            "note"
          ]
        }
      },
      "required": [
        "63"
      ]
    },
    "confidential_file_histories": {
      "type": "object",
      "properties": {
        "63": {
          "type": "object",
          "properties": {
            "message": {
              "type": "string"
            },
            "created_by": {
              "type": "string"
            },
            "created_date": {
              "type": "string"
            }
          },
          "required": [
            "message",
            "created_by",
            "created_date"
          ]
        }
      },
      "required": [
        "63"
      ]
    },
    "notes": {
      "type": "array",
      "items": {}
    },
    "is_active": {
      "type": "string"
    },
    "is_blacklist": {
      "type": "string"
    },
    "blacklist_reason": {
      "type": "null"
    },
    "is_managed": {
      "type": "null"
    },
    "type_managed": {
      "type": "null"
    },
    "unit_managed": {
      "type": "null"
    },
    "slack_channel": {
      "type": "null"
    },
    "access_code": {
      "type": "null"
    },
    "keycard": {
      "type": "string"
    },
    "wiki_access": {
      "type": "string"
    },
    "fbc": {
      "type": "string"
    },
    "twc": {
      "type": "string"
    },
    "linkedin_address": {
      "type": "string"
    },
    "how_know_us": {
      "type": "string"
    },
    "tags": {
      "type": "array",
      "items": {}
    },
    "active_server_count": {
      "type": "integer"
    },
    "ordered_product_count": {
      "type": "integer"
    },
    "max_product_count": {
      "type": "boolean"
    },
    "total_monthly": {
      "type": "null"
    },
    "max_revenue": {
      "type": "boolean"
    },
    "is_has_suspended_product": {
      "type": "boolean"
    }
  },
  "required": [
    "result",
    "customer_id",
    "type",
    "disc_level",
    "discount",
    "company_id",
    "whmcs_clientid",
    "sales_person_id",
    "sales_person_name",
    "account_manager_id",
    "account_manager_name",
    "salutation",
    "first_name",
    "middle_name",
    "last_name",
    "company_name",
    "address",
    "address_cont",
    "city",
    "state",
    "zip",
    "province",
    "country_code",
    "country",
    "phone_full",
    "phone",
    "phone_country_code",
    "phone_ext",
    "fax",
    "primary_email_address",
    "secondary_email_address",
    "website",
    "im_address",
    "im_type",
    "join_date",
    "sharepoint_url",
    "receive_nagios_alert_opt",
    "receive_newsletter_opt",
    "last_login",
    "additional_contacts",
    "reseller_contacts",
    "billing_info",
    "confidential_files",
    "confidential_file_histories",
    "notes",
    "is_active",
    "is_blacklist",
    "blacklist_reason",
    "is_managed",
    "type_managed",
    "unit_managed",
    "slack_channel",
    "access_code",
    "keycard",
    "wiki_access",
    "fbc",
    "twc",
    "linkedin_address",
    "how_know_us",
    "tags",
    "active_server_count",
    "ordered_product_count",
    "max_product_count",
    "total_monthly",
    "max_revenue",
    "is_has_suspended_product"
  ]
}
'''
def parsedSchema = new JsonSlurper().parseText(schemaContent)

// Parse the response JSON
def parsedResponse = new JsonSlurper().parseText(jsonResponse)

// Validate the schema
def schemaFactory = JsonSchemaFactory.byDefault()
def jsonSchema = schemaFactory.getJsonSchema(parsedSchema.toString())
def validationReport = jsonSchema.validate(parsedResponse)

// Check the validation report
if (!validationReport.isSuccess()) {
    println("Schema Validation Failed: " + validationReport)
    assert false // Mark test as failed
} else {
    println("Schema Validation Passed")
}
