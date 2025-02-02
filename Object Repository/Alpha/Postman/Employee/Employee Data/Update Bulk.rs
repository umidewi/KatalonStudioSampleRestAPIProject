<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Bulk</name>
   <tag></tag>
   <elementGuidId>c3b94357-ec1d-4d9e-9580-a32fbf0ebee8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;data\&quot;: [\n        {\n            \&quot;record_id\&quot;: 1,\n            \&quot;category_id\&quot;: 2,\n            \&quot;doc_type\&quot;: \&quot;text\&quot;,\n            \&quot;doc_label\&quot;: \&quot;Certificate No.\&quot;,\n            \&quot;doc_value\&quot;: \&quot;1234567812345679\&quot;\n        },\n        {\n            \&quot;record_id\&quot;: 2,\n            \&quot;category_id\&quot;: 2,\n            \&quot;doc_type\&quot;: \&quot;date\&quot;,\n            \&quot;doc_label\&quot;: \&quot;Expiration Date\&quot;,\n            \&quot;doc_value\&quot;: \&quot;2020-12-31\&quot;\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d8e846af-432a-4023-a0d9-39230b457b62</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0175928a-6d45-4431-afca-93304460570a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}/data</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.alpha_gw_api_url</defaultValue>
      <description></description>
      <id>2e83e5a0-7076-4643-9c21-e38972bcb33b</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>f3dde0ea-c0b7-40b2-8347-c77ac2cf31fb</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
