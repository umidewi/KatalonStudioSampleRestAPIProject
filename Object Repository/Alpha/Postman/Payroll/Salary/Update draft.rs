<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update draft</name>
   <tag></tag>
   <elementGuidId>b34d5011-d891-4d4c-9f65-76beb7bc614e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;pay_type_id\&quot;: 2,\r\n    \&quot;pay_period_id\&quot;: 2,\r\n    \&quot;base_nominal\&quot;: 6842102,\r\n    \&quot;allowance\&quot;: [\r\n        {\r\n            \&quot;type_id\&quot;: 1,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 12345\r\n        },\r\n        {\r\n            \&quot;type_id\&quot;: 2,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 54321\r\n        }\r\n    ],\r\n    \&quot;deduction\&quot;: [\r\n        {\r\n            \&quot;type_id\&quot;: 3,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 25000\r\n        },\r\n        {\r\n            \&quot;type_id\&quot;: 4,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 50000\r\n        }\r\n    ],\r\n    \&quot;reasons\&quot; : [\r\n        {\r\n            \&quot;reason_id\&quot; : 1 \r\n        }, \r\n        {\r\n            \&quot;reason_id\&quot; : 2\r\n        }\r\n    ],\r\n    \&quot;effective_date\&quot; : \&quot;2020-08-31\&quot;,\r\n    \&quot;custom_reason\&quot; : null\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a2117437-60f3-498a-a129-308714d8dfca</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>68caf911-122a-46dc-a3f9-9b295d995a4f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>${alpha_header_user_id_key}</name>
      <type>Main</type>
      <value>${alpha_header_user_id_value}</value>
      <webElementGuid>d1fb4d63-a0e5-4f2c-b810-689f2e83cf1a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Proxy-Authorization</name>
      <type>Main</type>
      <value>${SECRET_SERVICE}</value>
      <webElementGuid>2dacc64d-c9af-4af0-a2d7-33dddf760ea4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}/salaries/17</restUrl>
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
      <id>c6cc35e5-5a32-404f-ab3b-498de4d43656</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>395de976-b9f3-4745-9b93-0041b3987d35</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>1dfdea48-754b-47c3-b31a-f63364e1082a</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_key</defaultValue>
      <description></description>
      <id>35782111-1395-454a-99a6-3fd20a0fce19</id>
      <masked>false</masked>
      <name>alpha_header_user_id_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>e463bc98-dc04-4f9c-8e57-1906bd769e3c</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
