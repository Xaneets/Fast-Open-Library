#!/bin/bash

RED_COLOR='\033[0;31m'
GREEB_COLOR='\033[0;32m'
NO_COLOR='\033[0m'

cargo build --release

mkdir "resources"

cp target/release/fast_open_library resources/fast_open_library

cd resources


echo -e "Do you want to download and extract dumps? (${GREEB_COLOR}y${NO_COLOR}/${RED_COLOR}n${NO_COLOR}) "
read -r choice
if [[ $choice == "y" ]]; then
    wget -O ol_dump_authors_latest.txt.gz https://openlibrary.org/data/ol_dump_authors_latest.txt.gz
    wget -O ol_dump_works_latest.txt.gz https://openlibrary.org/data/ol_dump_works_latest.txt.gz
    wget -O ol_dump_editions_latest.txt.gz https://openlibrary.org/data/ol_dump_editions_latest.txt.gz
    echo "Files downloaded."
    gzip -d -c ol_dump_authors_latest.txt.gz > ol_dump_authors_latest.txt
    gzip -d -c ol_dump_works_latest.txt.gz > ol_dump_works_latest.txt
    gzip -d -c ol_dump_editions_latest.txt.gz > ol_dump_editions_latest.txt
    echo "Files unzipped."
fi

./fast_open_library





