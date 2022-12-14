<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Team Without Description</name>
   <tag></tag>
   <elementGuidId>20919b38-c88d-4f0b-8bba-e87ae69988e4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;data\&quot;: {\n        \&quot;desc\&quot;: \&quot;\&quot;,\n        \&quot;name\&quot;: \&quot;Create Team Without Description\&quot;,\n        \&quot;type\&quot;: \&quot;team\&quot;\n    },\n    \&quot;selector\&quot;: {\n        \&quot;companyId\&quot;: \&quot;61eba2c85080f47b25ddc8f8\&quot;\n    }\n}&quot;,
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
      <webElementGuid>105b1942-cce4-4a23-912f-654cbc54ca84</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt ${token}</value>
      <webElementGuid>06e635c3-1c32-4197-90a5-d4fe36ff44b1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/teams</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>9c95bca8-3108-4326-9c99-cd32fa2ae215</id>
      <name>Team-POST Create Team Without Description Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Validation JSON/Team/Team-POST Create Team Without Description.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>39c1638f-a3c1-4a96-94a2-26d8c158e7e5</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>f72c4565-db3b-4efa-938e-bceb19e8239f</id>
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
