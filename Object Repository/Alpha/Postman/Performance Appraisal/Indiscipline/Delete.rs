<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Delete</name>
   <tag></tag>
   <elementGuidId>45937f51-5494-43e1-8ba9-2b28da6cc73e</elementGuidId>
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
      <webElementGuid>3dd7cec7-e5b9-4695-beea-5ddb2de9ec0e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>31407031-73b6-4db7-a9de-2a7fc65edbbc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>${alpha_header_user_id_key}</name>
      <type>Main</type>
      <value>${alpha_header_user_id_value}</value>
      <webElementGuid>2e54ce98-d0e1-46bd-a6f3-ef964b10a26e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Proxy-Authorization</name>
      <type>Main</type>
      <value>${SECRET_SERVICE}</value>
      <webElementGuid>05e37b39-6f91-4ba5-9afb-0d26e72ff7cf</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Company-Id</name>
      <type>Main</type>
      <value>1</value>
      <webElementGuid>8f09821e-2143-488e-95bb-0a006df00f46</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/indisciplines/10</restUrl>
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
      <id>b18f73f9-3326-4c05-b9dd-73b56be26705</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>411ac631-5c2f-40ef-a622-a31205612d20</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_key</defaultValue>
      <description></description>
      <id>23d53c3c-ac7e-4d93-8cc7-0ebcef04a628</id>
      <masked>false</masked>
      <name>alpha_header_user_id_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>80c7b581-6f49-4565-9490-2d35dc5d1edd</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>9d4dc182-7f0c-400a-bb78-35be512b1103</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
