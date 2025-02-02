<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Leave Request</name>
   <tag></tag>
   <elementGuidId>eb26c865-eefa-4b04-8d1d-a3bd7e4b78a9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;leave_type_id\&quot;: 1,\n\t\&quot;message\&quot;: \&quot;Family Trip to Canada 2020\&quot;,\n\t\&quot;contact\&quot;: \&quot;081808180818\&quot;,\n\t\&quot;leaves\&quot;: [\n\t\t{\n\t\t\t\&quot;start_date\&quot;: \&quot;2020-07-02\&quot;,\n\t\t\t\&quot;duration\&quot;: 0.5,\n\t\t\t\&quot;time_of_day\&quot;: \&quot;afternoon\&quot;\n\t\t},\n\t\t{\n\t\t\t\&quot;start_date\&quot;: \&quot;2020-07-03\&quot;,\n\t\t\t\&quot;duration\&quot;: 3,\n\t\t\t\&quot;time_of_day\&quot;: \&quot;full\&quot;\n\t\t}\n\t]\n}&quot;,
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
      <webElementGuid>e20be280-e1a3-469a-a423-3cd48024677f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d7b3ac5d-57bd-4f2f-8e37-abe06f3bbeca</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/my/leaves/requests</restUrl>
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
      <id>e8b03a6a-09a2-4edb-8f41-2caa97680c61</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
