import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import internal.GlobalVariable

def response = WS.sendRequest(findTestObject('GET_CurrentAirPollution_JakartaSelatan'))

// Assert status code
WS.verifyResponseStatusCode(response, 200)

def json = new JsonSlurper().parseText(response.getResponseBodyContent())

// Assert list presence
assert json.list.size() > 0

def airData = json.list[0]

// Assert AQI and components
assert airData.main.aqi in 1..5
assert airData.components.co instanceof Number
assert airData.components.pm2_5 instanceof Number
