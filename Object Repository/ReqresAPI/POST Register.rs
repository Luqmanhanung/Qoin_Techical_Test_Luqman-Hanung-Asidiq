<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Request object for registering a user</description>
   <name>POST Register</name>
   <tag></tag>
   <elementGuidId>5d70d9e5-144c-4739-92bf-cd2cc833f5ce</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;${GlobalVariable.userEmail}\&quot;,\n  \&quot;password\&quot;: \&quot;password\&quot;\n}\n&quot;,
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
      <webElementGuid>4b7c21f1-d62d-405d-a16a-e660841ed22c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/register</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

// Verify the response code status
WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


// Verify that the response contains the token
WS.verifyElementPropertyValue(response, 'token', { value can be any non-empty string })

// Ensure that the token is not empty
def token = WS.getElementPropertyValue(response, 'token')
WS.verifyNotEqual(token, null)
WS.verifyNotEqual(token, '')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
