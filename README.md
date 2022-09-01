# SAT_Recruting_TasK

* [General info])(#general-info)
* [Technologies](#technologies)
* [Setup](#setup)


## General info
<details>
<summary>Click here to see general information about <b>Project</b>!</summary>
REST API to calculate the fuel usage and projecting the probability of the unit injector fail
</details>

## Technologies
<ul>
<li>RUST</li>
<li>Actix</li>
</ul>

## Setup
<ul>
<li>Download and unzip the the files from this githun repository</li>
<li>Click on the "target" folder </li>
<li>Go to the "Debug" folder</li>
<li>Open the .exe file inside</li>
<li>Open the command line onm your computer</li>
<li>Change directory using the cd command to the direcory where you have download the whole package</li>
<li>Type the given formule inside to see how the API is working, in the bracket section put any value that you want</li>
  curl "localhost:8080/calculateDisselUsageForDistance?distance=[value]&yearOfProduction=[value]&fuelUsagePer100KM=[value]"
  
  curl "localhost:8080/probabilityOfUnitInjectorFail?VIN=[value]"
</ul>
