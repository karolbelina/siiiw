#!/bin/bash
curl -o temp/train.zip http://hdl.handle.net/11321/738
curl -o temp/test.zip http://hdl.handle.net/11321/738

unzip -q temp/train.zip -d temp/train/
unzip -q temp/test.zip -d temp/test/

for value in handle contents dublin_core.xml license.txt metadata_local.xml
do
    rm temp/train/$value
    rm temp/test/$value
done

mkdir -p data
mv temp/train/* data/
mv temp/test/* data/

rm -r temp
