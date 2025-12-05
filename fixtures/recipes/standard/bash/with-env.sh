#!/usr/bin/env -S plasma run bash
#PLASMA env "USER" "HOME"
#PLASMA output "env-output.txt"
#PLASMA cache ttl="30m"

echo "User: $USER" > env-output.txt
echo "Home: $HOME" >> env-output.txt
echo "Generated at: $(date)" >> env-output.txt
