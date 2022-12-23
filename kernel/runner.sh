#!/bin/bash
# delegate "cargo run/test" to make
cd .. && make run KERNELPATH=$1
