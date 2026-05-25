---
id: flawed-pan-file-losses
title: Flawed PAN File Yield Losses
category: Modules
tags:
  - yield loss
severity: Medium
impact_factor: Incorrect or nonrepresentative PAN files can bias simulated module performance, creating yield expectations that differ from field behavior before any physical plant fault exists.
detection_method: Compare manufacturer and third-party PAN files, validate parameters against independent test data, review model sensitivity, and reconcile measured plant performance against the file used for financing or design.
mitigation: Use validated module files, document the selected PAN source and version, perform sensitivity analysis, and update forecasts when independent testing shows material parameter differences.
images: []
---

## Field Signature

PAN file losses are modeling losses rather than direct field faults. They occur when the module parameter file used in PVsyst or a related energy model does not represent the actual module behavior well enough for the decision being made.

The field symptom is a persistent gap between expected and measured performance after weather, availability, clipping, curtailment, soiling, degradation, and instrumentation issues have been reviewed. The plant may be operating normally while the baseline expectation is wrong.

## Loss Mechanisms

A PAN file encodes module electrical behavior, temperature response, low-light performance, IAM-related assumptions, and other parameters used to translate weather into expected power. If those parameters are optimistic, incomplete, or inconsistent with the delivered module population, modeled energy can be overstated.

Differences between manufacturer-produced and independently produced files can therefore affect procurement comparisons, financing assumptions, and post-construction variance analysis.

## Diagnostic Workflow

Start by identifying the exact PAN file used for the energy estimate, including source, version, and any manual edits. Compare it with manufacturer data sheets, independent lab measurements, third-party PAN files, and measured site behavior.

Run sensitivity cases to determine whether the observed production gap is plausibly explained by PAN parameters. Keep the comparison separate from true operational losses so a model-baseline issue is not mistaken for a repairable field defect.

## Engineering Notes

Treat PAN provenance as a controlled modeling input. The risk is not only that one file is wrong, but that teams compare designs or diagnose operations using inconsistent module representations.

## References

- Scott Meredith, ["Quantifying Performance Differences Between Manufacturer-produced and Third Party-produced PAN Files"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), Anza Renewables poster.
