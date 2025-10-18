# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: A Small, Deterministic Math Service for the Factory Edge

**Low-latency numeric evaluation and root-finding next to your machines**

---

**THE CHALLENGE: LOTS OF SENSOR DATA, SIMPLE DECISIONS NEEDED QUICKLY**

Factories produce continuous telemetry, but many edge decisions reduce to fast math on recent measurements (thresholds, formulas, or solving for a value). Offloading every check to the cloud adds latency and cost.

**THE FIT: eROCK – NUMERIC CO-PROCESSOR AT THE EDGE**

**eRock** is a lightweight Rust microservice you run on an industrial PC or gateway. It exposes two capabilities over HTTP:

- **Expression evaluation**: compute y = f(x) over arrays (vectorized).
- **Root-finding**: robust bisection (with auto-bracketing) to solve f(t)=0 within a tolerance.

Upstream systems (PLCs, vision, analytics) call eRock with measured features and formulas; eRock returns the numbers quickly and deterministically.

---

### Examples on the Factory Floor (with upstream systems in place)

- **Maintenance rules:** Evaluate health indices (e.g., RMS, crest factor) that your vibration pipeline computes, and trip alerts when formulas cross thresholds.
- **Quality checks:** If a vision system outputs dimensions, use eRock to apply tolerance math at line speed.
- **Process KPIs:** Evaluate efficiency or mass‑balance equations from live process variables.
- **Geofence math:** Given positions/velocities, compute a time‑to‑breach or rule condition; your safety controller decides the action.

*(eRock does the math; streaming, feature extraction, and actuation remain in your systems.)*

---

### Why teams use eRock

- **Deterministic & lightweight:** Small Rust service, predictable latency, explicit iteration/tolerance limits.
- **Runs anywhere:** Edge gateway/IPC, x86 or ARM.
- **Simple integration:** JSON in/out; call from PLC gateways, SCADA connectors, or companion apps.

---

### Build a Smarter Edge—One Decision at a Time

Use eRock to evaluate formulas and solve for thresholds locally. Pair it with your existing telemetry, analytics, and control systems to reduce latency and cloud dependence.
