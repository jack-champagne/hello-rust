# My journey into rust!

This is my first steps at taking a good look at a language that has proved to be very well-liked amoung its developers. This readme will be periodically updated to include information on what I have learned and completed and where I am headed next.


Most of this code is taken from the rust-lang documentation book (if you have rust installed, it can be accessed at ```$ rustup docs --book```).


To use this project, clone this repository and move into the directory that you would like to build the chapters code in. For directories without a Config.toml, use rustc to compile.


### Log
\- 8/13/2020 11:48am -

Just finished chapter one and am looking to see more of the language (not looking to write a project right now). I am going to skip on to chapter three before I get my hands dirty. So far, I really like the built in documentation and book as well as their package management system/build system and direct level rustc compiler. It is greatly convienent and refreshing for some one who has struggle with ruby gems and the ruby development toolchain in the past on my Windows machine to see something so plug and play.

\- 8/13/2020 1:13pm -

Just finished chapter 3 and theres a lot of cool functionality I like about this language. The fluid implementation of expression evalutions is quite nice and I enjoy the function header format as well as data type default immutability and assigments. if let is nice and the loops look solid, something that I really enjoy in python in a much faster language. Moving back to chapter 2 to do the project next and then doing the mini-projects at the end of chapter 3.

\- 8/13/2020 3:27pm -

It is after my lunch break and I have just finished chapter 2. Was a good chapter that allowed me to utilize some concepts and knowledge from chpt 1 and 3. The program written was quite simple but I loved the structure of error catching and the match keyword. Also, cargo and its ability for crate managment is so powerful, I enjoy something that controls the programming environment in such a positive way. On to chapter 4.

\- 9/5/2020 2:06pm -

It is during Cody's wedding in Alabama and I am doing a little bit of programming and homework before I have to start getting anything ready for the ceremony tonight. Slices are cool, though they did take a while for me to get through. I have not worked with this sort of concept before but really like to see how it avoids common bugs through clever language specifications. It is also extremely powerful. I have now finished chapter 4 and the next chapter will be about structs. This chapter was something that is deeply important to how rust function as a language and I found it quite impressive. I really like how it combines the best aspects of garabage-collected languages and explicit languages while eliminating the weaker aspects of both. References are something that I am going to be getting more and more comfortable with as time goes on but alas, on to chapter 5.

\- 11/24/2020 4:52pm - 

It's been a long time but progress has changed course for the worse because of how school has been taking up so much of my life. Trying to cut back on the youtube again and I think I will be making sure progress through my youtube bot, once I polish it up a little further. I  have done a lot of work on it recently and I think it will be paying off soon. Anyways, the chapter on structs took me forever (obviously) to get through but now that it is over, Rust's implementation of object oriented programming is very enjoyable. It combines elements of scripting languages, object oriented languages, and low-level implementations of grouping data together (like c's structs) in a way that is clever and maintains elegance. I can see great flexibility and clarity from the syntax and like the style of how it is laid out for a programmer, especially a learning programmer. I think soon, after I learn a little more syntax from this book, I will be laying out the foundations of many critical data-structures and algorithms from CS187 into Rust for implementations sake, and maybe to do a speed comparison (Java bytecode stands no chance against Rust's compiler I believe)

\- 11/25/2020 1:02am -

Today was a really great day of programming for me. When I don't have any project, assignments, and work to think about, I really do enjoy programming for the fun of it. Learning a new language and exploring syntax and design is something that takes a good amount of energy but I do find that the Book makes learning aspects of Rust seem like this is *the* right way to do this. Enumerators, matching, and it let have been designed in a way to ensure code safety and maximize utility. I really enjoy Rust's implementation of this and it seems extremely intuitive and syntactilly consistent with the rest of the language. Continuing on to Chapter 7 next possibly??? We will see.

\- 11/26/2020 2:38pm

Finally finished, after a lot of work, chapter 7 of the Book. It was mainly focused on how rust code is organized, maintained, and how modules, packages, and crates all work together and what they are. The chapter was pretty good at explaining many core concepts and idioms of the Rust programming language but before I move on, I think I will probably review my knowledge of what a *crate* actually is because I cannot define it myself at the moment. I enjoyed this chapter as well and like the structure and layout of rusts privacy and code organization system. This chapter is a little different from the rest because it focused a lot less on specific code syntax and was more focused on higher level organization and privacy concepts. That is why you will see there is a different structure to this chapter. Onto collections!

\- 1/26/2021 3:17pm

Chapter 8 took a lot of work. As we go further and futher into this book, I believe that the concepts will become more advanced and it will consume more time to make the same amount of progress (chapter number-wise) but this is expected! And most likely a design decision of The Book. The chapter was a lot about data structures and what rust does and doesn't do. Rust's handling of strings is deeply essential and reminded me of an article I ready a while ago that was about the complexity of strings and unicode/UTF-8. Basically some characters we use less bytes to represent and because of such, there is a lot of interesting things with finding where some characters start and other characters end. (An interesting note, Rust's std does not provide any API for interacting with Graphemes themselves so we have to make some important decisions or use a lovely crate). I would also like to point out that borrowing isn't too too bad but I wish I have read at least *something* on dereferecing. The exercises at the end of the chapter do require you to do some dereferencing. Something that I like a lot: Most of my code works on the first successful build which is nice.

\- 3/19/2021 4:08pm

Chapter 9 was about panic!, Result, and how Rust handles error. Also learned about .expect and .unwrap stuff. There was a small section on closures but we were not formally introduced to them. I also found the 'Rust by Example' book on their website and will probably be transition over to that a bit. This book gets into some details with rust and its design but I learn best by doing and I think that other book will provide that for me.

\- 5/13/2021 1:36pm

Chapter 10 was all about generics, traits, and references (reference lifetimes to be specific). Something that is not immediately clear in the beginning of the book as how Rust actually avoids dangling pointers and the sort. This is very clearly explained in this chapter during reference lifetimes. I am very excited for the next chapter however, as it is a skill that I need much development in. Testing!

