<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Validate Menus</name>
   <tag></tag>
   <elementGuidId>325218eb-8bcd-4cb7-8a64-3ed47466127a</elementGuidId>
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
      <webElementGuid>447ba7ca-3d21-4471-9214-c67e865ba0fa</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>28320ee4-c1c6-4e07-9a5c-dd2f2e86d5c2</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/permissions/menus</restUrl>
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
      <id>e43386d8-0ce9-4e4e-a539-69ebf8145ea6</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
