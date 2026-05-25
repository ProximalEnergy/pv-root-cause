---
id: grid-curtailment-losses
title: Grid Curtailment Yield Losses
category: Grid
tags:
  - yield loss
severity: High
impact_factor: Curtailment limits export below available plant power, reducing realized energy and complicating performance metrics when availability and equipment health remain normal.
detection_method: Compare plant setpoints, inverter active power limits, grid operator dispatch, meter output, irradiance-supported available power, BESS charging behavior, and curtailment flags.
mitigation: Track curtailment separately from equipment loss, improve available-power estimation, tune BESS charging strategy where applicable, and use performance metrics designed for heavily curtailed plants.
images: []
---

## Field Signature

Grid curtailment losses occur when the plant is capable of producing more power than it is allowed to export. The limitation may come from a grid operator dispatch, interconnection constraint, plant controller command, negative pricing response, or site-level export cap.

The production signature is a flat or controlled output level below irradiance-supported available power. Inverter and tracker availability may look normal, which makes curtailment easy to misclassify as underperformance unless setpoints and dispatch signals are reviewed.

## Loss Mechanisms

Curtailment turns available solar resource into unused energy. In PV-plus-storage plants, some curtailed energy may be recoverable through DC-coupled or AC-coupled battery charging, but only if the battery has capacity, power headroom, controls permission, and an operating objective that values the charge.

Heavy curtailment also distorts conventional performance metrics. A low performance ratio during curtailed hours may say more about grid instructions than about module, inverter, or tracker health.

## Diagnostic Workflow

Start by reconstructing available power from irradiance, temperature, inverter status, clipping state, and modeled plant capability. Compare that estimate with meter output and plant controller limits.

Then classify each constrained interval by cause: grid dispatch, export limit, economic curtailment, plant controller behavior, BESS charging limit, outage, or telemetry error. For storage sites, track whether curtailed PV was charged, clipped, or abandoned.

## Engineering Notes

Use curtailment-aware KPIs when a plant is frequently constrained. Otherwise, teams may spend time chasing false equipment problems while missing real opportunities in dispatch, controls, or storage utilization.

## References

- Andres Fernandez, ["Sizing DC-Coupled Utility-Scale BESS Using a Charging Reliability Metric Under High Curtailment Conditions"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), Ingenieria Creativa - ICREA presentation.
- Gofran Chowdhury, ["Reliable and Actionable Performance Metrics for Heavily Curtailed Plants"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), 3E presentation.
