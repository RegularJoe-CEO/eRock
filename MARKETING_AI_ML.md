# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Fast Numeric Pre-Checks for AI Pipelines

**Low-latency expression evaluation and root-finding you can call before inference**

---

**THE CHALLENGE: KEEP GPUS FOR WHAT THEY’RE BEST AT**

Many pipelines perform simple numeric checks (thresholds, feature rules, score cutoffs) before running a neural net. If those checks aren’t efficient—or they happen on the wrong hardware—they add latency and waste expensive GPU time.

**THE FIT: eROCK – A SMALL NUMERIC SERVICE ON THE CPU**

**eRock** is a lightweight Rust microservice that exposes two operations over HTTP:

- **Expression evaluation**: compute `y = f(x)` over numeric arrays (SIMD-friendly).  
- **Root-finding**: robust bisection (with auto-bracketing) to solve `f(t)=0` within a tolerance.

You call eRock from your data loader, stream processor, or web service to run quick numeric rules. Keep GPUs focused on model inference.

---

### Where teams use it

- **Pre-check gates:** compute simple scores or threshold rules on features you already extracted; only forward candidates to your CNN/ML service.  
- **Signal screening:** in a stream job, call eRock to evaluate boolean rules on recent measurements and drop obvious negatives.  
- **Parameter solves:** use `bisect` or `bisect_auto` to solve for a cutoff or boundary value in a formula before scheduling a heavier model step.  
- **Edge deployments:** on CPU-only gateways, run the same numeric rules close to the data source to save round-trips.

*(eRock evaluates your formulas on numeric inputs you provide; image decoding, feature extraction, and inference remain in your pipeline.)*

---

### Why use eRock

- **Small & deterministic:** bounded iterations/tolerances; predictable latency on CPU.  
- **Runs on x86/ARM:** deploy to gateways or servers.  
- **Simple integration:** JSON in/out over HTTP; package as a lightweight container.

---

### Technical Profile

- **Language:** Rust  
- **Operations:** array expression eval; bisection (manual or auto-bracket)  
- **Design:** stateless requests; SIMD-friendly evaluation  
- **Deployment:** Docker container (x86/ARM)

---

### Keep your GPUs focused

Use eRock for quick numeric decisions before inference.
