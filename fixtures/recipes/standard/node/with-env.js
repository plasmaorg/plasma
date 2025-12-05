#!/usr/bin/env -S plasma run node
//PLASMA env "NODE_ENV" "USER"
//PLASMA output "env-info.json"
//PLASMA cache ttl="30m"

const fs = require('fs');

const envInfo = {
  nodeEnv: process.env.NODE_ENV || 'not set',
  user: process.env.USER || 'unknown',
  platform: process.platform,
  nodeVersion: process.version,
  timestamp: new Date().toISOString()
};

fs.writeFileSync('env-info.json', JSON.stringify(envInfo, null, 2));
console.log('Environment info written to env-info.json');
