#! /usr/bin/env bash

arg1=$1

if [ $arg1 == "gen" ]
then
	gcc creatingTriangle.c -o creatingTriangle -lX11 -lGL -lGLU;
	gcc rotatingTriangle.c -o rotatingTriangle -lX11 -lGL -lGLU;
	gcc scalerotateTriangle.c -o scalerotateTriangle -lX11 -lGL -lGLU;
	gcc animate3Sided-Box.c -o  animate3Sided-Box -lX11 -lGL -lGLU;
	gcc creatingSphere.c -o creatingSphere -lX11 -lGL -lGLU -lm;
	gcc plotingGraph.c -o plotingGraph -lX11 -lGL -lGLU -lm;
	gcc animatingBlocks.c -o animatingBlocks -lX11 -lGL -lGLU -lm;

elif [ $arg1 == "clean" ]
then
	rm creatingTriangle;
	rm rotatingTriangle;
	rm scalerotateTriangle;
	rm animate3Sided-Box;
	rm creatingSphere;
	rm plotingGraph;
	rm animatingBlocks;
else
	echo "build.sh param err!"
fi
