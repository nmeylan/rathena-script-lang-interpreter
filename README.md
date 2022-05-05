This project is work in progress, it will probably never be finished or never work.

# rAthena script language interpreter
An interpreter implementation for rAthena script language.

Inspired by [https://github.com/Doddler/Scripting-Language-Guide](https://github.com/Doddler/Scripting-Language-Guide)

# Generate Parser
`java -jar bin/antlr-rust.jar RathenaScriptLang.g4 -Dlanguage=Rust -visitor -o src/parser`

# Goal
My goal is to make [warper script](https://github.com/rathena/rathena/blob/master/npc/custom/warper.txt) working in my [custom emulator](https://github.com/nmeylan/rust-ro).