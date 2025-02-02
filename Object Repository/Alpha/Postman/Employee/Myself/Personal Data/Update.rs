<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update</name>
   <tag></tag>
   <elementGuidId>67da7b36-7abc-4399-907e-c9cecf606a8b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot;: \&quot;John\&quot;,\n    \&quot;middlename\&quot;: \&quot;Walker\&quot;,\n    \&quot;lastname\&quot;: \&quot;Doe\&quot;,\n    \&quot;birthdate\&quot;: \&quot;1990-01-01\&quot;,\n    \&quot;gender\&quot;: \&quot;male\&quot;,\n    \&quot;email\&quot;: \&quot;admin@admin.com\&quot;,\n    \&quot;join_date\&quot;: \&quot;2000-03-01\&quot;,\n    \&quot;personal_email\&quot;: \&quot;johndoe@mailinator.com\&quot;,\n    \&quot;blood_type\&quot;: \&quot;a\&quot;,\n    \&quot;blood_type_rh\&quot;: \&quot;+\&quot;,\n    \&quot;marital_status\&quot;: \&quot;single\&quot;,\n    \&quot;phone\&quot;: \&quot;081808180818\&quot;,\n    \&quot;mobile_phone\&quot;: \&quot;081808180818\&quot;,\n    \&quot;address\&quot;: \&quot;Intiland Tower Surabaya\&quot;,\n    \&quot;home_address\&quot;: \&quot;Somewhere in Surabaya\&quot;,\n    \&quot;id_type\&quot;: \&quot;passport\&quot;,\n    \&quot;id_number\&quot;: \&quot;X123456\&quot;,\n    \&quot;id_expiration\&quot;: \&quot;2020-12-31\&quot;,\n    \&quot;family_card_no\&quot;: \&quot;1234567890123456\&quot;,\n    \&quot;avatar_url\&quot;: \&quot;/images/temp/brie_larson.jpeg\&quot;,\n    \&quot;internal_id\&quot;: \&quot;0003001\&quot;,\n    \&quot;licenses\&quot; : [\n        {\n            \&quot;license_type\&quot;       : \&quot;id_card\&quot;, \n            \&quot;license_number\&quot;     : \&quot;1234567\&quot;, \n            \&quot;license_expiration\&quot; : \&quot;2023-12-31\&quot;, \n            \&quot;license_file\&quot;       : null,\n            \&quot;is_new\&quot; : 1\n        },\n        {\n            \&quot;license_type\&quot;       : \&quot;id_card\&quot;, \n            \&quot;license_number\&quot;     : \&quot;1234567\&quot;, \n            \&quot;license_expiration\&quot; : \&quot;2023-12-31\&quot;, \n            \&quot;license_file\&quot;       : null,\n            \&quot;is_replace\&quot; : 1\n        }\n    ]\n}&quot;,
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
      <webElementGuid>224bf259-c21b-4392-8b88-0d8e315e11f6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>60d01be2-b9d9-4a60-b0a4-82a0fbe590d1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/_self/employees</restUrl>
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
      <id>56021633-1096-48ee-82fe-8074931b2520</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
