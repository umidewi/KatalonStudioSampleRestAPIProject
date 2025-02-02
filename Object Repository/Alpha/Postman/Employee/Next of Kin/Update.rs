<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update</name>
   <tag></tag>
   <elementGuidId>66d89c87-80d3-4630-b822-249b240ef2a7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;fullname\&quot;: \&quot;Jennie Doe\&quot;,\n    \&quot;relationship\&quot;: \&quot;spouse\&quot;,\n    \&quot;id_type\&quot;: \&quot;passport\&quot;,\n    \&quot;id_number\&quot;: \&quot;X123456\&quot;,\n    \&quot;id_expiration\&quot;: \&quot;2020-12-31\&quot;,\n    \&quot;occupation\&quot;: \&quot;CFO\&quot;,\n    \&quot;phone\&quot;: \&quot;081808180818\&quot;,\n    \&quot;address\&quot;: \&quot;Somewhere in Sidoarjo\&quot;,\n    \&quot;gender\&quot;: \&quot;female\&quot;,\n    \&quot;birthdate\&quot;: \&quot;1991-12-01\&quot;\n}&quot;,
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
      <webElementGuid>d4e17730-cdf4-424c-b04a-716a8a2c990d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1fcc6011-03ea-4eff-afab-435b7162d811</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}/noks/1</restUrl>
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
      <id>d66e6e4d-9c50-46be-9b61-4f1856bca0c0</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>e25dafe3-9a6b-41ed-a48d-242ff0443696</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
