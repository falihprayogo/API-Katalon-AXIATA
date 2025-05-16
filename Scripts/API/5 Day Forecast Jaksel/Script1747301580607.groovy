import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import internal.GlobalVariable

def response = WS.sendRequest(findTestObject('GET_5DayForecast_JakartaSelatan'))

// Assert status code
WS.verifyResponseStatusCode(response, 200)

// Parse JSON
def json = new JsonSlurper().parseText(response.getResponseBodyContent())

// Assert body content
assert json.city.name == "Jakarta" || json.city.name == "Rawa Barat"
assert json.list.size() > 0

// Sample schema check
assert json.list[0].main.temp instanceof Number
assert json.list[0].dt_txt instanceof String
