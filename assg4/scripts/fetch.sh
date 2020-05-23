#!/bin/bash
mkdir -p temp

curl -s -o temp/train.zip https://clarin-pl.eu/dspace/bitstream/handle/11321/738/wiki_train_34_categories_data.zip
curl -s -o temp/test.zip https://clarin-pl.eu/dspace/bitstream/handle/11321/739/wiki_test_34_categories_data.zip

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
