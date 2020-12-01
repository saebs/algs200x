#!/bin/bash
echo Enter Name of Programming Challenge
read -p 'solution: ' appName
testData=test_case.txt
timeLimit=1
memLimit=524288
cd perfomance_tests_logs
rm *.log
cd ..

# /usr/bin/time -f'\nUser\tSystem\tReal\tMemory\n%Us\t%Ss\t%es\t%Mkb\n' -o "perfomance_tests_logs//$appName.log" cargo run --bin $appName $testData
/usr/bin/time -f'\nUser(s)\tSystem(s)\tMemory(kb)\n%U\t%S\t\t%M\n' -o "perfomance_tests_logs//$appName.log" cargo run --bin $appName $testData
echo
echo CONSTRAINTS
echo Time Limit: $timeLimit s
echo Memory Limit $memLimit kb 
echo 
echo BENCHMARKS
cat perfomance_tests_logs/$appName.log