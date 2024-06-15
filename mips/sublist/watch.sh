#!/bin/sh

while true; do
    java -jar $HOME/bin/Mars45.jar nc runner.mips impl.mips
    sleep 10
done

