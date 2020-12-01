#!/bin/bash
#TODO SOME error handling when you have time
solutionSourceCode="$1"
echo $solutionSourceCode
timeLimit=1
memLimit=524288
cd report 
rm *.txt
cd ..
# strip rust extension and parent folder
app=${solutionSourceCode%.rs}
app=${app:4}

# Compile Solution
rustc $solutionSourceCode -o bin/$app 
# Measure Runtime 
/usr/bin/time -f'\nUser(s)\tSystem(s)\tMemory(kb)\n%U\t%S\t\t%M\n' -o "report/$app.txt"  bin/$app  
echo
echo CONSTRAINTS
echo Time Limit: $timeLimit s
echo Memory Limit $memLimit kb 
echo 
echo BENCHMARKS
cat report/$app.txt