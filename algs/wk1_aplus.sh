#!/bin/bash
/usr/bin/time -f'program: %C \n user: %Us\n system: %Ss\n Elapsed: %Es\n Memory: %Mkb\n' -o perfomance_tests_logs/aplus.log cargo run --bin  week1 aplusb aplus_test_case.txt 
timeLimit=1
memLimit=524288
# parse determine if pas or fail speed test
echo Time Limit: 1s
echo Memory Limit 512mb
echo -----------------
echo Bechmarks
echo _________
cat perfomance_tests_logs/aplus.log