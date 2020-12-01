#!/bin/bash

for i in $(seq -w 1 25); do
    mkdir -p "$i"
    mkdir -p "$i/part1"
    mkdir -p "$i/part2"
done
