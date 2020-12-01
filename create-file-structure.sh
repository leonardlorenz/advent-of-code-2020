#!/bin/bash

for i in $(seq -w 1 24); do
    mkdir -p "$i"
    mkdir -p "$i/part1"
    mkdir -p "$i/part2"
done
