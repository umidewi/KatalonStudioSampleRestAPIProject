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


'Test Case 1.1-TC.01 for GET - Generate Token'

'Send Request for Token and Capture the response'
def response = WS.sendRequest(findTestObject('Alpha/Postman/Authentication/Login', [('alpha_oauth_url') : GlobalVariable.alpha_oauth_url
	, ('alpha_gw_username') : GlobalVariable.username_sql, ('alpha_gw_password') : GlobalVariable.alpha_gw_password
	, ('alpha_gw_client_id') : GlobalVariable.alpha_gw_client_id, ('alpha_gw_client_secret') : GlobalVariable.alpha_gw_client_secret]))

'Verify status code and body content'
WS.verifyResponseStatusCode(response, 404)

// Verify the response status code for unauthorized access
if (response.getStatusCode() == 404 ) {
	println("SQL Injection Test Passed")
} else {
	println("SQL Injection Test Failed: Allowed return token!")
	println("Response: " + response.getResponseBodyContent())
	assert false  // Fail the test case if unauthorized access is allowed
}