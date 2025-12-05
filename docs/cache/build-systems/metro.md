# Metro Integration

Metro integration guide for Plasma. This assumes you've already [completed the getting started guide](/getting-started).

## How It Works

Plasma provides HTTP-based caching for Metro bundler (React Native). When you navigate to your project, Plasma exports `PLASMA_HTTP_URL` which you can configure Metro to use.

## Quick Start

Update your `metro.config.js` to use Plasma's cache:

```javascript
const {getDefaultConfig} = require('metro-config');

module.exports = (async () => {
  const config = await getDefaultConfig();
  
  const cacheStores = [
    require('metro-cache/src/stores/FileStore'),
  ];
  
  // Add Plasma remote cache
  if (process.env.PLASMA_HTTP_URL) {
    cacheStores.push({
      get: async (key) => {
        try {
          const response = await fetch(`${process.env.PLASMA_HTTP_URL}/api/v1/artifacts/${key}`);
          return response.ok ? await response.buffer() : null;
        } catch {
          return null;
        }
      },
      set: async (key, value) => {
        try {
          await fetch(`${process.env.PLASMA_HTTP_URL}/api/v1/artifacts/${key}`, {
            method: 'PUT',
            body: value,
          });
        } catch {}
      },
    });
  }
  
  return {
    ...config,
    cacheStores,
  };
})();
```

Then start Metro:

```bash
cd ~/my-react-native-app
npm start
```
