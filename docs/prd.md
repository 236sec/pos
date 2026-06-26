# PRD: Restaurant POS System

## Problem Statement

A multi-branch restaurant chain needs a unified Point of Sale system to reduce order and payment time, manage stock and minimize waste, see sales in real-time, and reduce human error. The system must serve Dine-in, Takeaway, and Delivery channels across multiple devices — Counter, Tablet (Waiter), Kitchen Display/Printer, and Manager Dashboard — while supporting shift-based operations at each branch. Each branch must operate fully offline with local resilience, while the head office retains central control over menus, pricing, recipes, promotions, and user access.

## Solution

A hybrid-architecture POS system: each branch runs a self-sufficient local stack (Rust Axum + PostgreSQL + Redis via Docker Compose) that handles all operational data — orders, tables, shifts, payments, inventory. A cloud head office owns master data (menus, recipes, users, promotions, suppliers) and aggregates cross-branch reporting. Branches sync master data via polling pull, with an event-driven push planned post-MVP. Kitchen communication uses WebSocket for real-time order display with station-based routing. The frontend is served by four purpose-built Next.js applications — Counter, Tablet, Kitchen Display, and Manager Dashboard — sharing a component library and supporting Thai/English localization.

## User Stories

### Owner

1. As an Owner, I want to see real-time aggregate sales across all branches on a single dashboard, so that I can monitor business performance at a glance.
2. As an Owner, I want to compare revenue, order count, and average ticket across branches for a selected date range, so that I can identify underperforming locations.
3. As an Owner, I want to view profit and loss summaries per branch, so that I can make informed financial decisions.
4. As an Owner, I want to create, edit, and deactivate user accounts across all branches, so that I control who has access to the system.
5. As an Owner, I want to assign roles (Manager, Cashier, Waiter, Kitchen, Stock/Admin) to users, so that each staff member has appropriate permissions.
6. As an Owner, I want to configure granular permissions per role, so that I can tailor access to my restaurant's specific needs.
7. As an Owner, I want to create and manage promotions that apply to all branches or specific branches, so that I can run chain-wide marketing campaigns.
8. As an Owner, I want to view the audit trail for financial mutations across all branches, so that I can investigate discrepancies.
9. As an Owner, I want to register a new branch in the head office, so that it can receive its configuration bundle and begin operating.
10. As an Owner, I want to view top-selling menu items across branches, so that I can optimize the menu offering.

### Manager

11. As a Manager, I want to view a live dashboard showing today's revenue, order count, and average ticket for my branch, so that I can track performance in real-time.
12. As a Manager, I want to see best-selling menu items and their profit margins, so that I can make menu optimization decisions.
13. As a Manager, I want to approve or reject void requests with a mandatory reason code, so that I prevent unauthorized voids and track waste.
14. As a Manager, I want to view, filter, and export daily sales summaries, so that I can reconcile end-of-day figures.
15. As a Manager, I want to review shift reports showing each cashier's opening float, expected cash, actual cash, and over/short amount, so that I can hold staff accountable.
16. As a Manager, I want to view inventory valuation and cost-of-goods-sold reports, so that I can manage food costs.
17. As a Manager, I want to create and edit menu items with base prices, modifier groups, and per-channel pricing (Dine-in/Takeaway/Delivery), so that I maintain an accurate menu.
18. As a Manager, I want to define modifier groups (e.g., spicy level, extra toppings) with individual pricing, so that the POS captures all order customizations.
19. As a Manager, I want to set up the branch floor plan by creating zones and placing tables with visual coordinates, so that waiters can use the tablet floor plan.
20. As a Manager, I want to configure branch-level promotions with rules and rewards, so that I can run location-specific offers.
21. As a Manager, I want to manage suppliers with contact details and product catalogues, so that procurement is streamlined.
22. As a Manager, I want to create purchase orders for ingredients, so that stock can be replenished systematically.
23. As a Manager, I want to receive stock against a purchase order and record actual quantities and costs, so that inventory reflects real stock levels.
24. As a Manager, I want to assign roles to branch staff, so that each employee has the correct permissions.
25. As a Manager, I want to configure whether the kitchen uses a display, a printer, or both, so that the kitchen workflow matches branch preference.
26. As a Manager, I want to view menu performance rankings by revenue, quantity sold, and profit, so that I can optimize the menu mix.

### Cashier

27. As a Cashier, I want to open my shift by counting and recording my starting float, so that my shift is properly initialized.
28. As a Cashier, I want to close my shift by counting my ending cash and seeing the over/short calculation, so that I can reconcile my drawer.
29. As a Cashier, I want to create a new order for Dine-in, Takeaway, or Delivery, so that I can serve any customer.
30. As a Cashier, I want to add menu items to an order with modifier selections (spice level, add-ons, portion size), so that orders are accurately captured.
31. As a Cashier, I want to apply discounts and active promotions to an order, so that customers receive eligible offers.
32. As a Cashier, I want to accept cash payments and calculate change, so that I can complete cash transactions.
33. As a Cashier, I want to accept QR PromptPay payments with a reference, so that customers can pay digitally.
34. As a Cashier, I want to split a bill by selecting which items go to which new bill, so that groups can pay separately.
35. As a Cashier, I want to combine two bills into one, so that customers who decide to pay together can do so.
36. As a Cashier, I want to transfer items from one bill to another, so that I can correct billing errors.
37. As a Cashier, I want to request a void for an item or order with a reason code, which requires Manager approval, so that voids are controlled.
38. As a Cashier, I want to print a customer receipt, so that the customer has a record of their payment.
39. As a Cashier, I want to see my order history for the current shift, so that I can reference past transactions.
40. As a Cashier, I want to apply a percentage or fixed-amount discount to an order, so that I can handle discretionary discounts.
41. As a Cashier, I want to see real-time table availability, so that I can seat walk-in customers.

### Waiter

42. As a Waiter, I want to view a visual floor plan showing all tables with status colors (available, occupied, dirty, reserved), so that I can manage my section at a glance.
43. As a Waiter, I want to tap a table on the floor plan to create a new order, so that I can start taking orders quickly.
44. As a Waiter, I want to add menu items with modifiers and special instructions to a table order, so that the kitchen receives complete details.
45. As a Waiter, I want to send the order to the kitchen, so that preparation can begin.
46. As a Waiter, I want to see the real-time status of each item in an order (waiting, cooking, done), so that I know when to serve.
47. As a Waiter, I want to mark a table as dirty after customers leave, so that cleaning staff are notified.
48. As a Waiter, I want to split a bill by item at the table, so that groups can pay separately without going to the counter.
49. As a Waiter, I want to transfer items between tables, so that I can correct seating mistakes.
50. As a Waiter, I want to accept payment at the table (cash or PromptPay), so that customers don't need to visit the counter.
51. As a Waiter, I want to reserve a table for a future time, so that bookings are honoured.
52. As a Waiter, I want to see which menu items are out of stock, so that I don't take orders for unavailable dishes.
53. As a Waiter, I want to move a customer's order to a different table, so that I can accommodate seating changes.

### Kitchen

54. As a Kitchen staff, I want to see new orders appear on my station display in real-time, so that I can start cooking immediately.
55. As a Kitchen staff, I want to see only the items relevant to my station (grill, fry, cold, drinks), so that I'm not distracted by irrelevant orders.
56. As a Kitchen staff, I want to see elapsed time for each order item with color coding (green/yellow/red), so that I can prioritize delayed orders.
57. As a Kitchen staff, I want to mark an item as "in progress," so that waiters know cooking has started.
58. As a Kitchen staff, I want to mark an item as "done," so that waiters know it's ready to serve.
59. As a Kitchen staff, I want to mark an item as "out of stock," so that waiters are immediately notified and can't sell it further.
60. As a Kitchen staff, I want to see modifier selections and special instructions for each item, so that I prepare orders correctly.
61. As a Kitchen staff, I want printed kitchen tickets to appear automatically (if printer configured), so that I have a physical record.
62. As a Kitchen staff, I want to view orders sorted by wait time, so that I process the oldest orders first.

### Stock/Admin

63. As a Stock/Admin, I want to view current stock levels for all ingredients, so that I know what needs reordering.
64. As a Stock/Admin, I want to see stock movement history (additions, deductions, adjustments), so that I can trace inventory changes.
65. As a Stock/Admin, I want to create a purchase order with items, quantities, and supplier, so that I can initiate procurement.
66. As a Stock/Admin, I want to receive stock against a purchase order by confirming actual quantities and unit costs, so that inventory is accurately updated.
67. As a Stock/Admin, I want to manually adjust stock counts with a required reason, so that I can correct discrepancies found during stock counts.
68. As a Stock/Admin, I want to define a recipe (BOM) for each menu item specifying ingredients and quantities, so that inventory can be deducted automatically.
69. As a Stock/Admin, I want to define modifier ingredient deltas (e.g., extra cheese adds 50g), so that modifier selections affect stock accurately.
70. As a Stock/Admin, I want to set reorder thresholds per ingredient, so that I'm alerted before running out.
71. As a Stock/Admin, I want to manage suppliers with name, contact, and product list, so that procurement is organized.
72. As a Stock/Admin, I want to view cost of goods sold by menu item, so that I can optimize pricing and recipes.

### Cross-cutting

73. As any user, I want to log in with my credentials and access only the features my role permits, so that the system is secure.
74. As any user, I want to switch the interface language between Thai and English, so that I can work in my preferred language.
75. As any user, I want to change my password, so that I can maintain account security.
76. As a user with access to multiple branches, I want to select which branch I'm operating at login, so that I work in the correct context.

## Implementation Decisions

### Backend Services (Rust Axum — Cargo Workspace)

The backend is organized into seven services, each a separate crate:

- **Auth Service** — Authentication, credential cache sync, role/permission resolution, user management, branch assignments.
- **POS Service** — Order lifecycle (create → kitchen → serve → pay), billing (split/combine/transfer), void approval workflow, payment processing (Cash, PromptPay), shift open/close with float reconciliation.
- **Menu Service** — Menu items, modifier groups and options, per-branch and per-channel pricing (Dine-in/Takeaway/Delivery), table management (zones, floor plan, reservations).
- **Inventory Service** — Stock tracking, automated deduction on order placement and compensating restock on void, recipe/BOM management with modifier ingredient deltas, stock adjustments.
- **Procurement Service** — Supplier management, purchase orders, receiving goods against POs, stock-in recording.
- **Report Service** — Daily sales summary, shift reports, menu performance rankings, inventory valuation, COGS, cross-branch aggregation for head office.
- **Notification Service** — WebSocket connection management, kitchen ticket delivery with station-based routing, real-time dashboard push events, print daemon integration.

A shared `core` crate provides: error types, domain primitives (Money, Quantity, Timestamp), database pool management, configuration, and common middleware.

### Deep Modules (Pure Logic, Testable in Isolation)

These live within their respective services but are extracted as functions/modules with no I/O dependencies:

- **Pricing Engine** (Menu Service): `fn resolve(order_context) -> FinalPrice` — combines base menu price, modifier deltas, channel markup, and applied promotions into a final price.
- **Inventory Deduction Engine** (Inventory Service): `fn calculate_deductions(recipe, items) -> Vec<StockDelta>` — takes a recipe BOM and order items, outputs ingredient decrement amounts.
- **Kitchen Ticket Splitter** (Notification Service): `fn split_by_station(order) -> HashMap<Station, KitchenTicket>` — splits an order into station-specific tickets.
- **Permission Guard** (Auth Service): `fn can(user, action, resource) -> bool` — checks a user's role permissions against a requested action.
- **Shift Calculator** (POS Service): `fn reconcile(expected_cash, actual_cash, transactions) -> OverShort` — computes expected-to-actual cash difference.
- **Promotion Evaluator** (Menu Service): `fn evaluate(cart, active_promos) -> Vec<AppliedPromo>` — evaluates all active promotion rules against a cart and returns applicable rewards.

### Data Architecture

**Head Office Database** — master data tables:
- Users, roles, permissions, user-role-branch assignments
- Menu items, modifier groups, modifier options
- Recipe/BOM definitions
- Promotions (rules, conditions, rewards)
- Suppliers
- Branch registry

**Branch Database** — operational data tables:
- Orders, order items, order modifiers, order status history
- Bills (subsets of order items for payment)
- Payments (method, amount, reference, timestamp)
- Shifts (open/close, float, over/short)
- Inventory (current stock, movements, adjustments)
- Purchase orders and receiving records
- Kitchen ticket events
- Audit log (append-only)

**Key Schema Decisions:**
- Orders are the atomic unit sent to kitchen; Bills are payment groupings that reference order items. One order can have multiple bills. This supports split, combine, and transfer without affecting the kitchen.
- Modifier selections are stored as `(modifier_group_id, modifier_option_id, price_override_amount)` on order items, capturing what was chosen and at what price at order time — immune to later menu changes.
- `payment_method` is an enum: `Cash | PromptPay`, expandable to `Card | TrueMoney | BankTransfer`.
- Audit log is an append-only table: `(id, timestamp, actor_id, action, resource_type, resource_id, before_jsonb, after_jsonb, reason)`.
- Promotion rules are stored as `(trigger_type, condition_jsonb, reward_type, reward_jsonb, start_date, end_date, branch_id_nullable)`. Conditions and rewards are structured JSONB for flexibility.

### API Contracts

**Branch API — REST endpoints by service:**

*Auth Service:*
- `POST /auth/login` → returns session token + role permissions + branch info
- `POST /auth/logout`
- `GET /auth/me` → current user + permissions
- `PUT /auth/password`

*POS Service:*
- `POST /orders` → create order (dine-in/takeaway/delivery, table_id optional)
- `GET /orders/:id` → order with items, modifiers, status, bills
- `GET /orders/active` → all active (non-closed) orders
- `POST /orders/:id/items` → add item with modifiers
- `DELETE /orders/:id/items/:item_id` → remove item (with void request if needed)
- `POST /orders/:id/send-to-kitchen` → transitions order to kitchen
- `POST /orders/:id/void` → void request (requires manager_approval_pin)
- `GET /orders/:id/bills` → bills for this order
- `POST /orders/:id/bills/split` → split bill by selected item IDs
- `POST /bills/combine` → combine two bills
- `POST /bills/:id/transfer-items` → transfer items between bills
- `POST /bills/:id/pay` → process payment (method, amount, reference)
- `POST /shifts/open` → open shift (starting_float)
- `POST /shifts/:id/close` → close shift (ending_cash)
- `GET /shifts/current` → current user's active shift

*Menu Service:*
- `GET /menu/items` → all items with modifiers and pricing
- `GET /menu/categories` → item categories
- `GET /tables` → tables with zone and status
- `GET /tables/:id` → table detail with current order
- `PUT /tables/:id/status` → update table status
- `POST /tables/:id/reserve` → create reservation
- `GET /promotions/active` → currently active promotions

*Inventory Service:*
- `GET /inventory` → current stock levels
- `GET /inventory/:ingredient_id/movements` → stock movement history
- `POST /inventory/adjust` → manual stock adjustment (with reason)
- `GET /recipes/:menu_item_id` → recipe/BOM for item

*Procurement Service:*
- `GET /suppliers` → supplier list
- `POST /purchase-orders` → create PO
- `GET /purchase-orders` → PO list with status
- `POST /purchase-orders/:id/receive` → receive stock against PO

*Report Service:*
- `GET /reports/daily-summary?date=` → daily revenue, orders, payment breakdown
- `GET /reports/shift?shift_id=` → shift reconciliation report
- `GET /reports/menu-performance?from=&to=` → menu item ranking
- `GET /reports/inventory-valuation` → current stock value
- `GET /reports/cogs?from=&to=` → cost of goods sold

**Head Office API:**
- `POST /branches` → register new branch, generate config bundle
- `GET /reports/aggregate?from=&to=` → cross-branch aggregated reports

**Sync Endpoints:**
- `GET /sync/menus?since=` → changed menu data since timestamp
- `GET /sync/users?since=` → changed user/role data since timestamp
- `GET /sync/promotions?since=` → changed promotion data since timestamp

### Real-time Communication

**WebSocket endpoint:** `WS /ws/kitchen?station=grill`

- Server pushes `kitchen_ticket` events when an order is sent to kitchen
- Kitchen clients push `status_update` messages: `{ item_id, new_status: "in_progress" | "done" | "out_of_stock" }`
- Server fans out status updates to relevant tablet clients so waiters see live status
- Dashboard headline numbers (today's revenue, order count) are pushed via the same WebSocket infrastructure to authenticated dashboard clients
- Print daemon subscribes to `kitchen_ticket` events and renders to ESC/POS format

**Redis pub/sub channels:**
- `branch:{id}:kitchen:tickets` — kitchen ticket events
- `branch:{id}:kitchen:status` — item status updates
- `branch:{id}:dashboard:metrics` — live metric updates

### Sync Strategy

Branches poll head office every 60 seconds for master data changes:
1. Branch sends `GET /sync/menus?since={last_sync_timestamp}`
2. Head office returns only records updated since that timestamp
3. Branch upserts the changes locally
4. Repeat for users and promotions

Conflict resolution: head office is always the source of truth. Branch-local master data is read-only cache.

### Frontend Architecture

Four Next.js applications in a monorepo with a shared component package:

- **Counter App** — Optimized for keyboard-driven speed: quick item search, barcode-style menu grid, payment screen with cash calculator, shift management sidebar.
- **Tablet App** — Touch-first: interactive floor plan canvas, drag-to-order workflow, bill splitting interface, table status management.
- **Kitchen Display App** — Full-screen, glanceable: station-specific order queue with large text, color-coded elapsed timers, large touch targets for status bumps, audible alerts for new orders.
- **Manager Dashboard App** — Data-dense: report tables with export, menu configuration forms, user management tables, procurement workflows.

The shared package contains: menu item card, order card, payment widget, modifier picker, table icon, status badge, i18n translation files, API client.

### Localization

- Framework: `next-intl` configured from day one
- Languages: Thai (`th`) and English (`en`)
- Translation files per app + per shared component
- Backend error messages return i18n keys; frontend resolves to user's locale
- Menu items store `name` (Thai) and `name_en` (English); receipt language is user-configurable

## Testing Decisions

**Test philosophy:** Tests verify external behavior (inputs → outputs, state transitions, side effects on the database), not internal implementation details. Mock only at the boundary (external services, time). Prefer real PostgreSQL in test containers over mocked repositories.

**Deep modules (unit tests):** The six pure-logic modules are tested with table-driven unit tests — no database or network, just in-memory input → output assertions.
- Pricing Engine: test combinations of base price, modifier deltas, channel markup, and promotion application
- Inventory Deduction Engine: test recipe BOM resolution with modifier deltas
- Kitchen Ticket Splitter: test station assignment for various order compositions
- Permission Guard: test role → permission matrix exhaustively
- Shift Calculator: test float reconciliation with various transaction patterns
- Promotion Evaluator: test rule matching, condition evaluation, stacking priority

**Service integration tests:** Each service crate contains integration tests with a real PostgreSQL instance (test container or in-memory SQLite via SQLx) covering:
- Auth: login flow, credential cache sync, permission resolution
- POS: full order lifecycle from creation to payment, bill split/combine, void approval workflow, shift open/close with reconciliation
- Menu: CRUD for items and modifiers, per-channel price resolution
- Inventory: automatic deduction on order placement, restock on void, adjustment history
- Procurement: PO creation, partial/full receiving, stock increment
- Notification: WebSocket connection lifecycle, kitchen ticket fan-out, station routing
- Report: daily summary aggregation, shift report accuracy, COGS calculation

**API contract tests:** Each endpoint is tested for:
- Happy path response shape and status code
- Authorization (correct role required, wrong role returns 403)
- Validation errors (malformed input returns 422)
- Idempotency where applicable

## Out of Scope

- **Third-party delivery aggregator integration** (Grab, FoodPanda, LINEMAN) — the system supports delivery orders entered manually. Aggregator API integration is a future phase.
- **Integrated EDC/card terminal** — card payments are manual entry only. EDC machine integration is a separate sub-project.
- **Customer-facing QR ordering** — customers cannot scan a QR at the table to order from their phone. The API is architected to support this later.
- **Loyalty/membership program** — no customer accounts, points, or member tiers. Promotions are the only discount mechanism.
- **Time-based pricing** (happy hour) — not baked into base pricing. Time-based triggers are handled through the Promotion module.
- **Full BI suite** — interactive chart dashboards, year-over-year comparisons, labor cost reporting, and waste tracking are post-MVP.
- **Zero-touch branch provisioning** — branch setup requires pasting a config bundle into the docker-compose environment. Fully automated provisioning (Terraform, Ansible) is out of scope.
- **Kubernetes deployment** — production uses single-machine docker-compose per branch.
- **Mobile app** — the Tablet app is a responsive web app, not a native iOS/Android app.

## Further Notes

- **Offline-first design:** The branch operates entirely from its local PostgreSQL. The Rust backend, PostgreSQL, and Redis all run on the branch machine. Internet loss does not affect order taking, kitchen operations, or payment processing. Master data changes made at head office during an outage are synced when connectivity resumes.
- **Database per branch:** Each branch has its own PostgreSQL database. This eliminates cross-branch query complexity in operational code and ensures branch performance isolation. Head office aggregation happens via the Report Service reading from a central reporting database.
- **Print daemon:** A lightweight process (could be a separate Rust binary or a node service) subscribes to Redis kitchen ticket events and renders them to ESC/POS-compatible thermal printers. This is an adapter — the backend emits events, the daemon bridges to hardware.
- **Migration strategy:** SQLx forward-only migrations with `.sql` files in a `migrations/` directory. Head office and branch databases have separate migration directories. Rollbacks are handled by writing new forward migrations.
- **Security:** All endpoints except login require authentication. Void and price-override actions require Manager PIN verification. Audit log entries are created for every financial mutation — they are append-only and never updatable via application code. Passwords are hashed with argon2.
- **Configuration:** Branch docker-compose receives its configuration (database URL, branch ID, head office URL, Redis URL) via environment variables. A "config bundle" from head office bootstraps initial master data.
