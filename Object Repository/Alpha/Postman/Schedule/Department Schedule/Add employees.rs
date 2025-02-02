<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add employees</name>
   <tag></tag>
   <elementGuidId>bfe9c22c-b58b-4769-a1d7-bd0c814dcca9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;period\&quot; : \&quot;2020-12-21\&quot;,\r\n    \&quot;work_hour_id\&quot; : 1,\r\n    \&quot;employees\&quot; : [\r\n        {\r\n            \&quot;employee_id\&quot; : \&quot;07626cab-60dc-45c5-9122-9e831eed983e\&quot;\r\n        },\r\n        {\r\n            \&quot;employee_id\&quot; : \&quot;3b5dcf77-1757-45ff-8128-d3c6fa068aa9\&quot;\r\n        },\r\n        {\r\n            \&quot;employee_id\&quot; : \&quot;332fbd8f-1e3d-493e-92e8-f8fc4948a652\&quot;\r\n        }\r\n    ]\r\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Proxy-Authorization</name>
      <type>Main</type>
      <value>${SECRET_SERVICE}</value>
      <webElementGuid>6b32c469-e446-443c-9d26-d05d1b5e731f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>91a8f547-2574-4970-91f9-ca36610a3ee5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>e2f6d0ef-d433-4908-b9e0-c6f15dfae287</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/departments/${department_id}/schedules</restUrl>
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
      <id>23eb9e05-98c6-4b3f-98b2-d47a72d97594</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.department_id</defaultValue>
      <description></description>
      <id>6f4bfc77-d2fc-4b4c-9a76-8d4e231f5840</id>
      <masked>false</masked>
      <name>department_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>4e2d6a1e-2071-43d9-8f2d-e5fff215aa60</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>c9e36b40-b10e-45b4-89cd-e3c709989226</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
