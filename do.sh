#!/bin/bash
SCRIPT_NAME="do"
BASEDIR=$(dirname "$0")

if [ $# -ne 0  ]; then
        flag="${1}"
        if [ "$flag" == "run" ]; then
                (set -x; cargo run)
        elif [ "$flag" == "test" ]; then
                (set -x; cargo test)
        elif [ "$flag" == "build" ]; then
                (set -x; cargo build)
        elif [ "$flag" == "dummy" ]; then
                echo "dummy ${BASEDIR}"
        fi
else
    echo "Usage: \"./${SCRIPT_NAME} gen-pb\"";
    exit 1
fi

#case $opt in
#while getopts ":a:" opt; do
#a)
#echo "-a was triggered, Parameter: $OPTARG" >&2
#;;
#\?)
        #echo "Invalid option: -$OPTARG" >&2
        #exit 1
        #;;
#:)
        #echo "Option -$OPTARG requires an argument." >&2
        #exit 1
        #;;
        #esac
        #done
