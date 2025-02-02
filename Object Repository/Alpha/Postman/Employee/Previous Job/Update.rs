<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update</name>
   <tag></tag>
   <elementGuidId>cbac78ab-0313-418d-b308-03671754a1fe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;employee_uuid\&quot;: \&quot;${alpha_header_user_id_value}\&quot;,\n    \&quot;company\&quot;: \&quot;PT Microsoft Indonesia\&quot;,\n    \&quot;city\&quot;: \&quot;Jakarta Selatan\&quot;,\n    \&quot;country\&quot;: \&quot;ID\&quot;,\n    \&quot;position\&quot;: \&quot;VP of Product\&quot;,\n    \&quot;description\&quot;: \&quot;I do this, I do that\&quot;,\n    \&quot;date_start_year\&quot;: 2020,\n    \&quot;date_start_month\&quot;: 1,\n    \&quot;date_start_date\&quot;: 1,\n    \&quot;date_end_year\&quot;: 2020,\n    \&quot;date_end_month\&quot;: 12,\n    \&quot;date_end_date\&quot;: 31,\n    \&quot;reason\&quot;: \&quot;No specific reason\&quot;\n}&quot;,
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
      <webElementGuid>edfa1915-ea78-4393-92b3-03f74e0c929c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>bfa95041-0893-4da8-93bb-5cfb0795ce65</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}/jobs/2</restUrl>
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
      <id>827386d2-7491-4b54-8d34-351cc8f2297d</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>0b9ff2b2-3c4e-4f37-9f29-3e41d0070804</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
