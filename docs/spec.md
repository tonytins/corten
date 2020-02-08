# Specifications

## Iridium

Corten [MIPS64 Release 6](https://en.wikipedia.org/wiki/MIPS_architecture#MIPS32/MIPS64_Release_6) virtual machine based on Fletcher Haynes's [So you want to build a language VM](https://blog.subnetzero.io/post/building-language-vm-part-01/) tutorial.

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
| 13 | LTE | Less then or equal to |
| 14 | LT | Less then |
| 15 | JMPE | Jump if equal |
| 16 | NOP |
| _ | IGL | Illegal action |