# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Pre‑Process More, Burn Less

**Cut CPU cycles and energy use before your models fire**

---

**THE CHALLENGE: WASTED POWER BEFORE YOUR MODEL**

Deep learning stacks often spend more time preparing tensors than running the neural net. Python loops, JavaScript transforms and GPU pre‑processing all eat energy and slow your inference pipeline.

**THE FIT: eROCK – OFFLOAD THE MATH AND SAVE POWER**

`eRock` is a native Rust microservice that crunches numbers on the CPU using SIMD. It normalizes data, applies thresholds and solves for parameters faster than interpreted languages, freeing your GPUs and reducing the number of inference servers you need.

---

### Where it wins in AI/ML

- **Image and sensor normalization** – Evaluate brightness, scaling and simple transforms on raw arrays without GPU involvement; only call your model when needed.
- **Dynamic filtering and scoring** – Compute fast heuristics to discard 90% of candidates in your pipeline, saving GPU time and energy.
- **Feature engineering at the edge** – Generate features or boundary values for incoming data streams on CPU; keep your model pipelines lean.
- **Edge AI triggers** – Use `erock_bisect_auto` to solve for thresholds that decide when to wake up heavier ML models.

---

### Practical compute benefits

- **Higher throughput on the same hardware** – Low overhead evaluation means your servers handle more requests per second.
- **Lower inference cost** – Burn CPU cycles instead of GPU cycles; run fewer GPU instances while meeting SLA.
- **Colder datacenter racks** – Less CPU time per request means less heat and longer component life.

---

### Keep your ML stack lean and mean

With `eRock`, you can pre‑process data quickly on CPUs, leaving your GPUs for what they do best: heavy inference.
