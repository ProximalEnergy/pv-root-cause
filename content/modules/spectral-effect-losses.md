---
id: spectral-effect-losses
title: Spectral Effect Yield Losses
category: Modules
tags:
  - yield loss
severity: Medium
impact_factor: Differences between the incident solar spectrum, the reference spectrum, and module spectral response can bias capacity tests and shift realized energy yield.
detection_method: Compare measured performance with spectral irradiance, air mass, precipitable water, module technology response, temperature-corrected output, and model assumptions.
mitigation: Apply appropriate spectral corrections in testing and modeling, use technology-specific spectral response data, and separate spectral effects from temperature, soiling, degradation, and irradiance sensor bias.
images: []
---

## Field Signature

Spectral effect losses occur when the light reaching the module differs from the reference spectrum used for rating, modeling, or capacity testing. The plant can appear to underperform or overperform after ordinary irradiance and temperature corrections if the module's spectral response does not match the prevailing spectrum.

The signature is often correlated with air mass, season, atmospheric water content, aerosol conditions, and module technology. It can be especially important when comparing technologies or evaluating tests performed during narrow weather windows.

## Loss Mechanisms

PV modules respond differently across wavelengths. When the available spectrum shifts toward wavelengths that a technology converts less efficiently, the effective irradiance at the cell is lower than a broadband irradiance measurement alone suggests.

For tandem devices, spectral effects can also interact with current matching between subcells, luminescent coupling, and temperature coefficients. That makes spectral assumptions part of the performance model rather than a minor correction.

## Diagnostic Workflow

Start by reviewing whether the capacity test or energy model applied a spectral correction. Compare residuals against air mass, precipitable water, aerosol optical depth, season, and time of day after removing clipping, curtailment, temperature, soiling, and availability effects.

Where available, use spectral irradiance measurements or validated spectral model inputs. Technology-specific spectral response data are necessary because the same atmospheric spectrum can produce different effective irradiance for different module designs.

## Engineering Notes

Do not use spectral correction as a catch-all explanation for unexplained underperformance. It should improve the structure of the residuals and be tied to measured or modeled atmospheric conditions.

## References

- Keith McIntosh, ["How spectral effects impact capacity testing - a case study"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), PV Lighthouse and Kiewit Engineering presentation.
- Phillip Hamer and Zihui Zhang, ["Impact of Spectrum, Luminescent Coupling, and Temperature Coefficient on Tandem Device Performance"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), University of New South Wales poster.
