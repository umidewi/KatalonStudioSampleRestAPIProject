<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>User - all</name>
   <tag></tag>
   <elementGuidId>6ae82487-f07e-4181-995c-ae949b6e70ed</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
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
      <webElementGuid>67aad7a3-d367-4200-a994-1ad8eb673bfb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>92c9e4a6-fe4f-4991-9bd9-987a5976190a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>${alpha_header_user_id_key}</name>
      <type>Main</type>
      <value>${alpha_header_user_id_value}</value>
      <webElementGuid>2844b5b8-95fa-4cd0-ac54-97a3fb9a69d4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Proxy-Authorization</name>
      <type>Main</type>
      <value>${SECRET_SERVICE}</value>
      <webElementGuid>d61cf704-d34e-4178-89a9-b879c66b4caf</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}/payslip?year=2020</restUrl>
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
      <id>846bec3c-5790-494d-9a5f-64030f425bc2</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>80cad2aa-7c0b-4273-9a2b-b6113ab97c8b</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>de6b6908-41aa-4540-a3b0-7511d661c5e2</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_key</defaultValue>
      <description></description>
      <id>b935b6d3-1f0e-4ef7-801e-5638509f1d91</id>
      <masked>false</masked>
      <name>alpha_header_user_id_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>a7412343-99a3-476d-b15c-563816626a1d</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
