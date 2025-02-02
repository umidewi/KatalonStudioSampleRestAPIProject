<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update</name>
   <tag></tag>
   <elementGuidId>4cce02d5-1011-47cc-9b8e-0b46f58f0780</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;name\&quot; : \&quot;developer 79\&quot;,\r\n    \&quot;work_hour\&quot; : [\r\n        {\r\n            \&quot;work_hour_id\&quot; : 1\r\n        },\r\n        {\r\n            \&quot;work_hour_id\&quot; : 1\r\n        },\r\n        {\r\n            \&quot;work_hour_id\&quot; : 1\r\n        },\r\n        {\r\n            \&quot;work_hour_id\&quot; : 1\r\n        },\r\n        {\r\n            \&quot;work_hour_id\&quot; : 1\r\n        },\r\n        {\r\n            \&quot;work_hour_id\&quot; : 4\r\n        },\r\n        {\r\n            \&quot;work_hour_id\&quot; : 4\r\n        }\r\n    ]\r\n}&quot;,
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
      <webElementGuid>f90c19af-724b-4560-93fe-0f6b8f3da3f2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>057690bc-8e98-4a58-a03f-47ff35e78ddf</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>cada12fa-f574-49bc-9911-be17d9ca61bd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/fixed/schedules/${fixed_schedule_id}</restUrl>
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
      <id>90305eed-dee1-44e9-9dd7-f2bc001bbaaa</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.fixed_schedule_id</defaultValue>
      <description></description>
      <id>7bb54528-e271-41e3-8c0b-88ec3daaa2a5</id>
      <masked>false</masked>
      <name>fixed_schedule_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>d51099c8-68eb-4588-989c-49640d81a3fd</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>b7ef2651-c386-41a4-b232-26da8009afdb</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
