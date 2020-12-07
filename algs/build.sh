#!/bin/bash
#TODO SOME error handling when you have time
solutionSourceCode="$1"
echo $solutionSourceCode
# strip rust extension and parent folder
app=${solutionSourceCode%.rs}
app=${app:4}

# Compile Solution
echo compiling ...
rustc -O $solutionSourceCode -o bin/$app 
# Run Build
echo running ...
bin/./$app