---
id: system-vs-module-degradation
title: System Degradation vs Module Degradation
category: Modules
tags:
  - degradation
  - yield loss
severity: Medium
impact_factor: System-level degradation can be higher than the nameplate module degradation rate when modules degrade at different rates and create additional electrical mismatch.
detection_method: Compare long-term normalized plant output, string or inverter trends, IV curve data, and module test results to separate average module degradation from mismatch-driven system degradation.
mitigation: Model degradation at the system level, account for module-to-module spread, and use field measurements to update long-term energy assumptions.
images: []
---

## Field Signature

Module degradation describes how an individual module loses power over time. System degradation describes how the full PV system loses energy over time.

Those are not always the same number. If every module degraded at exactly the same rate, system degradation would closely follow module degradation. In real plants, modules age at different rates. That spread creates mismatch because weaker modules can limit the current of strings or substrings around them.

## Loss Mechanisms

The average module may still be within warranty while the system shows a larger energy decline. The extra loss comes from the unevenness of the module population, not only from the average loss in module nameplate power.

This matters when translating a module degradation guarantee into an energy forecast. A plant is an electrical network, so variation between modules can reduce delivered energy beyond the average module degradation rate.

## Modeling Notes

Remember to model system-level degradation losses in the performance model, not just module-level degradation. The system loss should account for mismatch created as modules degrade at different rates.

Some independent engineers add about 0.1% per year to the module degradation rate when evaluating system degradation. The adjustment is meant to account for mismatch from modules degrading at different rates.

Treat that 0.1% as a modeling allowance, not a universal rule. Use site measurements, module technology, warranty terms, and IV or string-level data when available.
