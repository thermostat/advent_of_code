#/bin/bash

OUTDIR=advent_of_code_rendered
NBDIR=advent_of_code/2022

rm -rf ${OUTDIR}
mkdir ${OUTDIR}

jupyter-nbconvert --execute --to html --output-dir ${OUTDIR} ${NBDIR}/*.ipynb

