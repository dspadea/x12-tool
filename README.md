# x12-tool

A Tool for inspecting and converting X12 EDI

## Maturity

This tool is extremely simple right now, but does contain useful functionality that should generally work ok. 

## Installation

```shell
cargo install --git https://github.com/dspadea/x12-tool.git
```

## Usage

The `x12-tool` mostly focuses around "docless" mode right now. "Docless" mode does not attempt to parse into specific loops or document types. It is a very simple parsing of EDI segments in isolation for the purposes of making the document easier to read or convert to other simple formats like CSV or array-based JSON. 

The default mode will be a full parsing of the loops and structures which are supported by the `x12-types` library. This is in its infancy. It can currently parse some types of documents and output the parsed structure in Rust Debug format. This is experimental, and probably not very useful right now.


### Docless Parse and View in tabular mode

```
cat ~/projects/x12/x12-samples/005010X212\ Health\ Care\ Claim\ Status\ Request\ and\ Response/X212-276-provider-request.edi| x12-tool --docless --docless-output-mode=tabular --tabular-show-txn-sets

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/x12-tool --docless --docless-output-mode=tabular --tabular-show-txn-sets`
SEG   01       02         03                       04         05    06              07    08              09         10    11    12    13        14    15    16
----- -----    -----      -----                    -----      ----- -----           ----- -----           -----      ----- ----- ----- -----     ----- ----- -----
ISA   00                  00                                  ZZ    123456789012345 ZZ    123456789012346 080503     1705  >     00501 000010216 0     T     :
GS    HR       1234567890 1234567890               20080503   1705  20213           X     005010X212


----EDI 276 Transaction Set ----
ST    276      0001       005010X212
BHT   0010     13         ABC276XXX                20050915   1425
HL    1                   20                       1
NM1   PR       2          ABC INSURANCE                                                   PI              12345
HL    2        1          21                       1
NM1   41       2          XYZ SERVICE                                                     46              X67E
HL    3        2          19                       1
NM1   1P       2          HOME HOSPITAL                                                   XX              1666666661
HL    4        3          22                       0
DMG   D8       19301210   M
NM1   IL       1          SMITH                    FRED                                   MI              123456789A
TRN   1        ABCXYZ1
REF   BLT      111
REF   EJ       SM123456
AMT   T3       8513.88
DTP   472      RD8        20050831-20050906
HL    5        2          19                       1
NM1   1P       2          HOME HOSPITAL PHYSICIANS                                        XX              6166666666
HL    6        5          22                       1
NM1   IL       1          MANN                     JOHN                                   MI              345678901
HL    7        6          23
DMG   D8       19951101   M
NM1   QC       1          MANN                     JOSEPH
TRN   1        ABCXYZ3
REF   EJ       MA345678
SVC   HC:99203 150                                                                  1
DTP   472      D8         20050501
SE    28       0001
---- END EDI 276 Transaction Set (Declared 28 segments, found 28) ----


GE    1        20213
IEA   1        000010216
```

### Docless Parse and Output as CSV
```
cat ~/projects/x12/x12-samples/005010X212\ Health\ Care\ Claim\ Status\ Request\ and\ Response/X212-276-provider-request.edi| x12-tool --docless --docless-output-mode=csv

SEG,01,02,03,04,05,06,07,08,09,10,11,12,13,14,15,16
ISA,00,          ,00,          ,ZZ,123456789012345,ZZ,123456789012346,080503,1705,>,00501,000010216,0,T,:
GS,HR,1234567890,1234567890,20080503,1705,20213,X,005010X212,,,,,,,,
ST,276,0001,005010X212,,,,,,,,,,,,,
BHT,0010,13,ABC276XXX,20050915,1425,,,,,,,,,,,
HL,1,,20,1,,,,,,,,,,,,
NM1,PR,2,ABC INSURANCE,,,,,PI,12345,,,,,,,
HL,2,1,21,1,,,,,,,,,,,,
NM1,41,2,XYZ SERVICE,,,,,46,X67E,,,,,,,
HL,3,2,19,1,,,,,,,,,,,,
NM1,1P,2,HOME HOSPITAL,,,,,XX,1666666661,,,,,,,
HL,4,3,22,0,,,,,,,,,,,,
DMG,D8,19301210,M,,,,,,,,,,,,,
NM1,IL,1,SMITH,FRED,,,,MI,123456789A,,,,,,,
TRN,1,ABCXYZ1,,,,,,,,,,,,,,
REF,BLT,111,,,,,,,,,,,,,,
REF,EJ,SM123456,,,,,,,,,,,,,,
AMT,T3,8513.88,,,,,,,,,,,,,,
DTP,472,RD8,20050831-20050906,,,,,,,,,,,,,
HL,5,2,19,1,,,,,,,,,,,,
NM1,1P,2,HOME HOSPITAL PHYSICIANS,,,,,XX,6166666666,,,,,,,
HL,6,5,22,1,,,,,,,,,,,,
NM1,IL,1,MANN,JOHN,,,,MI,345678901,,,,,,,
HL,7,6,23,,,,,,,,,,,,,
DMG,D8,19951101,M,,,,,,,,,,,,,
NM1,QC,1,MANN,JOSEPH,,,,,,,,,,,,
TRN,1,ABCXYZ3,,,,,,,,,,,,,,
REF,EJ,MA345678,,,,,,,,,,,,,,
SVC,HC:99203,150,,,,,1,,,,,,,,,
DTP,472,D8,20050501,,,,,,,,,,,,,
SE,28,0001,,,,,,,,,,,,,,
GE,1,20213,,,,,,,,,,,,,,
IEA,1,000010216,,,,,,,,,,,,,,
```
### Docless Parse and Output as JSON

```
cat ~/projects/x12/x12-samples/005010X212\ Health\ Care\ Claim\ Status\ Request\ and\ Response/X212-276-provider-request.edi| x12-tool --docless --docless-output-mode=json
[
  [
    "ISA",
    "00",
    "          ",
    "00",
    "          ",
    "ZZ",
    "123456789012345",
    "ZZ",
    "123456789012346",
    "080503",
    "1705",
    ">",
    "00501",
    "000010216",
    "0",
    "T",
    ":"
  ],


.... snip ....

  [
    "GE",
    "1",
    "20213"
  ],
  [
    "IEA",
    "1",
    "000010216"
  ]
]
```