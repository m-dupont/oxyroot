̣#!/bin/bash

if ! command -v root 2>&1 >/dev/null
then
    echo "root executable could not be found. Please source the adequate thisroot.sh"
    exit 1
fi

if ! command -v cout2json 2>&1 >/dev/null
then
    echo "cout2json executable could not be found. Install with 'cargo install cout2json'"
    exit 1
fi

rm -rf /tmp/rust/gen_streamers_titles
rm -rf /tmp/rust/gen_streamers
cargo r --bin oxyroot-pre-gentitles
cat /tmp/rust/gen_streamers_titles/gen_all.txt | cout2json -t --delimiter "#" > /tmp/oxyroot.json
cargo r --release --bin oxyroot-pre-gendumps
rm /tmp/oxyroot.rs
cargo r --release --bin oxyroot-post-genrustfromdumps

rustfmt /tmp/oxyroot.rs
printf "If /tmp/oxyroot.rs is ok, replace streamers_db_generated with it\n"
#cp /tmp/oxyroot.rs  oxyroot/src/rdict/streamers/streamers_db_generated.rs