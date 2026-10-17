# ZapFast: WhatsApp for Windows without OpenGL 2.1

**A native, unofficial WhatsApp client for Windows 11, adapted to work with
Microsoft Basic Display Adapter.**

**Main change:** the Windows renderer uses **Direct3D 12/WARP**, removing the
base version's requirement for an **OpenGL 2.1** context. WARP can render on
the CPU when the installed display driver cannot provide that OpenGL support.

This is a small personal fork of [ZapFast](https://github.com/crmne/zapfast),
prepared and tested for the PC described below.

**AI assistance credit for this adaptation: GPT-6.1 Sol (medium).**
The original client is by Carmine Paolino and the ZapFast contributors.

Tested PC: Windows 11 Pro x64, Intel N95, about 16 GB RAM, Microsoft Basic
Display Adapter, and a 1280x800 desktop at 100% scaling. Other computers and
operating systems need their own validation.

The validated version is **0.19.0-dx12.3**. It also improves recovery after QR
expiration, chat-list copying, and contact search. It retains the original
protocol, encrypted archive, and application features.

**[Download Windows x64 ZIP](https://github.com/SSParzival/zapfast-whatsapp-windows-no-opengl21/releases/download/v0.19.0-dx12.3/ZapFast-0.19.0-dx12.3-windows-x64.zip)**
| [SHA256](https://github.com/SSParzival/zapfast-whatsapp-windows-no-opengl21/releases/download/v0.19.0-dx12.3/ZapFast-0.19.0-dx12.3-windows-x64.zip.sha256)
| [Release notes and screenshot](https://github.com/SSParzival/zapfast-whatsapp-windows-no-opengl21/releases/tag/v0.19.0-dx12.3).
This compatibility build is published as a prerelease. Extract the ZIP and
open `zapfast.exe`, or use `start-software-rendering.cmd` to force CPU WARP.

To build and validate on Windows, install Rust and the required C++ tools,
then run:

```powershell
.\scripts\build-windows-compat.ps1
```

Output: `dist/windows-compat`. Link your phone through **WhatsApp > Linked
devices**. Official updates are disabled in this variant to preserve its
Windows graphics adaptation; update this fork manually.

Details: [compatibility](docs/windows-compatibility.md),
[pairing recovery](docs/pairing-recovery.md), [performance](docs/performance-windows.md),
and [fork publication](FORK.md). The Windows suites passed 1,867 and 1,869
tests with zero failures, plus seven synthetic graphics checks. The reports
identify what was tested. Account authorization and real message delivery
still require verification by the account owner.

The [original README](README.upstream.md), [MIT license](LICENSE), and
[third-party notices](THIRD-PARTY-NOTICES.md) are preserved. This fork is
independent of the original project, WhatsApp, and Meta.
