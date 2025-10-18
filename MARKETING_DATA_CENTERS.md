# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: A Small, Efficient Numeric Service for the Data Center

**Low‑latency CPU math for pre‑checks and rule evaluation**

---

**THE CHALLENGE: COSTLY CYCLES ON SIMPLE MATH**

Many data‑center pipelines spend real CPU or GPU time on straightforward numeric work—thresholds, formula checks, or solving for a decision value—before heavier analytics or ML. Doing those checks efficiently on CPU improves end‑to‑end latency and can save power for these stages.

**THE FIT: eROCK – DETERMINISTIC NUMERIC EVALUATION ON CPU**

**eRock** is a lightweight Rust microservice that exposes two operations over HTTP:

- **Expression evaluation:** compute y = f(x) over numeric arrays (SIMD‑friendly).
- **Root‑finding:** robust bisection (with optional auto‑bracketing) to solve f(t)=0 within a tolerance.

Upstream services call eRock with numeric inputs; eRock returns results quickly and predictably so heavier components stay focused on their core work.

---

### Where teams use it

- **Finance / Pricing & Risk:** run CPU‑side numeric pre‑checks before submitting to pricing or risk engines (exact latency depends on deployment and batching).
- **Ad Tech / Filtering:** evaluate boolean rules on candidate records to reduce downstream load (when batched and colocated).
- **Scientific / HPC pipelines:** pre‑filter arrays with user‑defined formulas before compute‑intensive stages.
- **Telemetry gateways:** in your stream processor or gateway, call eRock to compute rule outcomes near ingest.

*(eRock evaluates your formulas on numeric arrays you provide; streaming, queues, and actuation remain in your system.)*

---

### Why use eRock

- **Deterministic & lightweight:** bounded iterations/tolerances; predictable CPU behavior.
- **Runs on x86/ARM:** deploy on standard servers or edge appliances.
- **Simple integration:** JSON over HTTP; small container footprint.

---

### Technical Profile

- **Language:** Rust
- **Operations:** array expression eval; bisection (manual/auto‑bracket)
- **Design:** stateless requests; SIMD‑friendly evaluation
- **Deployment:** Docker container (x86/ARM)

---

**Process the simple math efficiently—save the heavy lifting for where it matters.**
