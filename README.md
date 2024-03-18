# AU-ENERGY-SCRAPER
Australian energy data pipeline in rust. Inspired by [OpenNEM](https://opennem.org.au/energy/nem/?range=7d&interval=30m&view=discrete-time) ⚡ ♥

## Strategy

Rust is the right choice to handle strict typing with explicit errors. The issue is there are so many abbreviations and poor data structures there are lots of moving parts.

I'll start off looking at simple ones first for example [Current Rooftop PV Actual](http://nemweb.com.au/Reports/CURRENT/ROOFTOP_PV/ACTUAL/)

## NEMWEB Reports
Each report (zip archive) is uploaded every 30mins or 5mins. They shouldn't change and latest can be accessed via http url for example above.

## Zip Archive
Reports are contained in zip archives and most contain one csv file.

## CSV file 
Each csv contains an ordered identifier `C, I, D, C` in first column. Heres my best guess at what they mean:
   * C: "Comment" or "Control" for comment or a control line that provides metadata.
   * I: "Information" or "Instruction". Similar to C, but specific to data structure to come
   * D: "Data" containing the actual data entries.
   * C (again): Another C line at the end of the data might indicate the end of the report or dataset.

Below is a simple example of Rooftop PV Actual:

```title="example rooftop actual PV csv from archive" linenums="1"
C,NEMP.WORLD,ROOFTOP_PV_ACTUAL_MEASUREMENT,AEMO,PUBLIC,2024/03/03,20:00:03,0000000412707330,DEMAND,0000000412707330
I,ROOFTOP,ACTUAL,2,INTERVAL_DATETIME,REGIONID,POWER,QI,TYPE,LASTCHANGED
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",NSW1,0,1,MEASUREMENT,"2024/03/03 19:49:13"
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",QLD1,0,1,MEASUREMENT,"2024/03/03 19:49:13"
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",QLDC,0,1,MEASUREMENT,"2024/03/03 19:49:13"
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",QLDN,0,1,MEASUREMENT,"2024/03/03 19:49:14"
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",QLDS,0,1,MEASUREMENT,"2024/03/03 19:49:14"
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",SA1,6.617,1,MEASUREMENT,"2024/03/03 19:49:14"
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",TAS1,0,1,MEASUREMENT,"2024/03/03 19:49:14"
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",TASN,0,1,MEASUREMENT,"2024/03/03 19:49:14"
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",TASS,0,1,MEASUREMENT,"2024/03/03 19:49:14"
D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",VIC1,0,1,MEASUREMENT,"2024/03/03 19:49:14"
C,"END OF REPORT",13
```

Some CSV files contain multiple pairs of Information and Data for example:

```bash title="example of identifier column with multiple data sections"
C, # Control
I, # headers
D, # some data
D, # some more data
I, # new headers
D, # different data
C, # Control
```

## Parsing CSV file
Each Zip  