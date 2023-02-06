# Fast-Open-Library
Quick utilities for working with an open library
___

Introducing the Fast-Open-Library project on GitHub - a high-performance tool for downloading, cleaning,
and preparing data from open libraries written in Rust. With this tool, you can quickly retrieve vast
amounts of data from popular libraries, remove any irrelevant information, and get it ready for analysis
and modeling. The project leverages the power of Rust to ensure lightning-fast performance and efficient
memory utilization, making it the perfect solution for big data tasks. Whether you're working on a machine
learning project or simply need to clean and format data for further analysis, Fast-Open-Library has got you covered.
Try it out today and experience the difference that Rust-powered performance can make!

The open library file can be a challenge to work with due to inconsistent data format. 
Direct importing often leads to errors as the number of columns in the file varies. 
Cleaning up such a massive file manually is a daunting task. 
That's why we have created Fast-Open-Library, a Rust program designed to handle this task effortlessly.
FastDataPrep reads the CSV file and filters out rows with less than 5 columns, producing a cleaned
version of the data for easy import and analysis. Say goodbye to messy downloads and embrace
the power of rust with Fast-Open-Library.

### Getting started

`git clone https://github.com/Xaneets/Fast-Open-Library.git`

`cd Fast-Open-Library/`

If you want to download and clear the data immediately, then use start script
```
chmod +x start.sh

./start
```
Or do the same, but in manual mode. Note: the dump files must have a name `ol_dump_authors_latest.txt`, 
`ol_dump_works_latest.txt`, `ol_dump_editions_latest.txt`.
1. build project `cargo build --release`. Note: the `release` key is very important for execution speed
2. place the files next to the executable file (you can move the executable file to a directory with dumps 
`cp target/release/fast_open_library dir_path`)
3. and run the executable file `./fast_open_library`

Now you have a csv file ready to be imported into your database.