#!/usr/bin/env -S plasma run bash
#PLASMA depends "./build.sh" use-outputs=#true
#PLASMA input "build/"
#PLASMA output "deploy.log"
#PLASMA env "DEPLOY_ENV"

echo "Deploying to ${DEPLOY_ENV:-production}..."
echo "Deploy started at: $(date)" > deploy.log

# Check if build exists
if [ -d "build" ]; then
    echo "Build directory found" >> deploy.log
    ls -la build/ >> deploy.log
    echo "Deployment successful!" >> deploy.log
else
    echo "ERROR: Build directory not found!" >> deploy.log
    exit 1
fi
