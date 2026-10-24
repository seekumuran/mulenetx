# PSP Pentest Device

> A handheld embedded-systems project inspired by the PlayStation Portable form factor, exploring portable cybersecurity tooling for authorized security testing. This README covers the design intent and open questions. Implementation status, hardware specifications, and test results are marked `[TBD]` until confirmed.

> **Disclaimer:** This is an independent project. It is not affiliated with, endorsed by, or derived from Sony or the PlayStation Portable, and it does not use any proprietary PSP technology.

## Contents

1. [Project Overview](#1-project-overview)
2. [Implementation Status](#2-implementation-status)
3. [Hardware Architecture](#3-hardware-architecture)
4. [Software Architecture](#4-software-architecture)
5. [Security Tools](#5-security-tools)
6. [User Interface and Controls](#6-user-interface-and-controls)
7. [Networking and System Integration](#7-networking-and-system-integration)
8. [Installation and Usage](#8-installation-and-usage)
9. [Responsible Use](#9-responsible-use)
10. [Testing and Validation](#10-testing-and-validation)
11. [Repository Structure](#11-repository-structure)
12. [Engineering Challenges and Future Scope](#12-engineering-challenges-and-future-scope)
13. [Credits and License](#13-credits-and-license)

## 1. Project Overview

This project explores building a compact handheld device that combines an embedded platform, a physical control interface, and a set of security utilities for systems the user owns or is authorized to test.

Areas of exploration:

- Portable cybersecurity tooling `[TBD: scope]`.
- Embedded hardware and system integration `[TBD: platform]`.
- A handheld user interface with physical controls.
- Network diagnostics and security assessment on authorized networks.
- Modular tools that can be added or removed independently.

## 2. Implementation Status

| Area | Status |
|---|---|
| Hardware enclosure and layout | `[TBD]` |
| Processing platform | `[TBD]` |
| Display and controls | `[TBD]` |
| Power and charging | `[TBD]` |
| Operating system or firmware | `[TBD]` |
| Main menu and navigation | `[TBD]` |
| Security tools | `[TBD]`, see Section 5 |
| Networking | `[TBD]` |
| Tested end to end | Not yet performed |

## 3. Hardware Architecture

| Component | Role | Model or part | Key specifications | Status |
|---|---|---|---|---|
| Enclosure | Handheld housing | `[TBD]` | `[TBD]` | `[TBD]` |
| Processing platform | Runs OS and tools | `[TBD]` | `[TBD]` | `[TBD]` |
| Display | Output to user | `[TBD]` | `[TBD]` | `[TBD]` |
| Input controls | Navigation and selection | `[TBD]` | `[TBD]` | `[TBD]` |
| Battery and charging circuit | Power | `[TBD]` | `[TBD]` | `[TBD]` |
| Wireless adapter | Network access | `[TBD]` | `[TBD]` | `[TBD]` |
| Storage | Logs and configuration | `[TBD]` | `[TBD]` | `[TBD]` |
| External interfaces | Peripherals | `[TBD]` | `[TBD]` | `[TBD]` |

## 4. Software Architecture

```text
Boot -> OS or firmware -> main interface -> tool modules -> logging / reports
                                 |
                       network interface layer
```

This is a proposed layout. Confirm each component before relying on it.

| Area | Status | Notes |
|---|---|---|
| Operating system or firmware | `[TBD]` | |
| Main interface | `[TBD]` | |
| Tool modules | `[TBD]` | See Section 5 |
| Logging and reports | `[TBD]` | |
| Configuration management | `[TBD]` | |

## 5. Security Tools

Tools are listed by status. Nothing in this table is confirmed as working until it has been tested.

| Tool | Purpose | Inputs | Outputs | Status |
|---|---|---|---|---|
| Network discovery | Identify hosts on a authorized network | `[TBD]` | `[TBD]` | Planned |
| Port and service audit | Check open services on owned systems | `[TBD]` | `[TBD]` | Planned |
| Local network configuration inspection | Review interface and route settings | `[TBD]` | `[TBD]` | Planned |
| Wireless security assessment | Review settings on owned networks | `[TBD]` | `[TBD]` | Planned |
| Packet inspection | Analyse traffic on an authorized test network | `[TBD]` | `[TBD]` | Planned |
| Cryptographic utilities | Hashing and encoding helpers | `[TBD]` | `[TBD]` | Planned |
| Report generation | Summarise findings | `[TBD]` | `[TBD]` | Planned |

### Scope Limits

- The device does not perform attacks, bypass authentication, or disrupt services.
- No tool is designed to collect credentials from third parties.
- Tools are for systems the user owns or has written permission to test.

## 6. User Interface and Controls

| Element | Design | Status |
|---|---|---|
| Main menu | Tool selection list | Conceptual |
| Navigation | Directional pad and select / back buttons `[TBD: confirm layout]` | Conceptual |
| Status screen | Connection and battery status | Conceptual |
| Output screen | Tool results, scrollable | Conceptual |
| Configuration | Network and device settings | Conceptual |
| Logs | Viewable and exportable records | Conceptual |
| Error feedback | Message with a recovery action | Conceptual |

These layouts are design intent. Update each row once the interface is built and tested.

## 7. Networking and System Integration

- **Wi-Fi:** `[TBD: adapter, supported modes, and driver]`
- **Ethernet:** `[TBD: if supported]`
- **Network configuration:** `[TBD: how interfaces are set up]`
- **Tool execution:** `[TBD: how tools are launched and how output is captured]`
- **Local storage:** `[TBD: where logs and results are kept]`
- **Permissions:** `[TBD: which operations need elevated privileges]`
- **Troubleshooting:** `[TBD: common connectivity failures and fixes]`

## 8. Installation and Usage

No installation commands are documented yet. They will be added once the platform and dependencies are confirmed.

- **Hardware requirements:** `[TBD]`
- **Software prerequisites:** `[TBD]`
- **Installation steps:** `[TBD]`
- **Startup procedure:** `[TBD]`
- **Usage examples:** `[TBD: only for tools that have been tested]`

**Lab guidance:** test on an isolated network that you own, with a known list of devices and written permission for any system you assess.

## 9. Responsible Use

- Use the device only on systems you own or have explicit written permission to assess.
- Test on isolated lab networks where possible.
- Store credentials, logs, and captured data securely, and delete them when no longer needed.
- Avoid collecting personal data beyond what a test requires.
- Do not scan or test third-party systems without authorization.
- Report findings to the system owner responsibly.

## 10. Testing and Validation

| Test | Method | Result | Status |
|---|---|---|---|
| Boot and interface | `[TBD]` | `[TBD]` | Not yet performed |
| Input responsiveness | `[TBD]` | `[TBD]` | Not yet performed |
| Network connectivity | `[TBD]` | `[TBD]` | Not yet performed |
| Individual tool behaviour | `[TBD]` | `[TBD]` | Not yet performed |
| Error handling | `[TBD]` | `[TBD]` | Not yet performed |
| Battery life | `[TBD]` | `[TBD]` | Not yet performed |
| Extended-use stability | `[TBD]` | `[TBD]` | Not yet performed |

No benchmark, security finding, or successful test result is reported in this document.

## 11. Repository Structure

Proposed layout, not yet confirmed:

```text
psp-pentest-device/
├── docs/
│   └── images/      # photos, diagrams, interface mockups
├── hardware/        # enclosure, wiring, power design
├── firmware/        # OS image or embedded code [TBD]
├── tools/           # security modules [TBD]
├── ui/              # interface assets and layouts
├── config/          # example configuration (no secrets)
├── tests/           # test procedures and logs
└── README.md
```

## 12. Engineering Challenges and Future Scope

### Engineering Challenges
- Limited battery capacity and power budget for a handheld device.
- Fitting a usable display and controls into a small enclosure.
- Managing software dependencies on an embedded platform.
- Network compatibility across different wireless adapters and configurations.
- Keeping long-running tools reliable on constrained hardware.

### Future Work (not implemented)
- Additional authorized assessment modules.
- Improved report generation.
- A more polished interface.
- Better power management.
- Modular hardware expansion.

## 13. Credits and License

**Upstream projects:** `[TBD: list only projects actually used]`

**License:** `[TBD: to be confirmed by the author]`
