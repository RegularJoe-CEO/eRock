# SPDX-FileCopyrightText: 2025 Eric Waller
# SPDX-License-Identifier: LicenseRef-eRock-Business-1.0

import erock_clean as erock
import json

def evaluate(expr, variables={}):
    vars_dict = {k: list(v) for k, v in variables.items()}
    result = erock.eval(expr, vars_dict)
    return result if isinstance(result, float) else result  # Err str if fail

print(evaluate("2 + 3 * 4"))  # 14.0
print(evaluate("x + y", {"x": [5.0], "y": [10.0]}))  # 15.0
print(evaluate("x + y", {"x": [1.0, 2.0], "y": [3.0, 4.0]}))  # 4.0 (first)
vars_after = {}
result = evaluate("z = 7 + 3", vars_after)
print(result, "z in vars:", "z" in vars_after)  # 10.0 True
