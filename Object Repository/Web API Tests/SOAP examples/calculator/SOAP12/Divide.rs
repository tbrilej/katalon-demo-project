<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Divide</name>
   <tag></tag>
   <elementGuidId>6ca35cbb-874a-4bd4-9252-222af6cc9cc5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>&lt;SOAP-ENV:Envelope xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tns=&quot;http://tempuri.org/&quot;>
  &lt;SOAP-ENV:Header/>
  &lt;SOAP-ENV:Body>
    &lt;tns:Divide>
      &lt;tns:intA>1&lt;/tns:intA>
      &lt;tns:intB>2&lt;/tns:intB>
    &lt;/tns:Divide>
  &lt;/SOAP-ENV:Body>
&lt;/SOAP-ENV:Envelope>
</soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP12</soapRequestMethod>
   <soapServiceFunction>Divide</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress>http://www.dneonline.com/calculator.asmx?WSDL</wsdlAddress>
</WebServiceRequestEntity>
