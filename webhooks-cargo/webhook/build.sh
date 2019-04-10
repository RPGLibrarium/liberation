#!/bin/sh
set -xe

#cd /tmp/build
rm -rf ./*

# clone and build
git clone --recursive https://github.com/RPGLibrarium/Liberation.git
cd Liberation

date="$(date -Iseconds)"
releasedir="liberation.release.$date"
staticdir="liberation.static.$date"

cargo build --release --target-dir  /target/"$releasedir"

# copy static files
cp -R /web/app /target/"$staticdir"

# change symlink
cd /target
ln -sfn ./"$releasedir" ./liberation.release
ln -sfn ./"$releasedir" ./liberation.static

# cleanup -> 2 days or older may be deleted!
find . -maxdepth 1 -mindepth 1 -type d -mtime +2 -exec rm -rf "{}" \;
