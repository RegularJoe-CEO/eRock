# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

### eRock: Fast Data Triage with Fewer Watts

**Cut through the noise quickly and conserve compute resources**

---

**THE CHALLENGE: HUGE DATA STREAMS BURNING POWER**

Particle detectors, telescopes and sequencing machines pump out terabytes of numbers. Filtering this torrent with Python loops or offline batch jobs wastes CPU cycles and delays analysis, all while burning through compute budgets.

**THE FIT: eROCK – LOW‑POWER TRIAGE**

`eRock` is built in Rust with SIMD acceleration. It can apply user‑defined filters and solve for crossing points on streams of numeric data faster than general‑purpose languages. Because it consumes far less CPU time per element, your compute nodes run cooler and your cluster budgets stretch further.

---

### Where it wins in HPC and science

- **Pre‑trigger filtering** – Evaluate conditions on high‑rate data before writing to disk, preserving only interesting events with minimal CPU load.
- **Genomics and bioinformatics** – Quickly compute quality thresholds and simple scores on sequencing reads; reduce CPU hours for initial filtering.
- **Astronomy & signal processing** – Apply threshold logic and root solves on FFT bins or radiometric measurements without GPU overhead.
- **Climate and simulation ingest** – Clean and normalize sensor feeds on the fly so your models start with pre‑filtered data.

---

### Practical research benefits

- **More experiments per cycle** – Save compute cycles on filtering and channel them into deeper analysis.
- **Smaller HPC clusters** – When front‑end filtering is cheaper, you need fewer nodes to handle ingestion.
- **Predictable time and power per event** – Deterministic loops let you allocate compute resources accurately.

---

### Focus your compute on the hard problems

`eRock` lets you triage massive data streams quickly and with minimal power, so the heavy science gets more CPU time.
