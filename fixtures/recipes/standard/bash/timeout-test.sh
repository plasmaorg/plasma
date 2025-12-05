#!/usr/bin/env -S plasma run bash
#PLASMA output "timeout-output.txt"
#PLASMA exec timeout="2s"

echo "Starting long-running task..."
echo "This should timeout" > timeout-output.txt
sleep 10
echo "This should never execute" >> timeout-output.txt
