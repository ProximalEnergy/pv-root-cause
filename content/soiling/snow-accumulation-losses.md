---
id: snow-accumulation-losses
title: Snow Accumulation Yield Losses
category: Soiling
tags:
  - yield loss
severity: Medium
impact_factor: Snow cover blocks irradiance at the module surface, creates partial-string mismatch during melt and slide-off, and can materially reduce winter energy yield in cold regions.
detection_method: Compare production residuals with snowfall, snow depth, plane-of-array irradiance, module temperature, camera observations, and regional snow-loss model output.
mitigation: Use validated snow-loss assumptions in the energy model, track winter residuals separately from generic soiling, and evaluate tracker stow, tilt, maintenance, or design changes where economic.
images: []
---

## Field Signature

Snow accumulation losses occur when snow remains on the module surface after a snow event. The plant may be available and irradiance sensors may report usable resource, but covered modules cannot convert that irradiance into current.

The production signature can be abrupt after snowfall and gradual during melt or slide-off. Partial clearing can create uneven current across strings and substrings, so the loss is not always proportional to the visible snow-covered area.

## Loss Mechanisms

Uniform snow cover creates direct irradiance blocking. Patchy snow creates electrical mismatch because clear cells and covered cells operate at different current levels. Low sun angles, cold module temperatures, module frame geometry, portrait versus landscape orientation, and tracker position can all affect how long snow remains on the array.

Because snow losses are seasonal, they can be hidden inside monthly weather variance unless snowfall and recovery intervals are explicitly separated from other winter underperformance drivers.

## Diagnostic Workflow

Start with timestamped weather data: precipitation type, snowfall amount, snow depth, ambient temperature, wind, and irradiance. Align those data with SCADA production residuals and any camera, drone, or field observations of module cover.

Compare measured recovery curves against the snow-loss assumptions in the energy model. A site that clears more slowly than modeled may need a revised snow-loss factor even if the equipment is operating normally.

## Engineering Notes

Do not classify all winter residuals as snow. Cold-weather inverter behavior, clipping, curtailment, frozen tracker stow positions, sensor icing, and albedo effects can overlap with snow events and should be filtered before estimating recoverable loss.

## References

- Josh Peterson, ["PV Snow Losses for the Contiguous U.S.: Gridded Models with Ground-Based Validation"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), GroundWork Renewables poster.
