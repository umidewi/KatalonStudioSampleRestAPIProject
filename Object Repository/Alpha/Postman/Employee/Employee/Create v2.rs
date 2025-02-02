<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create v2</name>
   <tag></tag>
   <elementGuidId>422c8778-8f63-4658-9ba8-e8f4d0f37f71</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot;: \&quot;John\&quot;,\n    \&quot;middlename\&quot;: \&quot;\&quot;,\n    \&quot;lastname\&quot;: \&quot;Doe\&quot;,\n    \&quot;birthdate\&quot;: \&quot;1990-01-01\&quot;,\n    \&quot;gender\&quot;: \&quot;male\&quot;,\n    \&quot;email\&quot;: \&quot;test@admin.com\&quot;,\n    \&quot;join_date\&quot;: \&quot;2000-03-01\&quot;,\n    \&quot;position_id\&quot;: \&quot;1\&quot;,\n    \&quot;password_auto_generate\&quot; : false,\n    \&quot;password\&quot; : \&quot;Abcde.123!\&quot;,\n    \&quot;password_confirmation\&quot; : \&quot;Abcde.123!\&quot;\n}&quot;,
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
      <webElementGuid>290ec131-b2d7-4577-ac71-66dfd8709539</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>93338fcf-5154-4f97-a0d2-395cc30277c8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/create</restUrl>
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
      <id>5d6ce691-41b2-45c6-a9ef-8ab68a09d654</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
