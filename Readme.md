# Chat commands Parser

The theme of the parcer is a command like input in the enviroment like chats of different social apps (Discord, Telegram)
In current state the program parses the text that can be located in the text file for example input.txt and as a result it returns a file output.txt that has the modified text

List of commands that can be parced and apply an effect on the program:

/smile -> return a ":) "

/repeat number "text_in_quotes" -> returns a phrase in the quotes repeatedly number of times

/define "text_in_quotes1" "text_in_quotes" -> return nothing creates a variable "text_in_quotes1" that can be used by other command /use

/use "text_in_quotes" -> returns a defined before value that if it matches any of variable ("text_in_quotes")

Example Input:
The world is beautiful /smile I like it /repeat 100 ":" 
Hello /define "user" "Bob" /use "user"!
 /use "user"
 /use "user"
  /use "user"
   /use "user"

Example Result Output:
The world is beautiful :) I like it ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
Hello Bob!
 Bob
 Bob
  Bob
   Bob

Logic of parsing:

![alt text](https://github.com/Kasgor/chat_commands_parcer/blob/master/file.png?raw=true)

This application is a parser built with Rust using the `pest`. It takes command-line input, parses it and displays structured information about the input. 

https://crates.io/crates/chat_commands_parcer

