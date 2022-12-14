<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Card</name>
   <tag></tag>
   <elementGuidId>de061469-2b99-404d-802f-81648498617b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;Update Card from API\&quot;,\n    \&quot;workload\&quot;: 4,\n    \&quot;startDate\&quot;: \&quot;2021-09-13T17:00:00.000Z\&quot;,\n    \&quot;dueDate\&quot;: \&quot;2021-09-16T17:00:00.000Z\&quot;,\n    \&quot;isNotified\&quot;: {\n        \&quot;startOneDay\&quot;: false,\n        \&quot;startOneHour\&quot;: false,\n        \&quot;dueOneDay\&quot;: false,\n        \&quot;dueOneHour\&quot;: false\n    }\n}&quot;,
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
      <webElementGuid>460f95ab-335e-4966-8ce8-042df2e070f4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt ${token}</value>
      <webElementGuid>0c83f977-dbfa-4342-86ec-b4adcc9e6007</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${url}/cards/6374a269fef0c6ab2e2d68a4</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>b5cd5927-838f-43a9-ae97-cce72b84aa81</id>
      <name>Card-PATCH Update Card Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Validation JSON/Card/Card-PATCH Update Card.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>3739c7e7-f3fb-488a-bab6-1a43fc548fce</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>75ed1875-2800-4ef8-be8a-1c41817f57f2</id>
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
