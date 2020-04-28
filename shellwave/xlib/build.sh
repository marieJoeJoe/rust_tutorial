#! /usr/bin/env bash

arg1=$1

if [ $arg1 == "gen" ]
then
	gcc creatingTriangle.c -o creatingTriangle -lX11 -lGL -lGLU;
	gcc rotatingTriangle.c -o rotatingTriangle -lX11 -lGL -lGLU;
	gcc scalerotateTriangle.c -o scalerotateTriangle -lX11 -lGL -lGLU;
elif [ $arg1 == "clean" ]
then
	rm creatingTriangle;
	rm rotatingTriangle;
	rm scalerotateTriangle;
else
	echo "build.sh param err!"
fi
