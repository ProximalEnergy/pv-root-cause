---
id: inverter-voltage-floor-losses
title: Inverter Voltage Floor Losses
category: Inverters
tags:
  - yield loss
severity: Medium
impact_factor: When array Vmp falls below the inverter minimum DC input voltage, the inverter cannot hold the array at its maximum power point and recoverable energy can be left on the DC side.
detection_method: Compare inverter DC voltage limits, AC voltage, reactive power dispatch, module temperature, in-situ IV curve Vmp/Pmp, and SCADA operating points during suspected voltage-floor events.
mitigation: Verify inverter settings and AC voltage behavior, coordinate transformer tap or voltage setpoint changes, validate reactive power requirements, and retest MPPT recovery under high-temperature operating conditions.
images:
  - path: /assets/inverters/voltage-floor.png
    caption: DC voltage versus AC voltage scatter plot isolating inverter voltage floor events and zooming in on the constrained operating band.
    alt: Scatter plots showing inverter voltage floor events as red points near the lower DC voltage operating boundary.
---

## Field Signature

Inverter voltage floor losses occur when the PV array wants to operate at a DC maximum power point voltage below the inverter's minimum usable input voltage. The inverter remains online, but it cannot track the true array Vmp. The result is a gap between the power available from the array and the power actually harvested at the inverter operating point.

The condition is most likely to appear during hot operating periods because module voltage falls as cell temperature rises. It can also be amplified when grid or plant requirements increase reactive power demand and shift the inverter's effective voltage behavior. The observed production symptom can look like clipping or generic underperformance unless DC voltage, AC voltage, reactive power, and IV-derived Vmp are reviewed together.

## Loss Mechanisms

The inverter's DC input voltage window acts as a hard operating constraint. When degraded modules, high temperature, string design margin, or reactive power operation push array Vmp below that constraint, the inverter is forced to operate above the array maximum power point. In that state, available DC power is not fully converted even though irradiance and inverter availability may look normal.

The Leeward Renewable Energy and OTT HydroMet poster "Quantifying Power Losses from Inverter Voltage Floor Limitations" reports field work at a 100 MW California PV plant beginning in spring 2025. The poster describes summer 2025 average energy losses of about 1.2% from inverter voltage flooring, with voltage or setting adjustments estimated to recover a large share of the measured loss.

## Diagnostic Workflow

Start by identifying intervals where inverter DC voltage is close to or pinned by the minimum operating threshold while irradiance-supported output is lower than expected. Filter out unrelated outages, curtailment, clipping, soiling, and sensor faults before assigning the event to voltage flooring.

Compare the inverter operating voltage against an independent estimate of array Vmp. In-situ IV tracers, string-level IV sweeps, or a temperature-corrected module model can show where the array wants to operate. A voltage-floor event is supported when measured operating voltage stays above the IV-derived Vmp and the corresponding operating power is below IV-derived Pmp.

Review AC voltage and reactive power dispatch at the same timestamps. High reactive power demand or AC voltage hysteresis can change the effective inverter operating window, so the root cause may involve plant voltage control, transformer tap position, inverter settings, or grid support requirements rather than module degradation alone.

## Engineering Notes

Do not treat this as ordinary inverter clipping. Clipping is a power ceiling; voltage flooring is an MPPT constraint caused by the DC voltage window. The two can occur in different weather windows and require different mitigations.

Mitigation should be validated with the inverter supplier and plant electrical engineer. Candidate actions include transformer tap adjustment, inverter parameter changes, revised voltage control strategy, or future string design changes that preserve low-temperature and high-temperature voltage margin. After changes are made, compare recovered operation against IV-derived Vmp/Pmp and confirm that reactive power obligations are still met.

## References

- Sha Li, Brett Pendleton, Michael Gostein, and Josh Horst, ["Quantifying Power Losses from Inverter Voltage Floor Limitations"](https://pvpmc.sandia.gov/2026-pvpmc-albuquerque/), Leeward Renewable Energy and OTT HydroMet poster.
