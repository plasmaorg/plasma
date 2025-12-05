# TurboRepo Integration

TurboRepo integration guide for Plasma. This assumes you've already [completed the getting started guide](/getting-started).

## How It Works

Plasma provides remote caching for TurboRepo via the v8 HTTP API. When you navigate to your project, Plasma automatically exports:

- `TURBO_API` - Points to Plasma's HTTP cache server
- `TURBO_TOKEN` - Auto-generated for authentication
- `TURBO_TEAM` - Auto-generated team identifier

TurboRepo automatically detects these environment variables and enables remote caching.

## Quick Start

```bash
cd ~/my-turborepo-workspace
turbo run build
```

> [!TIP]
> TurboRepo automatically detects the `TURBO_API`, `TURBO_TOKEN`, and `TURBO_TEAM` environment variables exported by Plasma's daemon. No manual configuration needed!
