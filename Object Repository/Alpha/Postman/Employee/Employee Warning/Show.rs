<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Show</name>
   <tag></tag>
   <elementGuidId>81a3866b-9f6f-479a-8e66-ad9a3d7c8fdb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;name\&quot; : \&quot;SP1\&quot;,\r\n    \&quot;description\&quot; : \&quot;Description Warning\&quot;,\r\n    \&quot;status\&quot; : 1\r\n}&quot;,
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
      <webElementGuid>d4efed86-e6af-41ed-a3a1-829514a75d25</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>cc18766e-f6f8-4cd8-a6ed-248248291782</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Proxy-Authorization</name>
      <type>Main</type>
      <value>${SECRET_SERVICE}</value>
      <webElementGuid>ddfbb5fe-02a0-48d3-a70c-a4923696bb10</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>${alpha_header_user_id_key}</name>
      <type>Main</type>
      <value>${alpha_header_user_id_value}</value>
      <webElementGuid>7ea922fe-281d-4676-8f3b-0c8172eac2a3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}/warning/${warning_type_id}</restUrl>
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
      <id>893700c4-00a4-4746-b937-0f8b5155f2ff</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>8e2a8814-14a5-4393-b90b-aaa7a4f42078</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.warning_type_id</defaultValue>
      <description></description>
      <id>7d07fa74-5bb0-4313-ab8b-6bef6587572f</id>
      <masked>false</masked>
      <name>warning_type_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SECRET_SERVICE</defaultValue>
      <description></description>
      <id>3515d1fd-3b03-43a5-bbfc-aa48e514381a</id>
      <masked>false</masked>
      <name>SECRET_SERVICE</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_key</defaultValue>
      <description></description>
      <id>49685018-533c-408e-bef3-ea150b744717</id>
      <masked>false</masked>
      <name>alpha_header_user_id_key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
