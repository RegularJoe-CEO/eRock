# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Fast Numeric Filters for Scientific Pipelines

**Low‑latency expression evaluation and root‑finding you can call inside HPC data flows**

---

**THE CHALLENGE: QUICK DECISIONS ON HUGE DATA**

Scientific facilities generate enormous data streams, but many early keep/drop decisions reduce to fast numeric checks on features already extracted by DAQ or preprocessing code. If those checks aren’t efficient, they add latency and consume precious compute.

**THE FIT: eROCK – A SMALL, DETERMINISTIC CPU SERVICE**

**eRock** is a lightweight Rust microservice exposing two operations over HTTP:

- **Expression evaluation:** compute y = f(x) over numeric arrays (SIMD‑friendly).
- **Root‑finding:** robust bisection (with optional auto‑bracketing) to solve f(t)=0 within a tolerance.

Your DAQ/stream job calls eRock with numeric inputs; eRock returns results quickly and predictably so heavier stages stay focused on deep analysis.

---

### Where teams use it

- **High‑rate experiments:** evaluate user‑defined formula gates on features your DAQ pipeline computes, reducing downstream load.
- **Radio astronomy:** apply simple numeric filters to channelized or aggregated measurements before more expensive detection steps.
- **Genomics pipelines:** run scalar or array‑based rule checks on per‑read or per‑chunk quality metrics produced upstream.
- **Earth observation & climate:** compute threshold logic on derived variables prior to assimilation or large simulations.

*(eRock evaluates formulas on numeric arrays you provide; domain‑specific parsing, DSP, and data transport remain in your system.)*

---

### Why use eRock

- **Deterministic:** explicit tolerances/iteration caps; reproducible given fixed inputs.
- **CPU‑friendly:** runs on x86/ARM; SIMD‑aware evaluation.
- **Simple to integrate:** JSON over HTTP; small container footprint.

---

### Technical Profile

- **Language:** Rust
- **Operations:** array expression eval; bisection (manual/auto‑bracket)
- **Design:** stateless requests; SIMD‑friendly evaluation
- **Deployment:** Docker container (x86/ARM)

---

**Filter the simple stuff fast. Save the big iron for discovery.**
