# Restaurant POS — Requirements Input

## Overview
A Point of Sale system for a multi-branch restaurant chain supporting Dine-in, Takeaway, and Delivery channels.

**Devices**: Counter (PC/laptop), Tablet (Waiter), Kitchen Display / Printer, Manager Dashboard

**Primary Modules**: Sales, Menu, Table, Inventory, Recipe/BOM, Procurement, Promotion, Payment, Report, User, Audit

## Business Goals
- Reduce time to take order and payment
- Manage stock and reduce waste
- See sales in real-time
- Reduce human error
- Ready for many branch stores with shift working

## Functional Requirements
- Open/close shift per cashier with float tracking
- Separate bills, combine bills, transfer items between bills/tables
- Void with mandatory Manager approval + reason codes
- Menu with modifier groups (required/optional, single/multi-select), pricing per modifier
- Spicy-level management
- Menu recipes (BOM) with ingredient deltas for modifiers
- Table management with zone-based floor plan and visual coordinates
- Kitchen table reservation
- Kitchen status tracking via station-based display with elapsed time coloring
- Stock management with deduction on order placement, restock on void
- Procurement: Purchase Order → Receive → Stock-in with supplier management
- Promotions: rule-based auto-trigger with conditions and rewards
- Sales dashboard with live WebSocket headline numbers + periodic detail
- Best menu, cost, and profit reporting (daily summary, shift report, COGS)
- Payment: Cash + QR PromptPay, expandable to card/other methods
- Per-branch and per-channel pricing (Dine-in / Takeaway / Delivery)
- Delivery order management (manual entry, architected for aggregator API)
- Audit trail for all financial mutations and Order aggregate event log

## Actors
- **Owner** — cross-branch oversight, dashboard, user management
- **Manager** — branch operations, approvals, reporting, user assignment
- **Cashier** — counter orders, payments, shift open/close
- **Waiter** — table orders via tablet, bill management, serving
- **Kitchen** — order queue, station-based preparation, status updates
- **Stock/Admin** — inventory management, procurement, menu/recipe maintenance

## Architecture
- **Topology**: Hybrid — head office (cloud) owns master data (menus, recipes, users, promotions, suppliers); each branch runs full local stack for operational data (orders, tables, shifts, inventory, payments)
- **Master data sync**: Polling pull from branches to head office (60s interval), event-driven push post-MVP
- **Offline**: Full local operation — branch works completely offline, syncs when online
- **Kitchen real-time**: WebSocket from branch Rust backend to Kitchen Display
- **Auth**: Local credential cache synced from head office, validated locally
- **Deployment**: Single docker-compose per branch (Rust Axum + PostgreSQL + Redis), separate cloud aggregator deploy
- **Caching**: Redis for menu/price cache + pub/sub for real-time events
- **Migrations**: SQLx built-in forward migrations

## Stack
- **Frontend**: Next.js with `next-intl` (Thai + English)
- **Backend**: Rust Axum with SQLx
- **Database**: PostgreSQL (branch-local + head-office)
- **Cache/PubSub**: Redis
- **Containerization**: Docker + docker-compose
- **Payments**: QR PromptPay, Cash
