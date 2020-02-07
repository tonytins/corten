# Specifications

## Iridium

Corten is based on Fletcher Haynes's [So you want to build a language VM](https://blog.subnetzero.io/post/building-language-vm-part-01/) tutorial. His virtual machine used for the tutorial is known as [Iridium](https://github.com/fhaynes/iridium). Despite it's origins, it does aim to be full a fledged virtual machine and is already on it's [third iteration](https://gitlab.com/fletchercp/iridium3) with support for SSH, PIDs and Strings. Corten aims to be Iridium 1-compatible.

## Instruction Set 

| Opcode | Function | Comment |
| --- | --- | --- |
| 0 | LOAD | Load program |
| 1 | ADD |
| 2 | SUB |
| 3 | MUL |
| 4 | DIV |
| 6 | HLT | Halt |
| 5 | JMP | Jump |
| 8 | JMPF | Jump forward |
| 9 | JMPB | Jump backward |
| 10 | EQ | Equal |
| 11 | NEQ | Not equal |
| 12 | GTE | Greater then or equal to |
| 13 | GT | Greater then |
| 14 | LTE | Less then or equal |
| 15 | LT | Less then
| 16 | JMPE | Jump if equal |
| _ | IGL | Illegal action |