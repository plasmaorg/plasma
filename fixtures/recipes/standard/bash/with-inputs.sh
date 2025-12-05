#!/usr/bin/env -S plasma run bash
#PLASMA input "input.txt"
#PLASMA output "output.txt"
#PLASMA cache ttl="1h"

# This script processes input.txt and generates output.txt
echo "Processing input file..."
cat input.txt | tr '[:lower:]' '[:upper:]' > output.txt
echo "Processing complete!"
