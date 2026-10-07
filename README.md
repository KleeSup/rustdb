# rustdb
A very very simple Redis-like cached in-memory database. This project's intent is not to be used really anywhere but is rather an idea I had when I decided that I wanted to learn Rust. 

## Planned features / ToDo's
I want to expand this project with stuff I am interested in so I can learn how to do it in Rust. This would include:
- [x] Rust Enums: Learning the most powerful part of this language (or so I've been told).
- [x] Error Handling: Using Rust error types (which differ heavily to C++ as there is no `nullptr` type) and optionals to an useful extend.
- [ ] Networking: I'd like this database to have a client-server model using TCP.
- [ ] Data storage: I want to learn ways of storing data in Rust. I could imagine adding a `DUMP <filename> <type>` command that puts all the data in the cache into a file (Json, XML, Yaml etc.).
- [ ] Multithreading and Concurrency: I want a server handling multiple clients while synchronizing data accesses. This is probably a cool thing to do in Rust.   

## DISCLAIMER
> Code might not be perfect or optimized. This is my first project in Rust and I try to learn without using AI. I want to do it "the old way" by reading docs, comparing Rust code to my very well known languages (primarily Java/Kotlin, Python and C++) and just doing try-and-error debugging until I am somewhat comfortable with starting a real/bigger project. 

> This project is therefore also a way for me to relief the spirit of what made programming enjoyable for me when I started learning to code in 2018 :D
