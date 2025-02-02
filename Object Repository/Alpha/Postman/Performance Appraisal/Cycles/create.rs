<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create</name>
   <tag></tag>
   <elementGuidId>c20a0889-07b0-44b1-9584-5828e10c84a4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;name\&quot;: \&quot;Cycle : monthly, join_date, before 2 week, 1 times with new settings\&quot;,\r\n    \&quot;status\&quot;: 7,\r\n    \&quot;timeframe_id\&quot;: 1,\r\n    \&quot;custom_duration\&quot;: null,\r\n    \&quot;custom_period\&quot;: null,\r\n    \&quot;cycle_period\&quot;: 1,\r\n    \&quot;timeframe_date\&quot;: \&quot;join_date\&quot;,\r\n    \&quot;timeframe_duration\&quot;: null,\r\n    \&quot;timeframe_period\&quot;: null,\r\n    \&quot;timeframe_spesific_date\&quot;: null,\r\n    \&quot;generate_option\&quot;: 0,\r\n    \&quot;review_duration\&quot;: 2,\r\n    \&quot;review_period\&quot;: 2,\r\n    \&quot;cycle_targets\&quot;: [\r\n        {\r\n            \&quot;review_target\&quot;: \&quot;filtered_employee\&quot;,\r\n            \&quot;detail_value\&quot;: [\r\n                {\r\n                    \&quot;department_id\&quot;: [\r\n                        1,\r\n                        2\r\n                    ],\r\n                    \&quot;department_category_id\&quot;: [\r\n                        3,\r\n                        4\r\n                    ]\r\n                },\r\n                {\r\n                    \&quot;department_id\&quot;: [\r\n                        1\r\n                    ],\r\n                    \&quot;department_category_id\&quot;: [\r\n                        1\r\n                    ],\r\n                    \&quot;employment_period_id\&quot;: [\r\n                        2\r\n                    ],\r\n                    \&quot;position_id\&quot;: [\r\n                        3\r\n                    ]\r\n                }\r\n            ]\r\n        }\r\n    ],\r\n    \&quot;cycle_details\&quot;: [\r\n        {\r\n            \&quot;review_method_id\&quot;: 1, // Untuk manager review\r\n            \&quot;template_id\&quot;: 5,\r\n            \&quot;set_weight\&quot;: 50\r\n        },\r\n        {\r\n            \&quot;review_method_id\&quot;: 2, // Untuk peer review\r\n            \&quot;template_id\&quot;: 5,\r\n            \&quot;set_weight\&quot;: 5\r\n        },\r\n        {\r\n            \&quot;review_method_id\&quot;: 3, // Untuk self review\r\n            \&quot;template_id\&quot;: 5,\r\n            \&quot;set_weight\&quot;: 5\r\n        }\r\n    ],\r\n    \&quot;settings\&quot;: {\r\n        \&quot;quality\&quot;: {\r\n            \&quot;template_id\&quot;: 1,\r\n            \&quot;set_weight\&quot;: 5\r\n        },\r\n        \&quot;overall\&quot;: {\r\n            \&quot;set_weight\&quot;: 5,\r\n            \&quot;overall_detail\&quot;: [\r\n                {\r\n                    \&quot;Question\&quot;: \&quot;Question A\&quot;,\r\n                    \&quot;Score\&quot;: 20\r\n                },\r\n                {\r\n                    \&quot;Question\&quot;: \&quot;Question B\&quot;,\r\n                    \&quot;Score\&quot;: 30\r\n                }\r\n            ]\r\n        },\r\n        \&quot;indisciplinary\&quot;: {\r\n            \&quot;set_weight\&quot;: 5,\r\n            \&quot;detail\&quot;: [\r\n                {\r\n                    // Misal sick\r\n                    \&quot;type\&quot;: 1,\r\n                    \&quot;from\&quot;: \&quot;Alpha\&quot;,\r\n                    \&quot;set_weight\&quot;: 10\r\n                },\r\n                {\r\n                    \&quot;type\&quot;: 2,\r\n                    \&quot;from\&quot;: \&quot;Alpha\&quot;,\r\n                    \&quot;set_weight\&quot;: 20\r\n                },\r\n                {\r\n                    \&quot;type\&quot;: 3,\r\n                    \&quot;from\&quot;: \&quot;Manual\&quot;,\r\n                    \&quot;set_weight\&quot;: 70\r\n                }\r\n            ]\r\n        },\r\n        \&quot;certification\&quot;: 1,\r\n        \&quot;recommendations\&quot;: [\r\n            1,\r\n            2,\r\n            3\r\n        ],\r\n        \&quot;additional_notes\&quot;: 1,\r\n        \&quot;is_assign_manager\&quot; : 1\r\n    }\r\n}&quot;,
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
      <webElementGuid>159a4632-fd42-43ea-9fce-6b966f9dd29b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>f17cadb3-3216-4c86-a94b-86420d6a9b82</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>${alpha_header_user_id_key}</name>
      <type>Main</type>
      <value>${alpha_header_user_id_value}</value>
      <webElementGuid>46763df9-6f86-46cc-bb4f-3a14e1272099</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Proxy-Authorization</name>
      <type>Main</type>
      <value>${SECRET_SERVICE}</value>
      <webElementGuid>eba27c2e-3895-4b10-8043-03b088db0c58</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Company-Id</name>
      <type>Main</type>
      <value>1</value>
      <webElementGuid>62b5e042-201c-4c4c-9a24-6f1597740514</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/cycles</restUrl>
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
      <id>72a05931-c2e1-414e-8315-79bbb4991995</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>2260e72c-92b9-40d5-b3ee-7e51f3a3e0c9</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_key</defaultValue>
      <description></description>
      <id>786a18d0-3630-4d7e-9865-44037cf4c5d3</id>
      <masked>false</masked>
      <name>alpha_header_user_id_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>3941c9ee-fe67-4378-ba65-97de2072c65e</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>461fa02b-ae19-4ed6-b196-72d3d05b849a</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
