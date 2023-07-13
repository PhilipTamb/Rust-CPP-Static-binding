[dependencies]

libloading = "0.8"



[build-dependencies]

cc = "1.0"


g++ -dynamiclib -o libnum.dylib src/num.cpp

rustc -l num -L . src/main.rs
