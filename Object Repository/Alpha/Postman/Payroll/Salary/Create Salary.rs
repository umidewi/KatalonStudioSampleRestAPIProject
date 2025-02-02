<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Salary</name>
   <tag></tag>
   <elementGuidId>f3a5db64-e9c9-484b-b82a-4d8c47ec0d11</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;pay_type_id\&quot;: 2,\r\n    \&quot;pay_period_id\&quot;: 2,\r\n    \&quot;base_nominal\&quot;: 7000000,\r\n    \&quot;allowance\&quot;: [\r\n        {\r\n            \&quot;type_id\&quot;: 1,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 2000000\r\n        },\r\n        {\r\n            \&quot;type_id\&quot;: 2,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 2500000\r\n        }\r\n    ],\r\n    \&quot;deduction\&quot;: [\r\n        {\r\n            \&quot;type_id\&quot;: 3,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 300000\r\n        },\r\n        {\r\n            \&quot;type_id\&quot;: 4,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 50000\r\n        }\r\n    ],\r\n    \&quot;tax\&quot;: [\r\n        {\r\n            \&quot;type_id\&quot;: 1,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 10000\r\n        },\r\n        {\r\n            \&quot;type_id\&quot;: 2,\r\n            \&quot;currency\&quot;: \&quot;idr\&quot;,\r\n            \&quot;value\&quot;: 20000\r\n        }\r\n    ],\r\n    \&quot;reasons\&quot; : [\r\n        {\r\n            \&quot;reason_id\&quot; : 1 \r\n        }, \r\n        {\r\n            \&quot;reason_id\&quot; : 2\r\n        }\r\n    ],\r\n    \&quot;effective_date\&quot; : \&quot;2020-08-31\&quot;,\r\n    \&quot;custom_reason\&quot; : \&quot;lorem ipsum dolor sit amet\&quot;\r\n}&quot;,
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
      <webElementGuid>4da3513e-2387-4740-865d-e55768e3010d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>73a98a4e-8062-4963-b61e-2731cc0c6b97</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>${alpha_header_user_id_key}</name>
      <type>Main</type>
      <value>${alpha_header_user_id_value}</value>
      <webElementGuid>855fd37e-4722-47ca-926e-37e974bd65b2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Proxy-Authorization</name>
      <type>Main</type>
      <value>${SECRET_SERVICE}</value>
      <webElementGuid>6b9f770a-d2d9-4940-9993-d63126b2ad38</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}/salaries</restUrl>
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
      <id>beb41a68-a1c1-46f5-a92a-4fde5963775f</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>b16b7d42-12a9-4257-b156-240f48a4f916</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>359b9682-8c4c-48f4-9833-1b397e183485</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_key</defaultValue>
      <description></description>
      <id>b35ea8ef-7be1-4d5c-948f-679e3638dd81</id>
      <masked>false</masked>
      <name>alpha_header_user_id_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>fc952ba1-4051-4c53-8d7d-dcbb9e3bf50c</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
