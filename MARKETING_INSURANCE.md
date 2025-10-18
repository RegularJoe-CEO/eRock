# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Fast Numeric Calculations for Insurance Platforms

**Low‑latency expression evaluation and root‑finding you can call from underwriting and rating systems**

---

**THE CHALLENGE: SLOW MATH IN THE CRITICAL PATH**

Modern quoting and risk workflows still spend valuable time doing straightforward numeric work—rating formulas, thresholds, and parameter solves—before or alongside calls to external data sources. If those calculations aren’t efficient, they add latency and cost.

**THE FIT: eROCK – A SMALL, DETERMINISTIC CPU SERVICE**

**eRock** is a lightweight Rust microservice that exposes two operations over HTTP:

- **Expression evaluation:** compute y = f(x) over numeric arrays (SIMD‑friendly).
- **Root‑finding:** robust bisection (manual or auto‑bracket) to solve f(t)=0 within a tolerance.

Your platform supplies the inputs and formulas; eRock returns results quickly and predictably so your quoting and risk services can respond faster.

---

### Where teams use it

- **Quoting paths:** compute rating factors and surcharges from normalized inputs; keep external I/O out of the hot loop.
- **Usage‑based scoring:** have your telematics pipeline call eRock for numeric score updates on already‑derived features.
- **Exposure calculations:** evaluate portfolio‑level or per‑location formulas before more expensive simulation steps.
- **Rules with thresholds:** use root‑finding to solve for boundary values in pricing or eligibility formulas.

*(eRock evaluates your formulas; data acquisition, eligibility logic, policy binding, and simulations remain in your systems.)*

---

### Why use eRock

- **Deterministic:** explicit tolerances/iteration caps; same inputs → same outputs.
- **CPU‑efficient:** runs on x86/ARM; SIMD‑aware evaluation.
- **Simple integration:** JSON over HTTP; small container footprint.

---

### Technical Profile

- **Language:** Rust
- **Operations:** array expression eval; bisection (manual/auto‑bracket)
- **Design:** stateless requests; SIMD‑friendly evaluation
- **Deployment:** Docker container (x86/ARM)

---

### Upgrade your math path

Use eRock to compute the numeric pieces quickly—so your underwriting can move faster.
