#!/bin/bash

dist_path=$HOME/workplace/challenges/advent_of_code/"$1"/"$2"

cp -r ./template/. "$dist_path"/src/.
rm "$dist_path"/src/main.rs
