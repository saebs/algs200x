#!/bin/bash
echo 1. aplusb
echo Enter Name of Programming Challenge
read -p 'solution: ' appName
testData=test_case.txt
timeLimit=1
memLimit=524288
cd grader_report 
rm *.txt
cd ..
rustc src/$appName.rs -o bin/$appName 
/usr/bin/time -f'\nUser(s)\tSystem(s)\tMemory(kb)\n%U\t%S\t\t%M\n' -o "grader_report//$appName.txt"  bin/$appName  
echo
echo CONSTRAINTS
echo Time Limit: $timeLimit s
echo Memory Limit $memLimit kb 
echo 
echo BENCHMARKS
cat grader_report/$appName.txt
# cat /dev/stdin | cut -d' ' -f 2,3 | sort