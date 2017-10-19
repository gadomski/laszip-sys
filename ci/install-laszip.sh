#!/usr/bin/env sh

set -ex

git clone https://github.com/LASzip/LASzip
mkdir LASzip/build && cd LASzip/build
cmake .. \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX="$1"
make
make install
