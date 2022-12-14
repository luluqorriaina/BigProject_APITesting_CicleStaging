<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Checklist</name>
   <tag></tag>
   <elementGuidId>6d208442-868a-4803-9e80-98f8caf26c85</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;name&quot;,
      &quot;value&quot;: &quot;Child Task 2&quot;
    },
    {
      &quot;name&quot;: &quot;startDate&quot;,
      &quot;value&quot;: &quot;2022-07-20T20:58:18+07:00&quot;
    },
    {
      &quot;name&quot;: &quot;dueDate&quot;,
      &quot;value&quot;: &quot;2022-07-25T20:58:18+07:00&quot;
    },
    {
      &quot;name&quot;: &quot;isComplete&quot;,
      &quot;value&quot;: &quot;false&quot;
    },
    {
      &quot;name&quot;: &quot;dueDateType&quot;,
      &quot;value&quot;: &quot;range&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
      <webElementGuid>da6acb9e-1c13-4960-88db-21485267b2d7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt ${token}</value>
      <webElementGuid>557cde0e-5264-4bc6-bea4-7cf16e7f36a4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${url}/cards/6374a269fef0c6ab2e2d68a4/checklists/6375276b66ccc7048fbc2646</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>5c81ffab-636f-4fc1-9c74-29583586ef2b</id>
      <name>CC-Update Checklist Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Validation JSON/Card Checklist/CC-PUT Update Checklist.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>627f505e-7885-4b28-9832-7d9e61c9116e</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>89e58ef2-24ae-4301-8ac8-394802ff8aec</id>
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
