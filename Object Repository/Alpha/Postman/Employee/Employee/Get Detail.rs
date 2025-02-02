<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Detail</name>
   <tag></tag>
   <elementGuidId>4cf141c9-b66d-4d7a-969d-ce24dda0913d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${alpha_token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>02937655-95e0-45e3-ab82-6e678dff87fd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>21dddbc4-0fc1-439b-b9c5-254b092f4dba</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${alpha_token}</value>
      <webElementGuid>cef43afe-a4e4-4250-becd-38bb61da50c7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${alpha_gw_api_url}/employees/${alpha_header_user_id_value}</restUrl>
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
      <id>94dc55dc-5343-4b61-ae03-2a3b5e411052</id>
      <masked>false</masked>
      <name>alpha_gw_api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.alpha_header_user_id_value</defaultValue>
      <description></description>
      <id>eb35bc1b-1d3c-4779-a651-3bfbc4b33214</id>
      <masked>false</masked>
      <name>alpha_header_user_id_value</name>
   </variables>
   <variables>
      <defaultValue>'eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIyIiwianRpIjoiNzdkYjhkNjg1OGM1ZThhYWZiNzA1MTY3OTcyODM0MWQ4N2QxYWU3YTNiNTdjYWQxNTVhMGE0YmI4ZTc2M2E3YjFiZTY4MTA5ODliOTkzZTciLCJpYXQiOjE3Mzc1MjQ3NjQsIm5iZiI6MTczNzUyNDc2NCwiZXhwIjoxNzM4MTI5NTY0LCJzdWIiOiIyNDMiLCJzY29wZXMiOlsiKiJdfQ.yvHXXtSj07TxZ_ytbIL7-KN7mt_gtMpjrM_cQIRluy02woIGT0qt9TRS5Ue5EzDTpGh0GFyfY_smSD5jWVgMGGnSMMQA6uilPAwV1FFj_SIjH211uqDDS0NO6MYXPZ1lCg3FuWVuujhqJG0WfDZwhNXY5vsb8fs0Lr8qlQyRFjSEJrGW_gJtlV_EbrTjD3GniPvMCaHERfYLl5c9FHtryCotzXHdQPRum2mFJrCpgtVvJ5y62faYFBVvXvwTvGsnUBBH-6p3wbzKt3wU838YmZMndwU23XFkukhNjV6hYeEe4fNovnX46X_4V_O18TDsNzBdW3Eh8hGFilcFI04p7n52awevFgjerY36wVRyb7zY-keCJu2LSdxLW8_AuoST8yDTSUJmlxhb44F9-L9zvK7joBoa2xzIFhw_7eNnocsusUFaxpQZJF40XCp5Wvd2arDN34gQAlE804N1o3uwri0CmWAcE36tYSRl3iE8IKEhuNqCOp8FUOLvgXfsRRrjQP4coi7EivF7fQqDUbEUeFHXonKvV9Bs1H6qcxmpMEzVAgDV1KiLM5WCQhV35YzWg2PlYGr1xY1Q5mr5vSjh8PblAlJYEimNkCkEzuj8LmcxT1ZgxtDnNsbdMZAgk66lEj2bOswqA8VrJnFJEhZJb3zzQ5-LQMkow5O8XwZi_yM'</defaultValue>
      <description></description>
      <id>7b35fb66-6c76-497e-87ef-651aa3dffd92</id>
      <masked>false</masked>
      <name>alpha_token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
