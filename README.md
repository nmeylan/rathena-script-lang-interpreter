This project is work in progress, it will probably never be finished or never work.

# rAthena script language interpreter
A parser, compiler and VM implementation for rAthena script language.

Inspired by [https://github.com/Doddler/Scripting-Language-Guide](https://github.com/Doddler/Scripting-Language-Guide) and [https://craftinginterpreters.com/](https://craftinginterpreters.com/).

VM architecture inspired by [jvm](https://docs.oracle.com/javase/specs/jvms/se11/html/index.html)

# Generate Parser
`java -jar bin/antlr4.9.4-rust.jar RathenaScriptLang.g4 -Dlanguage=Rust -visitor -o src/parser`

# Goal
My goal is to make [warper script](https://github.com/rathena/rathena/blob/master/npc/custom/warper.txt) working in my [custom emulator](https://github.com/nmeylan/rust-ro).

# Grammar
Grammar is defined in `RathenaScriptLang.g4` file. Parser is generated using `antlr`.

# Architecture
## VM Memory layout
![](doc/vm%20architecture.png)

## VM lifecycle
```mermaid
graph TD
  Server[RagnarokServer] --> Boot --> Load[Load bytecode] --> VM;
    Compiler --> ReadTxt[Parse source] --> AST[Generate AST] --> Visit[Visit ast nodes] --> Chunk[Generate chunks] --> ByteCode[Write bytes code] --> Bytecode;
    VM[VM] --> Startup;
    Server --> Player[Player interact with NPC] --> Execute;
    Execute[Execute script] --> VM[VM] --> Instantiate[Instantiate script] --> Read2[Read byte code] --> Run[Execute program] --> I2[Execute op code];
    Startup[Bootstrap] --> Loading --> Linking --> Initialization
 

```
