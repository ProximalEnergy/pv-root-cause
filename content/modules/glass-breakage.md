---
id: "glass-breakage"
title: "Glass Breakage"
category: "Modules"
tags:
  - "Glass breakage"
  - "PV failure"
  - "insulation fault"
  - "module safety"
severity: High
impact_factor: >-
    Broken module glass can cause safety hazards, insulation faults, moisture ingress, and accelerated electrical degradation.
detection_method: >-
    Use visual inspection, insulation testing, electroluminescence, and operating data review to determine electrical and safety impact.
mitigation: >-
    Replace modules with broken glass when safety or insulation risk is present and investigate root causes such as clamp stress, hail, handling, or thermal damage.
contributors:
  - name: Alexandre Mathieu
    url: https://github.com/AlexandreHugoMathieu
images:
  - path: /assets/page/glassbreakage_images/intro.PNG
    caption: >-
      Figure 1: Glass breakage illustration [1]
    alt: >-
      Figure 1: Glass breakage illustration [1]
  - path: /assets/page/glassbreakage_images/stats.PNG
    caption: >-
      Figure 2: Failure frequency for PV module defects with an impact on the system [2]
    alt: >-
      Figure 2: Failure frequency for PV module defects with an impact on the system [2]
  - path: /assets/page/glassbreakage_images/manufacturing.PNG
    caption: >-
      Figure 3: Glass breakage due to weak manufacturing process, reported by Herz et al. [2] from [SUPSI]
    alt: >-
      Figure 3: Glass breakage due to weak manufacturing process, reported by Herz et al. [2] from [SUPSI]
  - path: /assets/page/glassbreakage_images/clamp.PNG
    caption: >-
      Figure 4: Glass breakage caused by too tight screws and poor clamp design [3]
    alt: >-
      Figure 4: Glass breakage caused by too tight screws and poor clamp design [3]
  - path: /assets/page/glassbreakage_images/hail.PNG
    caption: >-
      Figure 5: Module with broken glass after heavy hail storm [8]
    alt: >-
      Figure 5: Module with broken glass after heavy hail storm [8]
  - path: /assets/page/glassbreakage_images/burnmark.PNG
    caption: >-
      Figure 6: Glass breakage of tempered glass induced by burn mark [1]
    alt: >-
      Figure 6: Glass breakage of tempered glass induced by burn mark [1]
  - path: /assets/page/glassbreakage_images/electroluminescence.PNG
    caption: >-
      Figure 7: EL image of hail affected modules (left: almost intact, right: hardly damaged)
    alt: >-
      Figure 7: EL image of hail affected modules (left: almost intact, right: hardly damaged)
---

**Glass breakage** is a common PV module failure that impacts plant safety and performance, potentially leading to **insulation faults** that can stop the entire plant operation.


![Figure 1: Glass breakage illustration [1]](/assets/page/glassbreakage_images/intro.PNG)

*Figure 1: Glass breakage illustration [1]*


This blog post presents an overview on PV glass breakage: the root causes, signature, concomittant failures and detection methods.

## I. Apparition

Glass breakage is a recurrent failure with the highest share among the “sudden” failures in Herz et al. study [2] 


![Figure 2: Failure frequency for PV module defects with an impact on the system [2]](/assets/page/glassbreakage_images/stats.PNG)

*Figure 2: Failure frequency for PV module defects with an impact on the system [2]*


The front glass of PV modules is designed to protect the solar cells from the external environment while keeping a high optic transmission. However, this glass can break due to various causes:

- **Manufacturing defect:** Glass breakage can come from the manufacturing phase where incorrect lamination can result in higher mechanical stress on the module materials which increases the likelihood of glass breakage [3].


![Figure 3: Glass breakage due to weak manufacturing process, reported by Herz et al. [2] from [SUPSI]](/assets/page/glassbreakage_images/manufacturing.PNG)

*Figure 3: Glass breakage due to weak manufacturing process, reported by Herz et al. [2] from [SUPSI]*


- **Installation and Design Flaws**: Improper module clamping on frameless PV modules  can also lead to glass breakage [2], [3]. Excessive mechanical stress during the installation phase might lead to higher tension on the PV modules [3]. Additionally, transportation occasionally breaks PV module front glasses [3].


![Figure 4: Glass breakage caused by too tight screws and poor clamp design [3]](/assets/page/glassbreakage_images/clamp.PNG)

*Figure 4: Glass breakage caused by too tight screws and poor clamp design [3]*


- **External mechanical-stress events:** Glass breakages can occur from heavy impacts such as hail [4], stones, or other objects hitting the module surface [2]. Vandalism and animal activity, such as goats climbing on PV modules or birds dropping objects, can cause further damage. Some climates (cold [5] and arid [6]) are also more prone to glass breakage with, for instance, heavy snowfall, high winds, and sandstorms. Moreover, sandstorms can be abrasive to the front glass and weaken it [6]. Flying pebbles from cutting the grass are also mentioned as a reason for glass breakage [7].


![Figure 5: Module with broken glass after heavy hail storm [8]](/assets/page/glassbreakage_images/hail.PNG)

*Figure 5: Module with broken glass after heavy hail storm [8]*


- **Thermal Stress:** Glass breakage can result from high temperatures, such as burn marks [2], hot spots caused by shading or broken cell interconnects [2], [3], [7]. A high temperature gradient within the glass, especially in non-tempered glass, can increase the risk of cracking [7].


![Figure 6: Glass breakage of tempered glass induced by burn mark [1]](/assets/page/glassbreakage_images/burnmark.PNG)

*Figure 6: Glass breakage of tempered glass induced by burn mark [1]*


- **Mounting structure failure** In case the mounting structure has a defect, additional stress on the glass might break it [7]. 

Due to its structure, thin film technologies are particularly subject to glass breakage [4], [9], [10]. 

Jordan et al. [10] highlight a glass breakage occurrence in the order of 2% for moderate climate. The IEA report [3] shows a 10% share in all PV module failures. Following the reasoning from Sarquis Filho [11], glass breakage has a 10.8% occurrence from the solar bankability project [12], which would correspond to 7 $10^{-9}$ failure/unit/hour if we consider a generic 0.065 $10^{-6}$ failure/unit/hour

## II. Signatures

Glass breakage events lead to a loss of 1% to 2% for temperate climates in Herz et al. study [2].  

Glass breakage typically leads to a partial loss of transmittance [11]. However, the most severe production losses come from the compromised integrity of the module due to moisture ingress and insulation defect.  

In the short term, glass breakage is usually accompanied by cell cracks, reducing nominal power if significant parts are inactive. These can cause hotspots [3], [4], [13] and might eventually lead to fire [4]. Additionally, glass breakage might lead to ground faults [3], [4], [7], [11]. This effect can be intermittent with lower insulation or short circuits during rainy days [7], [11].  

In the long term, broken glass no longer serves as an effective barrier against the environment and moisture, leading to accelerated degradation of the encapsulant and other components [13]. Specifically, moisture ingress accelerates the corrosion development in cell interconnections [3], [4], [7], [11]. 

For c-Si modules, performance might remain at the same level immediately after glass breakage, as the cells, encapsulation, and wiring can still be intact [13].

## III. Concomitant Failures

Glass breakage in PV modules can be triggered by localized heat dissipation, such as hotspots and burn marks caused by cell interconnection defects, which induce thermal stress [3], [4]. Additionally, failures in the junction box can result in resistive heating, which further contributes to glass breakage [3]. Mounting structures can also inflict damage on the glass [3].  

Glass breakage often occurs alongside other failures within the PV module. For instance, breakage can be accompanied by cell cracks, which not only weaken the module's structure but also create new hotspots [3], [4], [13].  

Glass breakage can lead to further complications, including insulation failure, which compromises the safety and operational integrity of the module [3]. The exposure of cells to environmental factors like moisture due to glass breakage accelerates degradation, leading to further hotspots, performance losses, and increased risk of electrical faults or fires.

## IV. Detection & Reparation

Visual inspection, infrared imaging, and monitoring tools are methods for detecting glass breakage in PV modules. Visual inspection [2], [14] is the simplest and most efficient way to detect glass breakages. Infrared imaging [2], [14] detects temperature anomalies associated with glass breakage. Electroluminescence and UV-fluorescence can also identify cell cracks caused by glass breakage. Additionally, monitoring tools that track electrical performance can signal performance losses from glass breakage [14].


![Figure 7: EL image of hail affected modules (left: almost intact, right: hardly damaged)](/assets/page/glassbreakage_images/electroluminescence.PNG)

*Figure 7: EL image of hail affected modules (left: almost intact, right: hardly damaged)*


Modules with broken glass need to be replaced [11]. For minor cracks, resealing may help temporarily [4]. 


## References

[1] M. Köntges, G. Oreski, U. Jahn, M. Herz, P. Hacke, and K.-A. Weiss, ‘Assessment of photovoltaic module failures in the field’, IEA PVPS, IEA-PVPS T13-09:2017, 2017.  

[2] M. Herz, G. Friesen, U. Jahn, M. Köntges, S. Lindig, and D. Moser, ‘Quantification of Technical Risks in PV power Systems’, IEA PVPS, Technical Report IEA-PVPS T13-23:2021, Feb. 2022.  

[3] M. Köntges et al., ‘Review of Failures of Photovoltaic Modules’, IEA PVPS T13, Technical Report IEA-PVPS T13-01:2014, 2014.  

[4] C. Miquel, C. Stravrou, N. Lebert, and J. Sarantou, ‘Dysfonctionnement électriques des installations photovoltaïques: points de vigilance.’, AQC - HESPUL, Technical Report PTVIGI1801, Oct. 2018.  

[5] A. Omazic et al., ‘Relation between degradation of polymeric components in crystalline silicon PV module and climatic conditions: A literature review’, Sol. Energy Mater. Sol. Cells, vol. 192, pp. 123–133, 2019, doi: 10.1016/j.solmat.2018.12.027.  

[6] M. W. Akram, G. Li, Y. Jin, and X. Chen, ‘Failures of Photovoltaic modules and their Detection: A Review’, Appl. Energy, vol. 313, p. 118822, 2022, doi: 10.1016/j.apenergy.2022.118822.  

[7] GovindaSamy TamizhMani and Joseph Kuitche, ‘Accelerated lifetime testing of photovoltaic modules’, Solar America Board for Codes and Standards, Arizona, USA, Jul. 2013.  

[8] W. Muehleisen et al., ‘Outdoor detection and visualization of hailstorm damages of photovoltaic plants’, Renew. Energy, vol. 118, pp. 138–145, 2018, doi: 10.1016/j.renene.2017.11.010.  

[9] N. Bansal, S. P. Jaiswal, and G. Singh, ‘Comparative investigation of performance evaluation, degradation causes, impact and corrective measures for ground mount and rooftop solar PV plants – A review’, Sustain. Energy Technol. Assess., vol. 47, p. 101526, Oct. 2021, doi: 10.1016/j.seta.2021.101526.  

[10] D. C. Jordan, T. J. Silverman, J. H. Wohlgemuth, S. R. Kurtz, and K. T. VanSant, ‘Photovoltaic failure and degradation modes’, Prog. Photovolt. Res. Appl., vol. 25, no. 4, pp. 318–326, 2017, doi: 10.1002/pip.2866.  

[11] E. Sarquis Filho, ‘Toward a Predictive Strategy for the Operation and Maintenance of PV Systems’, PhD Thesis, 2023.  

[12] David Moser et al., ‘Technical Risks in PV Projects Report on Technical Risks in PV Project Development and PV Plant Operation’, Solar Bankability, Feb. 2017.  

[13] M. Aghaei et al., ‘Review of degradation and failure phenomena in photovoltaic modules’, Renew. Sustain. Energy Rev., vol. 159, p. 112160, 2022, doi: 10.1016/j.rser.2022.112160.  

[14] C. Miquel, C. Stravrou, N. Lebert, and J. Sarantou, ‘Méthodes de détection des dysfonctionnements électriques des installations photovoltaïques.’, AQC - HESPUL, Technical Report ETUC2P1901, Jun. 2019.
