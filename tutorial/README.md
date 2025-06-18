# S.A.G.E. – Substrate Asset Game Engine

## 1. Introduction & Problem Statement

-

### 1.1 What is SAGE?

SAGE (Substrate Asset Game Engine) is an on‑chain, generic **asset‐state and transition framework** built as a Substrate pallet.  It gives blockchain developers a declarative way to define asset schemas ("what data does an asset carry?") and permissible state transitions ("what are the allowed moves?") together with rule‑sets that gate those transitions.  In short, SAGE turns custom game mechanics into a reusable, auditable state machine that lives directly on the chain.

### 1.2 Why SAGE?

Traditional blockchain game development faces a familiar dilemma:

| Challenge                                                                   | Consequence                                         |
| --------------------------------------------------------------------------- | --------------------------------------------------- |
| **Boiler‑plate low‑level code** to manage storage items, indexes and events | Reinventing wheel, copied bugs across projects      |
| **Hard‑coded logic** tightly coupled to the pallet                          | Adds friction to iterate or extend mechanics        |
| **Ad‑hoc security checks** sprinkled around                                 | Inconsistent validation – subtle exploits           |
| **Poor composability** across projects                                      | Assets can’t interact outside their original pallet |
| **Slow iteration cycles** (new runtime release for every rule change)       | Kills rapid prototyping & modding                   |

SAGE solves these with:

- **Declarative asset model** – describe asset fields & types, no boiler‑plate storage code.
- **Transition primitives** – CRUD‑style operations (create, update, delete, mint, burn, split, merge) chained into higher‑level moves.
- **Rule engine** – deterministic, on‑chain evaluation of pre‑ & post‑conditions expressed in a compact DSL.
- **Composable I/O graph** – transitions consume one or more input assets and emit one or more output assets, enabling Lego‑like gameplay logic.
- **Hot‑swappable spec** – minor rule tweaks can be signalled & upgraded without a full runtime bump.
- **Substrate‑native** – inherits fork‑less upgrades, weight accounting and multi‑sig governance out of the box.

### 1.3 Problems SAGE Solves for Blockchain Developers

1. **Time‑to‑Market**: Write *specs* instead of *code*; ship prototypes in hours, not weeks.
2. **Security Guarantees**: Centralised rule‑evaluation and exhaustive asset accounting remove entire classes of logic bugs (double‑spend, missing ownership checks, inconsistent state).
3. **Upgrade‑Friendliness**: Evolving game mechanics becomes a governance event, not a hard‑fork.
4. **Cross‑Game Interop**: Uniform asset structure & event semantics make it trivial for wallets, explorers and other games to understand and reuse assets.
5. **Community Modding**: Players can propose new transitions or fine‑tune parameters through DAO governance – no privileged dev required.
6. **Analytics Ready**: Every transition emits structured events, enabling rich on‑chain analytics and replayable simulations.

### 1.4 Reference Use Case — *“Penguin Eats Fish”*

To ground these abstractions, SAGE ships with a canonical tutorial:

> **Scenario**: A *Penguin* player asset consumes a *Fish* consumable asset to restore health.

```
Input‑1: Penguin (type = PLAYER, health = 95)
Input‑2: Fish    (type = CONSUMABLE, health = 5)
Transition: EAT
    • Update  Penguin.health += Fish.health
    • Delete  Fish
Output‑1: Penguin (health = 100)
```

The accompanying visual (see `docs/img/penguin_eats_fish.png`) illustrates how SAGE formalises this flow:

1. **Assets** are green boxes: two inputs and one output.
2. **Transition** (EAT) is the mint‑green processor block.
3. **Rules** (blue diamonds) enforce prerequisites: caller must own both assets, `Input‑1` must be of type `PLAYER`, `Input‑2` must be `CONSUMABLE`.

Without SAGE, a developer would need to hand‑roll:

- Storage structs for assets and a secondary index for ownership.
- Custom dispatchable that checks ownership, reads & mutates storage, handles deletion, emits events.

With SAGE, this entire flow is encoded in \~15 lines of transition spec and deployed once; the framework handles the rest.

---

## 2. Architecture *(to be drafted)*

## 3. Asset Specification DSL *(to be drafted)*

## 4. Transition & Rule Language *(to be drafted)*

## 5. Tutorial Walkthrough *(to be drafted)*

## 6. API Reference *(to be drafted)*

## 7. FAQ & Troubleshooting *(to be drafted)*

