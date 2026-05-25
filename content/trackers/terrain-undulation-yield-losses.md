---
id: terrain-undulation-yield-losses
title: Terrain Undulation Yield Losses
category: Trackers
tags:
  - yield loss
severity: Medium
impact_factor: Rolling terrain changes tracker row geometry, backtracking behavior, and inter-row shading, which can leave recoverable energy losses concentrated in specific motor blocks.
detection_method: Compare terrain-following design data, tracker motor block boundaries, backtracking settings, shade-model output, and normalized row or inverter production across morning and afternoon windows.
mitigation: Refine backtracking parameters by terrain zone, validate motor block sizing assumptions, correct grading or tracker geometry where practical, and update energy models with measured row-level behavior.
images: []
---

## Field Signature

Terrain undulation losses appear when local grade changes make the actual tracker field differ from the simplified geometry assumed in backtracking and energy-yield models. Rows may be mechanically available and following commands, but the terrain causes uneven apparent row spacing, nonuniform tracker angles, or locally excessive shading.

The production signature is usually repeatable by sun position. Losses tend to be strongest during low sun-angle periods and can cluster around motor block boundaries, grade breaks, swales, and rolling sections of the array that depart from the model's representative slope.

## Loss Mechanisms

Single-axis backtracking assumes a geometric relationship between row pitch, tracker angle, module plane, and the horizon created by adjacent rows. Terrain undulations perturb that geometry. A row that is correctly backtracking under a flat or average-slope assumption may still shade an adjacent row, or may be driven farther off sun than necessary to avoid modeled shade.

Motor block size matters because one actuator or control group may span several terrain conditions. Larger blocks can force one command angle across rows that would benefit from different backtracking positions, increasing either shading loss or cosine loss.

## Diagnostic Workflow

Start by mapping underperformance against tracker block IDs, row locations, and time of day. Compare affected zones with topographic survey data, construction as-builts, and the terrain representation used in the original energy model.

Then isolate backtracking intervals from curtailment, clipping, outages, and inverter constraints. Drone imagery, row-end angle surveys, tracker command logs, and string or inverter normalized output can show whether losses align with grade breaks or motor block boundaries rather than equipment failure.

## Engineering Notes

Do not assume that a tracker is performing correctly just because it follows its commanded angle. For undulating terrain, the command itself may be the source of yield loss if the terrain model, backtracking algorithm, or motor block layout is too coarse.

## References

- Umay Akkoseoglu, ["A Workflow to Analyze the Impact of Terrain Undulations on Energy Yield for Different Motor Block Sizes"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), DNV presentation.
- Billy Hayes, ["Impacts on Backtracking Energy Generation from Underlying Terrain Undulations and Varying Motor Block Size"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), Array Technologies, Inc. poster.
