---
id: granular-dust-soiling-losses
title: Granular Dust Soiling Losses
category: Soiling
tags:
  - yield loss
severity: Medium
impact_factor: Dust deposited on the module glass reduces transmittance, depresses array current, and can accumulate into persistent energy loss between rain or cleaning events.
detection_method: Compare performance ratio trends, soiling station data, rain and cleaning records, irradiance-normalized output, and model residuals for dry accumulation periods.
mitigation: Calibrate the soiling model with operational data, optimize cleaning triggers by lost-energy value, and separate dust-driven loss from sensor drift, clipping, outages, and curtailment.
images: []
---

## Field Signature

Granular dust soiling appears as a gradual decline in irradiance-normalized production during dry periods, often followed by partial or full recovery after meaningful rain or washing. The plant remains online, but less light reaches the cells.

The effect is usually broad across an affected zone, although roads, exposed soil, agricultural activity, construction, prevailing wind, and row edge conditions can make the loss spatially uneven.

## Loss Mechanisms

Dust particles scatter and absorb incoming light before it reaches the photovoltaic cell. As deposition increases, module current falls and plant output declines. Nonuniform deposition can also introduce mismatch when strings or module substrings receive different effective irradiance.

Operational soiling models can overstate or understate the loss when local particle composition, cleaning history, rain thresholds, and sensor maintenance are not represented correctly.

## Diagnostic Workflow

Start with normalized production trends and identify dry periods where output declines without corresponding availability, clipping, curtailment, or irradiance sensor issues. Compare the trend with soiling station measurements, rainfall, cleaning logs, and nearby reference blocks when available.

Then test whether the assumed soiling model explains the observed recovery after rain or cleaning. If the modeled rate does not match operations, update the local accumulation and wash-off assumptions before using the model for cleaning economics.

## Engineering Notes

Do not rely on a single dirty irradiance sensor to quantify dust soiling. Sensor drift, poor cleaning practice, and local obstruction can create apparent soiling signatures that do not represent the array.

## References

- Umay Akkoseoglu, ["Evaluation of Dust Soiling Models Against Operational Data"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), DNV poster.
