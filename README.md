[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/embedded-lora-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/embedded-lora-rust)

# `embedded-lora`
`no-std`-compatible, opinionated drivers for some LoRa modems. Despite almost all modems supporting multiple radio
modulations, these drivers only implement LoRa. Furthermore, we currently only test the EU 868 MHz variants; other
variants may or may not work.

## Currently Implemented Drivers
- [embedded-lora-rfm95](./rfm95/README.md) for the popular RFM95 chips
