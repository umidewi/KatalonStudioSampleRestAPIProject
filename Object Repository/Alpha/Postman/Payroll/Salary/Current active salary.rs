<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Current active salary</name>
   <tag></tag>
   <elementGuidId>3918c27c-3909-4d27-8207-b87fd7397f39</elementGuidId>
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
      <webElementGuid>833e38f1-6232-417f-8e7e-b9fc4a1545b0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>569d5306-e330-40e7-a23d-f21af87b0dcb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>${alpha_header_user_id_key}</name>
      <type>Main</type>
      <value>${alpha_header_user_id_value}</value>
      <webElementGuid>43dbcd87-7622-4f3f-a53a-dcb88912e8e2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Proxy-Authorization</name>
      <type>Main</type>
      <value>${SECRET_SERVICE}</value>
      <webElementGuid>818f8bc3-a1f3-47dd-8a2c-2df12bd136e8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}/salaries/active</restUrl>
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
      <id>1195a708-0837-45fe-851e-9460d3108af2</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>3a30c1c0-6826-48f7-8dc5-675185e2c9f7</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>d1817a72-8847-4bb7-b8c8-3365a99082e7</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_key</defaultValue>
      <description></description>
      <id>aae620c3-0976-49bc-99a6-494b702c56f7</id>
      <masked>false</masked>
      <name>alpha_header_user_id_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>1630f459-036a-4b6c-8545-c155603fde71</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
