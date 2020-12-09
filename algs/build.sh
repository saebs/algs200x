#!/bin/bash
#TODO SOME error handling when you have time
solutionSourceCode="$1"
# echo $solutionSourceCode
# trim rust extension
app=${solutionSourceCode%.rs}
# trim parent folder characters 'src/'
app=${app:4}

# Compile Solution
echo compiling ...
rustc -Ov $solutionSourceCode -o bin/$app 
# Run Build
# echo running ...
# bin/./$app
exit 1