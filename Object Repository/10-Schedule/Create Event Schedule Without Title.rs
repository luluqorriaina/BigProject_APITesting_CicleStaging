<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Event Schedule Without Title</name>
   <tag></tag>
   <elementGuidId>1c3e04e5-9d96-4a26-ba49-afb1cdc3b227</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;title\&quot;: \&quot;\&quot;,\n    \&quot;content\&quot;: \&quot;\u003cp\u003eEvent Schedule Without Title\u003c/p\u003e\&quot;,\n    \&quot;startDate\&quot;: \&quot;2022-12-02T15:43:20.446Z\&quot;,\n    \&quot;endDate\&quot;: \&quot;2022-12-02T15:43:20.446Z\&quot;,\n    \&quot;subscribers\&quot;: [],\n    \&quot;mentionedUsers\&quot;: []\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5fcdf3e5-6514-4894-9b4a-734f8c38c556</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt ${token}</value>
      <webElementGuid>97602605-c907-4b05-928c-f7882b5ff12e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/schedules/6373b898fef0c6ab2e29fb1a/events</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>8243c784-c592-4228-9454-5ac03658c5c3</id>
      <name>Schedule-POST Create Event Schedule Without Title Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Validation JSON/Schedule/Schedule-POST Create Event Schedule Without Title.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>738668a8-1f7c-413b-9f5d-6aa60413d8d6</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>b7d493d9-362c-49d1-8d65-091baa9f6608</id>
      <masked>false</masked>
      <name>token</name>
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
