<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Profile</name>
   <tag></tag>
   <elementGuidId>e8ffb3eb-ae3e-49a3-98d6-29504c605794</elementGuidId>
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
      &quot;name&quot;: &quot;fullName&quot;,
      &quot;value&quot;: &quot;Lulu Q.A.&quot;
    },
    {
      &quot;name&quot;: &quot;bio&quot;,
      &quot;value&quot;: &quot;student&quot;
    },
    {
      &quot;name&quot;: &quot;status&quot;,
      &quot;value&quot;: &quot;a lifelong learner&quot;
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
      <webElementGuid>cb31038d-c142-4943-a66e-3923e2bc93f8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt ${token}</value>
      <webElementGuid>207cadea-9244-40df-8f0d-4c093731e3b5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${url}/users/636a309d20de7c7a7f116719</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>0dd7d9a0-3a9b-478a-90a6-0ca022ff730a</id>
      <name>User-PATCH Update Profile Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Validation JSON/User/User-PATCH Update Profile.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>f0e52002-7f6b-4a0e-ba1d-fc208686a028</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>6c1baa93-d345-4383-aaef-aeedd33624ce</id>
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
