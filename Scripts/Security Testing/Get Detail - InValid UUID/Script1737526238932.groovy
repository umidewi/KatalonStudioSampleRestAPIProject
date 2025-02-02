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
import org.openqa.selenium.Keys

import groovy.console.ui.SystemOutputInterceptor
import groovy.json.JsonSlurper as JsonSlurper


'Test Case 1.1-TC.01 for GET - Generate Token'

'Send Request for Token and Capture the response'
def response = WS.sendRequest(findTestObject('Alpha/Postman/Authentication/Login', [('alpha_oauth_url') : GlobalVariable.alpha_oauth_url
	, ('alpha_gw_username') : GlobalVariable.alpha_gw_username, ('alpha_gw_password') : GlobalVariable.alpha_gw_password
	, ('alpha_gw_client_id') : GlobalVariable.alpha_gw_client_id, ('alpha_gw_client_secret') : GlobalVariable.alpha_gw_client_secret]))

'Verify status code and body content'
WS.verifyResponseStatusCode(response, 200)
def responseBody = response.getResponseBodyContent()
def token = WS.getElementPropertyValue(response, 'access_token')

GlobalVariable.alpha_token = token

println('Global Var Alpha Token is:' + GlobalVariable.alpha_token)

'Test Case 1.2-TC.01 for GET - Check Token'

'Step 2: Get User Detail'
// Find the Get User Detail API Test Object
TestObject getUserDetailRequest = (findTestObject('Alpha/Postman/Employee/Employee/Get Detail', [('alpha_gw_api_url') : GlobalVariable.alpha_gw_api_url
	, ('alpha_header_user_id_value') : GlobalVariable.invalid_alpha_header_user_id_value, ('alpha_token') :GlobalVariable.alpha_token]))
 

// Send GET request to Get User Detail API
def userDetailResponse = WS.sendRequest(getUserDetailRequest)

// Validate the response status code
WS.verifyResponseStatusCode(userDetailResponse, 403)

println("User A cannot access user B profile.")

