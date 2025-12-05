#!/usr/bin/env -S plasma run bash
#PLASMA input "src/*.txt"
#PLASMA output "dist/"
#PLASMA cache ttl="1d"

# Process all .txt files in src/ directory
mkdir -p dist
for file in src/*.txt; do
    if [ -f "$file" ]; then
        basename=$(basename "$file")
        cat "$file" | wc -l > "dist/${basename%.txt}-lines.txt"
    fi
done
echo "Processed all text files"
