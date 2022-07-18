<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit User By User Email</name>
   <tag></tag>
   <elementGuidId>4671eb30-599e-4831-8089-179cd72b8764</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;Danny Choi\&quot;,\n    \&quot;email\&quot;: \&quot;danny_choi@gmail.com\&quot;,\n    \&quot;phone_number\&quot;: \&quot;08123456789\&quot;,\n    \&quot;image_url\&quot;: \&quot;\&quot;,\n    \&quot;specialization_id\&quot;: 1,\n    \&quot;address\&quot;: {\n        \&quot;detail_address\&quot;: \&quot;671 Lynn Street\&quot;,\n        \&quot;country\&quot;: \&quot;United States of America\&quot;,\n        \&quot;state_province\&quot;: \&quot;Massachusetts\&quot;,\n        \&quot;city\&quot;: \&quot;Worcester\&quot;,\n        \&quot;zip_code\&quot;: \&quot;01610\&quot;\n    }\n}&quot;,
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
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${base_network}/api/users/edit-admin</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.base_network</defaultValue>
      <description></description>
      <id>13c93849-6d6a-4b9b-96e3-3fdbadd04b0c</id>
      <masked>false</masked>
      <name>base_network</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token_admin</defaultValue>
      <description></description>
      <id>c7bc27bd-d6e0-4187-a013-ede66e3442fa</id>
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
