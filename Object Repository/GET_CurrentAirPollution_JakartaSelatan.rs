<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_CurrentAirPollution_JakartaSelatan</name>
   <tag></tag>
   <elementGuidId>075d4e70-cb80-4e0c-81f4-e995d0531d43</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;coord\&quot;: {\n      \&quot;lon\&quot;: 7.367,\n      \&quot;lat\&quot;: 45.133\n   },\n   \&quot;weather\&quot;: [\n      {\n         \&quot;id\&quot;: 501,\n         \&quot;main\&quot;: \&quot;Rain\&quot;,\n         \&quot;description\&quot;: \&quot;moderate rain\&quot;,\n         \&quot;icon\&quot;: \&quot;10d\&quot;\n      }\n   ],\n   \&quot;base\&quot;: \&quot;stations\&quot;,\n   \&quot;main\&quot;: {\n      \&quot;temp\&quot;: 284.2,\n      \&quot;feels_like\&quot;: 282.93,\n      \&quot;temp_min\&quot;: 283.06,\n      \&quot;temp_max\&quot;: 286.82,\n      \&quot;pressure\&quot;: 1021,\n      \&quot;humidity\&quot;: 60,\n      \&quot;sea_level\&quot;: 1021,\n      \&quot;grnd_level\&quot;: 910\n   },\n   \&quot;visibility\&quot;: 10000,\n   \&quot;wind\&quot;: {\n      \&quot;speed\&quot;: 4.09,\n      \&quot;deg\&quot;: 121,\n      \&quot;gust\&quot;: 3.47\n   },\n   \&quot;rain\&quot;: {\n      \&quot;1h\&quot;: 2.73\n   },\n   \&quot;clouds\&quot;: {\n      \&quot;all\&quot;: 83\n   },\n   \&quot;dt\&quot;: 1726660758,\n   \&quot;sys\&quot;: {\n      \&quot;type\&quot;: 1,\n      \&quot;id\&quot;: 6736,\n      \&quot;country\&quot;: \&quot;IT\&quot;,\n      \&quot;sunrise\&quot;: 1726636384,\n      \&quot;sunset\&quot;: 1726680975\n   },\n   \&quot;timezone\&quot;: 7200,\n   \&quot;id\&quot;: 3165523,\n   \&quot;name\&quot;: \&quot;Jakarta Selatan\&quot;,\n   \&quot;cod\&quot;: 200\n}                  &quot;,
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
      <webElementGuid>fc29b512-38de-416f-9644-14a07f24d5ef</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.openweathermap.org/data/2.5/air_pollution?lat=-6.2615&amp;lon=106.8106&amp;appid=${API_KEY}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'aa6bce23533b364aac710b69827b904c'</defaultValue>
      <description></description>
      <id>a5584d88-35b4-459a-950a-0a1193fd358f</id>
      <masked>false</masked>
      <name>API_KEY</name>
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
