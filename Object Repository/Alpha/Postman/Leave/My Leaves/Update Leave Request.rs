<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Leave Request</name>
   <tag></tag>
   <elementGuidId>fe3a5ebe-e790-444e-b05d-0cbc9d12de4b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;leave_type_id\&quot;: 1,\n\t\&quot;message\&quot;: \&quot;Family Trip to Canada 2020XX\&quot;,\n\t\&quot;contact\&quot;: \&quot;081808180819\&quot;,\n\t\&quot;leaves\&quot;: [\n\t\t{\n\t\t\t\&quot;start_date\&quot;: \&quot;2020-07-02\&quot;,\n\t\t\t\&quot;duration\&quot;: 0.5,\n\t\t\t\&quot;time_of_day\&quot;: \&quot;afternoon\&quot;\n\t\t},\n\t\t{\n\t\t\t\&quot;start_date\&quot;: \&quot;2020-07-03\&quot;,\n\t\t\t\&quot;duration\&quot;: 3,\n\t\t\t\&quot;time_of_day\&quot;: \&quot;full\&quot;\n\t\t}\n\t]\n}&quot;,
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
      <webElementGuid>6c938ed3-3527-4459-9ae9-79a22c7cab0a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0b2ad81f-2ec5-4c6e-adb9-e53f6294895b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/my/leaves/requests/1</restUrl>
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
      <id>5ff8c398-d912-4676-b226-291308af0137</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
