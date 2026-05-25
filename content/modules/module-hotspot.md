---
id: module-hotspot
title: Module Hotspot from Cell Damage
category: Modules
tags:
  - drone 
  - yield loss
severity: High
impact_factor: Localized heating can reduce string output and create escalating reliability risk when bypass diodes or cracked cells remain in service.
detection_method: Drone infrared inspection under stable irradiance, followed by IV curve comparison and visual confirmation at the affected module.
mitigation: Isolate the string, verify the thermal signature under load, replace the affected module when cell damage is confirmed, and retest string current after repair.
images:
  - path: /assets/modules/hotspot-ir.svg
    caption: Representative infrared inspection frame showing a localized cell hotspot within one module.
    alt: Simplified infrared module diagram with one hot cell.
---

## Field Signature

Module hotspots appear as localized high-temperature cells or substrings during loaded operation. A persistent hotspot is commonly associated with cracked cells, solder bond defects, soiling shadows, bypass diode behavior, or mismatch that forces part of the module into reverse bias.

## Diagnostic Workflow

Perform the infrared scan during clear, stable conditions with the inverter operating and irradiance high enough to make thermal contrast meaningful. Compare the affected module against adjacent modules in the same string, then confirm with string current, IV curve shape, and visual inspection.

## Engineering Notes

A single thermal anomaly does not prove permanent module damage. Treat the image as a screening result, then verify electrical impact and repeatability before approving replacement. Escalate priority when the anomaly is stable across inspections or when multiple modules in the same string show related signatures.
