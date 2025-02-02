<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update</name>
   <tag></tag>
   <elementGuidId>1f14ce8d-cb9f-46f9-b3d4-cf4516f773d3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;name\&quot; : \&quot;Indonesia 2\&quot;\r\n}&quot;,
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
      <webElementGuid>0fea56ba-ec76-4956-b147-454f3a3fb2fe</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>41928ba2-b997-4523-ad2c-4538ccc09a39</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>2f5dfe67-4f7e-43c5-8bfd-056e85e6d1d3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/country/${country_id}</restUrl>
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
      <id>bc6d93a0-9f55-4f1b-a6f0-ab015468ff7e</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.country_id</defaultValue>
      <description></description>
      <id>4189710d-1e4d-48b8-aba8-367eb6d1041e</id>
      <masked>false</masked>
      <name>country_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>e9928404-e5cd-4734-9481-d76f540e6a31</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
