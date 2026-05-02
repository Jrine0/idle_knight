# Idle Knight

Idle Knight is a Web3 creator platform where creators publish public and premium content, and supporters subscribe with wallet-based identity.

## What Idle Knight Does

-   Wallet-native identity for creators and supporters
-   Creator profiles with on-chain ownership controls
-   Public and premium content publishing
-   Subscription payments with expiry-based access windows
-   Premium content gating backed by contract state
-   Event-driven activity for subscriptions, payments, and access checks

## Architecture Overview

### On-chain responsibilities

-   Creator profile ownership and updates
-   Compact content metadata (IDs, creator ownership, access policy)
-   Subscription accounting (`subscriber -> creator -> expiry`)
-   Access verification for premium content
-   Payment settlement through contract-mediated token transfers

### Off-chain responsibilities

-   Media storage and delivery (IPFS / DB / CDN)
-   Feed indexing and query optimization
-   Optional API-side caching and request shaping

> Large media assets are intentionally kept off-chain. Only references and access rules are recorded on-chain.

## Project Structure

-   `soroban/idle-knight-contract`: contract code for creators, content metadata, subscriptions, and access gating
-   `backend/sorobanService.ts`: backend integration layer for chain health checks and access validation hooks
-   `src/lib/stellar/*`: wallet connection and domain helpers used by the frontend

## Development

```bash
npm install
npm run dev
```

Open http://localhost:3000.

## Production Checklist

1. Add contract test coverage (unit + integration).
2. Implement full read/write transaction pipelines for all user flows.
3. Index and persist events for analytics and creator dashboards.
4. Add subscription extension policy (stack/overwrite) and billing edge-case tests.
5. Add monitoring and alerting for RPC latency and failed transaction rates.
