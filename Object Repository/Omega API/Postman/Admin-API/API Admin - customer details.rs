<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API Admin - customer details</name>
   <tag></tag>
   <elementGuidId>f9a26e7f-105b-4521-997c-adb3e64565ac</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;app_key&quot;,
      &quot;value&quot;: &quot;${app_key}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;username&quot;,
      &quot;value&quot;: &quot;${username}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;version&quot;,
      &quot;value&quot;: &quot;${version}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;action&quot;,
      &quot;value&quot;: &quot;${action}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;customer_id&quot;,
      &quot;value&quot;: &quot;${customer_id}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <webElementGuid>eac5fdb3-cf08-47a8-9a8c-9fe4a4747d9d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${api_url}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.api_url</defaultValue>
      <description></description>
      <id>5cfd7718-7d76-4c17-a578-0dcf295bae97</id>
      <masked>false</masked>
      <name>api_url</name>
   </variables>
   <variables>
      <defaultValue>'vm99fEaPs7cb9DhkzLnHehht'</defaultValue>
      <description></description>
      <id>73592467-5dfd-4f77-80b8-0f48a29e0db5</id>
      <masked>false</masked>
      <name>app_key</name>
   </variables>
   <variables>
      <defaultValue>'2'</defaultValue>
      <description></description>
      <id>d0d961f2-18db-42a1-9aea-e00513cdab33</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>'rusidi'</defaultValue>
      <description></description>
      <id>5b0fc5a5-8f3d-450f-bae1-f23f5cdeee98</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'customer_getdetails'</defaultValue>
      <description></description>
      <id>58589954-664a-40a1-8df2-c7725041170b</id>
      <masked>false</masked>
      <name>action</name>
   </variables>
   <variables>
      <defaultValue>'6079'</defaultValue>
      <description></description>
      <id>9e2cd080-e373-4f05-9825-944102050917</id>
      <masked>false</masked>
      <name>customer_id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'first_name', 'Adi')

WS.verifyElementPropertyValue(response, '', '')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
