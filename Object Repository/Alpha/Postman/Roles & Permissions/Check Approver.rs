<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Check Approver</name>
   <tag></tag>
   <elementGuidId>8fe34e7c-6054-4369-84ae-af71c99c818c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;links\&quot;: [\n        {\n            \&quot;name\&quot;: \&quot;Menu\&quot;,\n            \&quot;separator\&quot;: false,\n            \&quot;children\&quot;: [\n                {\n                    \&quot;label\&quot;: \&quot;Dashboard\&quot;,\n                    \&quot;path\&quot;: \&quot;user_dashboard\&quot;,\n                    \&quot;permissions\&quot;: [\n                        \&quot;page.user.dashboard\&quot;\n                    ],\n                    \&quot;icon\&quot;: \&quot;fas fa-fw fa-house-user\&quot;\n                },\n                {\n                    \&quot;label\&quot;: \&quot;Profile\&quot;,\n                    \&quot;path\&quot;: \&quot;user_profile_index\&quot;,\n                    \&quot;permissions\&quot;: [\n                        \&quot;page.user.profile.index\&quot;,\n                        \&quot;page.user.profile.personal.list\&quot;\n                    ],\n                    \&quot;icon\&quot;: \&quot;fas fa-fw fa-user\&quot;\n                }\n            ]\n        }\n    ]\n}&quot;,
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
      <webElementGuid>ecd3711c-0f68-48fa-bb1d-09eb7b215f4d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>8c707593-a29b-4752-8a2c-f8c6e440d8ab</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/_self/employees/check-approver</restUrl>
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
      <id>afa6b7fe-03c9-4241-ba4e-4fdb303a2a21</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
