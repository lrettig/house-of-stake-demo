# House of Stake End to End Prototype

## Overview
This monorepo contains a basic, prototype Next.js frontend and a NEAR smart contract implementing veNEAR (vote escrowed NEAR) for House of Stake governance. It allows users to escrow native NEAR in exchange for veNEAR, which they can then use to vote on governance proposals or delegate to other users.

## Architecture
- Frontend: Next.js
- Smart Contract: Rust
- Technology stack: Next.js, Tailwind, NEAR SDK, Rust
- Important design decisions:
  - Implement a basic governance mechanism with proposal creation and voting
  - Use NEAR's Wallet Connection for user authentication

## Current Status
Nothing implemented yet.

## Directory Structure
- `frontend/` - Next.js frontend
- `contract/` - NEAR smart contract
- `shared/` - Shared types and utilities

## Documentation
- [NEAR SDK Documentation](https://docs.near.org/sdk/intro)
- [Rust Programming Language Documentation](https://doc.rust-lang.org/book/)
- [Next.js Documentation](https://nextjs.org/docs)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)

## Additional Notes
- This project is a work in progress and will be updated as we add more features and functionality.

## Warning
- This is a prototype and is not intended to be used in production.
- The smart contract is not audited and is not intended to be used in production.
- The frontend is not styled and is not intended to be used in production.
