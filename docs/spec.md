# Specifications

## Iridium

Corten is based on Fletcher Haynes's [So you want to build a language VM](https://blog.subnetzero.io/post/building-language-vm-part-01/) tutorial. His virtual machine used for the tutorial is known as [Iridium](https://github.com/fhaynes/iridium). Despite it's origins, it does aim to be full a fledged virtual machine and is already on it's [third iteration](https://gitlab.com/fletchercp/iridium3) with support for SSH, PIDs and Strings. Corten aims to be Iridium 1-compatible.

## Instruction Set 

| Register | Opcode |
| --- | --- |
| 0 | LOAD |
| 1 | ADD |
| 2 | SUB |
| 3 | MUL |
| 4 | DIV |
| 6 | HLT |
| 5 | JMP |
| 8 | JMPF |
| 9 | JMPB |
| _ | IGL |