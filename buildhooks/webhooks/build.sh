#!/bin/sh
set -xe

#cd /tmp/build
rm -rf ./*

# clone and build
git clone --recursive https://github.com/RPGLibrarium/Liberation.git
cd Liberation

dir="liberation.$(date -Iseconds)"

cargo build --release --target-dir  /target/"$dir"

# change symlink
cd /target
ln -sfn ./"$dir" ./liberation

# cleanup -> 2 days or older may be deleted!
find . -maxdepth 1 -mindepth 1 -type d -mtime +2 -exec rm -rf "{}" \;
