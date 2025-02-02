import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import com.github.fge.jsonschema.main.JsonSchemaFactory
import com.github.fge.jsonschema.core.report.ProcessingReport
import com.github.fge.jsonschema.core.exceptions.ProcessingException

// Send the API Request
def response = WS.sendRequest(findTestObject('Omega API/Postman/Admin-API/API Admin - customer details - schema', [('api_url') : GlobalVariable.api_url
            , ('app_key') : 'vm99fEaPs7cb9DhkzLnHehht', ('version') : '2', ('username') : 'rusidi', ('action') : 'customer_getdetails'
            , ('customer_id') : '6079']))

// Get the JSON response as String
def jsonResponse = response.getResponseText()

// Parse the JSON response
def parsedResponse = new JsonSlurper().parseText(jsonResponse)

def schemaFile = new File("C:/Users/umi.rahmawati/Katalon Studio/API Test/JSONSchema_GetCustomerDetails.json")
def schemaContent = schemaFile.text

/* Define the JSON Schema
String schema = '''
{
  "$schema": "http://json-schema.oderg/draft-07/schema",
  "type": "object",
  "properties": {
    "id": { "type": "integer" },
    "name": { "type": "string" },
    "age": { "type": "integer" },
    "email": { "type": "string", "format": "email" }
  },
  "required": ["id", "name", "age", "email"]
}
'''
*/
// Parse the schema
def schemaJson = new JsonSlurper().parseText(schemaContent)

// Create a JSON Schema Validator
def schemaFactory = JsonSchemaFactory.byDefault()
def jsonSchema = schemaFactory.getJsonSchema(schemaJson.toString())

// Validate the JSON response against the schema
ProcessingReport report = jsonSchema.validate(parsedResponse)

// Check if the validation passed
if (!report.isSuccess()) {
	println("Schema Validation Failed: " + report)
	assert false // Fail the test case
} else {
	println("Schema Validation Passed")
}


