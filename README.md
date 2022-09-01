# SAT_Recruting_TasK

* [General info](#general-info)
* [Technologies](#technologies)
* [Setup](#setup)


## General info
<details>
<summary>Click here to see general information about <b>Project</b>!</summary>
REST API to calculate the fuel usage and projecting the probability of the unit injector fail.
API was build with the help of the RUST framework Actix, which makes it run faster and you don't have to care about stringly typed objects since from request to response everything has types.
API implements following endpoints:
  GET: /calculateDisselUsageForDistance
  GET: /probabilityOfUnitInjectorFail

</details>

## Technologies
<ul>
<li>RUST</li>
<li>Actix</li>
</ul>

## Setup
<ul>
<li>Download and unzip the the files from this github repository</li>
<li>Open the "target" folder </li>
<li>Go to the "Debug" folder</li>
<li>RUN the .exe file inside</li>
<li>Open the command line on your computer</li>
<li>Type the given formule inside to see how the API is working, in the bracket section put any value that you want</li>
  
  
  `curl "localhost:8080/calculateDisselUsageForDistance?distance=[value]&yearOfProduction=[value]&fuelUsagePer100KM=[value]"`
  
  `curl "localhost:8080/probabilityOfUnitInjectorFail?VIN=[value]"`
  
  `curl "localhost:8080/calculateDisselUsageForDistance?distance=684&yearOfProduction=2007&fuelUsagePer100KM=9"` - EXAMPLE
  
  `curl "localhost:8080/probabilityOfUnitInjectorFail?VIN=123dfgdfhfgjf36345"` - EXAMPLE
</ul>
