<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create</name>
   <tag></tag>
   <elementGuidId>721a2872-9986-45bb-9320-58b04e131cc9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;employee_uuid\&quot;: \&quot;${alpha_header_user_id_value}\&quot;,\n    \&quot;company\&quot;: \&quot;PT Google Indonesia\&quot;,\n    \&quot;city\&quot;: \&quot;Jakarta Selatan\&quot;,\n    \&quot;country\&quot;: \&quot;ID\&quot;,\n    \&quot;position\&quot;: \&quot;Senior Product Manager\&quot;,\n    \&quot;description\&quot;: \&quot;I do this, I do that\&quot;,\n    \&quot;date_start_year\&quot;: 2012,\n    \&quot;date_start_month\&quot;: 1,\n    \&quot;date_start_date\&quot;: 1,\n    \&quot;date_end_year\&quot;: 2019,\n    \&quot;date_end_month\&quot;: 12,\n    \&quot;date_end_date\&quot;: 31,\n    \&quot;reason\&quot;: \&quot;No specific reason\&quot;\n}&quot;,
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
      <webElementGuid>4f551c00-4591-4f4f-960b-bfa701a965ca</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5d79c772-5220-414c-8d0c-70f93ed29076</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}/jobs</restUrl>
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
      <id>c9979f3d-2f6b-49d3-9542-ade8b541fcd6</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>110e76be-c2c1-4437-a59d-8b4f39204c5a</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
