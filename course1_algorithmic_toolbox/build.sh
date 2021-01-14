#!/bin/bash
#TODO SOME error handling when you have time
solutionSourceCode="$1"
# echo $solutionSourceCode
# trim rust extension
app=${solutionSourceCode%.rs}
# trim parent folder characters 'src/'
app=${app:4}
libPath=algs_std/src/
libBuild=algs_std/bld/
# Compile Solution
echo compiling ...
rustc -v algs_std/src/lib.rs --edition 2018  -L "algs_std/src/" --out-dir  algs_std/bld/

rustc -Ov $solutionSourceCode -o bin/$app  \
--extern crate=algs_std/bld/libalgs_std.rlib -L $libBuild --edition 2018 

echo finished ...
# Run Build
# echo running ...
# bin/./$app
exit 1