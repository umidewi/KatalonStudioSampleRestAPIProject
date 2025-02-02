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
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.TestObjectProperty
import groovy.json.JsonSlurper

// Define the valid token for User A
String userAToken = "Bearer validTokenForUserA"

// Define the valid user IDs
String userAId = "f15531d9-6b81-4c6b-a5ac-323cc8177bf8"  // User A ID
String userBId = "1e4de1e9-7808-43a9-af59-7eaeed41d41c"  // User B ID (unauthorized access attempt)

// Step 1: Set up the valid request for User A
RequestObject getUserProfile = findTestObject('Object Repository/Employee/Get Detail')

// Set the Authorization token for User A
getUserProfile.getHttpHeaderProperties().add(new TestObjectProperty("Authorization", TestObjectProperty.VALUE_TYPE_TEXT, userAToken))

// Replace {id} in the URL with User A's ID
String validUrl = getUserProfile.getRestUrl().replace("{id}", userAId)
getUserProfile.setRestUrl(validUrl)

// Step 2: Send the request for User A (valid scenario)
def responseForUserA = WS.sendRequest(getUserProfile)

// Verify the response status code for User A
WS.verifyResponseStatusCode(responseForUserA, 200)

// Parse the response to validate the data belongs to User A
def jsonResponseA = new JsonSlurper().parseText(responseForUserA.getResponseBodyContent())
assert jsonResponseA.id == userAId

println("User A can successfully access their own profile.")
/*
// Step 3: Modify the request for User B (IDOR attempt)
String maliciousUrl = getUserProfile.getRestUrl().replace(userAId, userBId)
getUserProfile.setRestUrl(maliciousUrl)

// Send the unauthorized request for User B's profile
def responseForUserB = WS.sendRequest(getUserProfile)

// Step 4: Validate the response
if (responseForUserB.getStatusCode() == 403 || responseForUserB.getStatusCode() == 401) {
	println("IDOR Test Passed: User A cannot access User B's profile.")
} else {
	println("IDOR Test Failed: User A can access User B's profile.")
	println("Response: " + responseForUserB.getResponseBodyContent())
	assert false
}
*/
