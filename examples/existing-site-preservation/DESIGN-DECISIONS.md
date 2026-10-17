# DESIGN-DECISIONS.md — existing-site-preservation

This ledger maintains a strict epistemic separation between **Observed Facts** (measured directly from source files or browser DOM), **Inferred Rules** (derived by Layoza's Design Intelligence Engine), and **User-Approved Decisions** (explicitly confirmed by the user).

## 1. Observed Facts (`ObservedFact`)

| ID | Category | Statement | Evidence Source | Confidence |
| :--- | :--- | :--- | :--- | :--- |
| `DEC-001` | framework | Project uses the `html-css-js` stack. | `project-scanner` | `85%` |

## 2. Inferred Design Rules (`InferredRule`)

| ID | Category | Statement | Basis | Confidence |
| :--- | :--- | :--- | :--- | :--- |
| `DEC-002` | preservation | Existing websites default to Preservation Mode: new sections or pages must reuse existing typography, color tokens, spacing scale, and motion intensity. | `layoza-mode-b-policy` | `100%` |

## 3. User-Approved Decisions (`UserApprovedDecision`)

_No user overrides recorded yet. Existing site preservation rules apply by default._
