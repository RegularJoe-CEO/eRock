# Agent Integration — eRock Edge (Rust, Axum, SIMD)
Deterministic, stateless JSON API for ultra‑fast numeric expressions and root‑finding. Edge‑ready, low‑latency, SIMD‑accelerated.

Base URL: http://localhost:8080

## OpenAI-style function tools
```json
[
  {
    "type": "function",
    "function": {
      "name": "erock_evaluate",
      "description": "Vectorized evaluation y=f(x) over an array (SIMD lanes).",
      "parameters": {
        "type": "object",
        "required": ["expr", "x"],
        "properties": {
          "expr": { "type": "string", "description": "Assignment 'y = ...' optional." },
          "x": { "type": "array", "items": { "type": "number" } },
          "vars": { "type": "object", "additionalProperties": { "type": "number" } }
        }
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "erock_bisect",
      "description": "Root finding with a supplied bracket [lo, hi].",
      "parameters": {
        "type": "object",
        "required": ["expr", "lo", "hi"],
        "properties": {
          "expr": { "type": "string" },
          "lo": { "type": "number" },
          "hi": { "type": "number" },
          "vars": { "type": "object", "additionalProperties": { "type": "number" } },
          "tol": { "type": "number", "default": 1e-9 },
          "max_iter": { "type": "integer", "default": 60 }
        }
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "erock_bisect_auto",
      "description": "Auto‑bracket around a guess using exponential expansion, then bisection.",
      "parameters": {
        "type": "object",
        "required": ["expr", "guess"],
        "properties": {
          "expr": { "type": "string" },
          "guess": { "type": "number" },
          "step": { "type": "number", "default": 1.0 },
          "max_expand": { "type": "integer", "default": 20 },
          "vars": { "type": "object", "additionalProperties": { "type": "number" } },
          "tol": { "type": "number", "default": 1e-9 },
          "max_iter": { "type": "integer", "default": 60 }
        }
      }
    }
  },
  {
    "type": "function",
    "function": {
      "name": "erock_health",
      "description": "Health probe and version.",
      "parameters": { "type": "object", "properties": {} }
    }
  }
]
