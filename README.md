# Command Line File Manager (Completed)
# For any feature requests please add an issue.

This CLI uses the native Windows terminal, built with visual studio and vs code. The terminal itself was developed with C#, with all underlying logic handled by Rust. C# passes input and recieves output from a dump via text file, controlled by Rust modules. 

This file manager provides more efficient and memory safe operations using Rust's strict type system, bundled with data structures like linked lists for caching.


# Tech Stack
* Rust
* C#
* Visual Studio
* Windows OS
* Docker


# Installation
1. Download the .NET 8.0 Runtime (x64) if you do not already have Visaul Studio downloaded. You do not need the entire .NET Framework just the runtime.
https://dotnet.microsoft.com/en-us/download/dotnet/thank-you/runtime-8.0.4-windows-x64-installer
2. `git clone https://github.com/ddswoosh/cli-file-manager`
3. Run the cli.exe file

# Features
* nd create directory) `nd test`
* dd (delete empty directory) `dd test` 
* odd (delete non-empty directory) `odd test` 
* nf (create file) `nf test python` 
* df (delete file) `df test.py` 
* open (open file in an editor) `open code test.py` 
* grab (store file/directory in an array) `grab test.py` 
* show (display file in the grab array) `show` 
* drop (drop file/directory from array) `drop` 
* mov (move file/directory) `mov test.py {or type holding to use grabbed file} new/test.py` 
* copy (copy file/directory) `copy test.py new/test.py` 
* cd (change directory) `cd test` or `cd back` to pop up a level
* list (display all items in current directory) `list`
* addext (display all items in current directory) `add javascript .js`
* added (display all items in current directory) `add vim ../vim.exe`
* cache (display 5 most recent nodes to be reverted) `cache`
* cache- (display 5 less recent nodes to be reverted) `cache-`
* cache+ (display 5 morerecent nodes to be reverted) `cache+`
* integer (stage a node for reversion) `1`
* revert (revert the operation stored in the staged node) `revert`
* c (clear screen) `c`
