# Data Structures and Algorithms Specialization
Coursera / Edx

https://www.coursera.org/specializations/data-structures-algorithms

# A personal learning project going through the course using rust.

The Goal of this Project is for self learning the Data Structures and Algorithms Specialization Course  
as instructed on Coursera/EDx and to learn Rust from first principles using only the Standard Libray and  
rust compiler only.  
It also attempts to implement an automated grading tool  
(as an alternative to the course paid subscription grader).  
Contributions from tests , developing the grader tool and more test cases are most welcome.  


# Requirements 
Any Linux/ Unix like shell  
GNU utilities: pmap, awk, tail  
Rust 1.45+  


# Setup
cd <course directory>  
mkdir bin  
rustc grader.rs  
chmod +x grader  


## Programming Challenges 
Save your solution source files in "src" folder in Course folder.  
You encouraged to use the rust standard library only.  
The objective is to learn from first principles.  
Leave crates for real world production projects.  
Make use of the "stresser.rs" module to run stress tests in your solution.  

## Building and Compiling Solution
Use the provided build tools or scripts as much as possible.   
Do Not Use Cargo  
<img src="https://i.kym-cdn.com/entries/icons/original/000/026/366/pather.jpg" alt="alt text" width="252" height="141">  
1. To build only invoke this command in your shell. ./build.sh src/<solution>.rs  

2. To build and run  "./build_run.sh src/<solution>.rs  "  
## 1. Running Grader 
NB: Compile grader if running for the first time

2. ./grader src/<solution>.rs  
