#!/bin/bash

DOCKER=podman
IMAGE=6519af2cf56c
#IMAGE=jupyter/scipy-notebook:85f615d5cafa

sudo ${DOCKER} run \
       -v /home/danw/notes:/home/jovyan/work  \
       -v /home/danw/code/jupnace:/home/jovyan/jupnace \
       -w /home/jovyan/work/ipy \
       --rm -it   -p 8888:8888 ${IMAGE} advent_of_code/render_nb.sh

