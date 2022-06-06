**WIP** [Check remaining todos](https://github.com/nmeylan/rathena-script-lang-interpreter/issues/1) or [code samples](https://github.com/nmeylan/rathena-script-lang-interpreter/tree/master/tests)

# rAthena script language interpreter
A grammar, parser, compiler and VM implementation for rAthena script language.

Inspired by [https://github.com/Doddler/Scripting-Language-Guide](https://github.com/Doddler/Scripting-Language-Guide) and [https://craftinginterpreters.com/](https://craftinginterpreters.com/).

VM architecture inspired by [jvm](https://docs.oracle.com/javase/specs/jvms/se11/html/index.html)

This crate offers a compiler and VM to be embedded in game server.

# Generate Parser
`java -jar bin/antlr4.9.4-rust.jar RathenaScriptLang.g4 -Dlanguage=Rust -visitor -o src/parser`

# Grammar
Grammar is defined in `RathenaScriptLang.g4` file. Parser is generated using `antlr`.

# Integration with a server
Currently only integretation for a rust server implementation is targeted. A C/C++ API can be provided to integrate it in [herculesWS](https://github.com/HerculesWS/Hercules) and [rathena](https://github.com/rathena/rathena)

Vm should be run inside mmo server. NPC script can be pre-compiled or compiled at VM startup, using this crate.

Rathena script command related to game, almost all commands listed [here](https://github.com/rathena/rathena/blob/master/doc/script_commands.txt#L1012), have to be implemented in server side. 

Compiler considers those command as "native" functions. VM require to implement a native functions handler, which has to be implemented by the server.

# Architecture
## VM Memory layout
```mermaid
flowchart LR
    subgraph S1[<b>VM</b>]
        direction TB
        A1["ConstantPool<hr><ul><li><i>String (#quot;hello world#quot;)</i></li><li><i>Number(1)</i></li></ul>"]
        A2[NativeMethodPool<hr><ul><li><i>print</i></li><li><i>callfunc</i></li><li>...</li></ul>]
        A3[Heap<hr><ul><li><i>Arrays</i></li><li><i>Instances</i></li></ul>]
        A4[ClassPool<hr><i>Classes</i>]
        A5[InstancePool*<hr><i>Instances</i>]
        A3 --> A1
        A3 --> A5
    end

    subgraph S3[<b>Callframe</b>]
        direction TB
        C1[Operand Stack]
        C2["Local variable pool<hr><ul><li><i>String pointer</i></li><li><i>Number pointer</i></li></ul>"]
        C2 --> A1
    end

    subgraph S2[<b>Thread</b>]
        direction TB
        B1[Stack<hr><ul><li><i>References</i></li></ul>]
        B1 --> A1
        B1 --> A2
        B1 --> A3
        B1 --> C2
    end
```
*InstancePool does not actually exist as instance are destroyed once main function has been executed*
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
