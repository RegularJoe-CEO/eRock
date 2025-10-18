# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Fast Numeric Guardrails for UAS & Robotics

**Low‑latency expression evaluation and root‑finding on the companion computer**

---

**THE CHALLENGE: QUICK, LOCAL DECISIONS WITHOUT HEAVY PIPELINES**

Small airborne and ground robots need simple numeric decisions close to the sensors—thresholds, envelope checks, or solving for a crossing time—without hauling every check through a large framework or a remote link.

**THE FIT: eROCK – A SMALL, DETERMINISTIC CPU SERVICE**

**eRock** is a lightweight Rust service that exposes two operations over a local API:

- **Expression evaluation:** compute y = f(x) over numeric arrays (SIMD‑friendly on CPU).
- **Root‑finding:** robust bisection (with optional auto‑bracketing) to solve f(t)=0 within a tolerance.

Run it on a companion computer; your flight or robot stack calls it for quick numeric checks and continues its own decision logic.

---

### Where teams use it

- **Geofence & envelope math:** compute boolean conditions or time‑to‑breach from positions and velocities you already have.
- **Rule evaluation:** apply user‑defined formulas on derived features (battery, temperatures, currents) to gate heavier actions.
- **Parameter solves:** use `bisect_auto` to solve for a boundary value in a control or safety equation.

*(eRock evaluates formulas you supply; perception, control, communications, and actuation remain in your stack.)*

---

### Why use eRock

- **Deterministic:** explicit tolerances/iteration caps; same inputs → same outputs.
- **CPU‑efficient:** small footprint, SIMD‑friendly evaluation on ARM64 or x86.
- **Simple integration:** local HTTP or IPC; ships as a static binary (container optional).

---

### Technical Profile

- **Language:** Rust
- **Operations:** array expression eval; bisection (manual/auto‑bracket)
- **Design:** stateless requests; bounded runtime per call
- **Targets:** ARM64 or x86 (static builds; optional container)

---

**Do the simple math locally and fast—keep the rest of your autonomy stack focused on flying the mission.**
