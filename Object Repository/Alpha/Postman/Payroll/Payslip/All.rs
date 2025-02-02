<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>All</name>
   <tag></tag>
   <elementGuidId>f67e82fc-ff30-4ce9-8c75-8a325413db4a</elementGuidId>
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
      <webElementGuid>b3195295-5e46-4efb-acfd-acc00f413ad9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>d01760f0-10d2-4ef7-be62-0ba6fa3da0d9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>${alpha_header_user_id_key}</name>
      <type>Main</type>
      <value>${alpha_header_user_id_value}</value>
      <webElementGuid>04d952b6-1670-449a-87b0-f8a54d474e5c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Proxy-Authorization</name>
      <type>Main</type>
      <value>${SECRET_SERVICE}</value>
      <webElementGuid>c6e613ea-7d29-45fa-aea4-534a9ae4843f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/payslip?status=published&amp;department_id=2&amp;period=2020-12&amp;sort_direction=desc&amp;sort_by=payslip_date&amp;limit=5&amp;page=1</restUrl>
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
      <id>e57d7616-2fb6-4778-9056-0a0a4c1a9447</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_token</defaultValue>
      <description></description>
      <id>64fcad57-bfca-4d24-9820-1bd36e839987</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_key</defaultValue>
      <description></description>
      <id>e382d26c-0f94-4d30-820b-c61f2e637271</id>
      <masked>false</masked>
      <name>alpha_header_user_id_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>9f8d4b00-05ec-4eaa-8e8f-a4fc3337250a</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>b228599d-954e-4041-9550-fede2b0b5793</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
