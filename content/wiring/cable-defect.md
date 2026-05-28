---
id: "cable-defect"
title: "Cable Defect"
category: "Wiring"
tags:
  - "Cable"
  - "PV failure"
  - "Insulation fault"
  - "DC wiring"
severity: High
impact_factor: >-
    Damaged or undersized DC cables can cause overheating, insulation faults, ground faults, production loss, and fire risk.
detection_method: >-
    Use visual inspection, insulation-resistance testing, thermal inspection, string current comparison, and fault localization on isolated circuits.
mitigation: >-
    Replace damaged cable runs, correct routing and strain relief, protect exposed wiring from UV and mechanical damage, and confirm insulation integrity after repair.
contributors:
  - name: Alexandre Mathieu
    url: https://github.com/AlexandreHugoMathieu
images:
  - path: /assets/page/cable_images/cable_defect.PNG
    caption: >-
      Figure 1: Corroded cable (left) and animal bites on cable (right) [4]
    alt: >-
      Figure 1: Corroded cable (left) and animal bites on cable (right) [4]
---

**Cables** are omnipresent over the PV array and enable the electrical connection between the PV array elements. Cable failures are among the **most severe** due to their frequency and impact on their performance [1].

This blog post deals with the causes, the apparition time and signature of cable failures.

# I.	Overview and Root causes

DC cables carry current from PV modules to the inverter. Depending on the size of the installation and the wiring architecture, they will be connected in parallel thanks to combiner boxes. The NF EN 50618 standard [2] especially defines single-core cables up to 1,500 V, for Class II equipment, capable of operating at a maximum core temperature of 90degree C for 25 years, including 20,000 hours at 120°C. 

Defects in DC cables can occur due to incorrect sizing during the design phase of the installation, or during the installation phase, or because of the exposure to harsh environments. As for connector defects, cable failures can lead to safety risks with potential risks of fires [3]. If cables are undersized or inadequate during the design phase, there might be an overheating risk. DC cables can also be damaged during installation due to pinching or deformation. Cable pinching can lead to insulation defects because the core might come into contact with the frame or mounting system. DC cables can also deteriorate if exposed to harsh environments such as UV radiation, rodents, or birds, which can also cause insulation defects. 
 


![Figure 1: Corroded cable (left) and animal bites on cable (right) [4]](/assets/page/cable_images/cable_defect.PNG)

*Figure 1: Corroded cable (left) and animal bites on cable (right) [4]*


# II. Apparition phase

The literature is scarce on the occurrence of DC cable failure from field data. Baschel et al. [5] outline from O&M reports, where no metadata information is disclosed due to confidentiality agreement, a failure rate of $0.002 \cdot 10^{-9} failures/hour$ while the deducted failure rate from Gallardo-Saavedra et al. [6], [7] is $0.845 \cdot 10^{-6} failures/hour$ which is quite a big difference. Also, Baschel et al. pinpoint a failure rate of $0.0483 \cdot 10^{-6} failures/hour$ for DC main cable. 

# III. Signature

Electrical insulation fault is one the main cable defects. This effect can be intermittent due to the humidity level conditions .  However, if the current leakage is lower than rated values from the protection devices, the fault might remain undetected and lead to significant power losses [7].  Damage cables can also lead electric arcs which might lead to fire.

Another defect consists in opening the wiring circuit and leads to significant power losses at the string level.

Early degradation with corrosion can also alter cable integrity and lead to an increase in serie resistance, causing higher power (and, then heat) dissipation [7].

Infrared images help to detect early symptoms while visual inspection enables to quickly scan cable defects.

**References**

[1]	A. Mathieu, G. Fraisse, M. Thebault, S. Thebault, S. Boddaert, and L. Gaillard, ‘Failure Risk Analysis of Photovoltaic Systems Based on Literature Review’, Kassel, Germany, Sep. 2022.

[2]	IEC, ‘IEC 50618:2014 Electric cables for photovoltaic systems’. Dec. 2014.

[3]	M. Herz, G. Friesen, U. Jahn, M. Köntges, S. Lindig, and D. Moser, ‘Quantification of Technical Risks in PV power Systems’, IEA PVPS, Technical Report IEA-PVPS T13-23:2021, Feb. 2022.

[4]	M. Köntges et al., ‘Review of Failures of Photovoltaic Modules’, IEA PVPS T13, Technical Report IEA-PVPS T13-01:2014, 2014.

[5]	S. Baschel, E. Koubli, J. Roy, and R. Gottschalg, ‘Impact of Component Reliability on Large Scale Photovoltaic Systems’ Performance’, Energies, vol. 11, no. 6, 2018, doi: 10.3390/en11061579.

[6]	S. Gallardo-Saavedra, L. Hernández-Callejo, and O. Duque-Pérez, ‘Quantitative failure rates and modes analysis in photovoltaic plants’, Energy, vol. 183, pp. 825–836, Sep. 2019, doi: 10.1016/J.ENERGY.2019.06.185.

[7]	E. Sarquis Filho, ‘Toward a Predictive Strategy for the Operation and Maintenance of PV Systems’, PhD Thesis, 2023.
