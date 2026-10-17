# ZapFast: WhatsApp for Windows without OpenGL 2.1

Version 0.19.0-dx12.3. A personal compatibility fork for a Windows 11 Pro x64
PC with Intel N95, about 16 GB RAM, and Microsoft Basic Display Adapter.
AI assistance credit for this adaptation: GPT-6.1 Sol (medium).
Original ZapFast: Carmine Paolino and contributors, https://github.com/crmne/zapfast.
Fork: https://github.com/SSParzival/zapfast-whatsapp-windows-no-opengl21.

Open `zapfast.exe`. Windows renders through Direct3D 12 without requiring an
OpenGL 2.1 context. `start-software-rendering.cmd` forces CPU WARP rendering.

Link your phone through WhatsApp > Linked devices. If the QR expires, click
Retry. Recovery preserves the stored session and local archive. Account
authorization is completed on the phone.

The fork retains the original client's features and improves chat-list
copying and contact search. Official updates are disabled to preserve the
graphics adaptation. Update this variant manually from the fork repository.

Read `windows-compatibility.md`, `pairing-recovery.md`, and
`performance-windows.md` for changes, checks, and validation limits. QR receipt
and renewal were checked with a temporary profile in .2; this variant retains
that pairing code. Graphics and performance checks use synthetic data.
Real message delivery requires verification by the account owner.

`LICENSE` and `THIRD-PARTY-NOTICES.md` are preserved. This fork is independent
of the original project, WhatsApp, and Meta.
