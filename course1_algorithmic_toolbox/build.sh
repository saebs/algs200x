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
rustc --crate-type=rlib "utils.rs" --edition 2018
rustc -Ov $solutionSourceCode -o bin/$app --extern crate=libutils.rlib -L "." --edition 2018 
echo finished ...
# Run Build
# echo running ...
# bin/./$app
exit 1