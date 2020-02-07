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
| 5 | HLT | Halt |
| 6 | JMP | Jump |
| 7 | JMPF | Jump forward |
| 8 | JMPB | Jump backward |
| 9 | EQ | Equal |
| 10 | NEQ | Not equal |
| 11 | GTE | Greater then or equal to |
| 12 | GT | Greater then |
| 13 | LTE | Less then or equal |
| 14 | LT | Less then
| 15 | JMPE | Jump if equal |
| 16 | NOP |
| _ | IGL | Illegal action |