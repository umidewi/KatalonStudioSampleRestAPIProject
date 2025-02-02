<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create</name>
   <tag></tag>
   <elementGuidId>5995eafd-6b0f-4d6e-89ea-e7cd23a38014</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;employee_id\&quot;: \&quot;${alpha_header_user_id_value}\&quot;,\n\t\&quot;leave_type_id\&quot;: 1,\n\t\&quot;message\&quot;: \&quot;Family Trip to Canada 2020\&quot;,\n\t\&quot;contact\&quot;: \&quot;081808180818\&quot;,\n    \&quot;is_draft\&quot;: false,\n    \&quot;company_setting_id\&quot; : 1,\n\t\&quot;leaves\&quot;: [\n\t\t{\n\t\t\t\&quot;start_date\&quot;: \&quot;2020-07-02\&quot;,\n\t\t\t\&quot;duration\&quot;: 0.5,\n\t\t\t\&quot;time_of_day\&quot;: \&quot;afternoon\&quot;\n\t\t},\n\t\t{\n\t\t\t\&quot;start_date\&quot;: \&quot;2020-07-03\&quot;,\n\t\t\t\&quot;duration\&quot;: 3,\n\t\t\t\&quot;time_of_day\&quot;: \&quot;full\&quot;\n\t\t}\n\t]\n}&quot;,
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
      <webElementGuid>095dafb6-7e71-4f6a-87ba-020fa678bb6e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>91baea46-84be-421d-aae3-317060aa0610</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/leaves/requests</restUrl>
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
      <id>4e10a190-4d11-438c-a3b8-14f8d8aa31aa</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>9dfff803-792f-44a8-9110-a4605c8f0969</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
