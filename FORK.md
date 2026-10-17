# About this fork and its publication

**ZapFast: WhatsApp for Windows without OpenGL 2.1**

Repository: <https://github.com/SSParzival/zapfast-whatsapp-windows-no-opengl21>.

A personal Windows 11 compatibility fork for an Intel N95 PC using Microsoft
Basic Display Adapter. The main change is Direct3D 12/WARP rendering without
requiring an OpenGL 2.1 context.

AI assistance credit for the adaptation: **GPT-6.1 Sol (medium)**. This credit
covers the fork's modifications. The original client is by Carmine Paolino
and the ZapFast contributors.

## Provenance and scope

- Original project: <https://github.com/crmne/zapfast>.
- Source baseline: `021c568639159b8dca7a76eb4adc6f5d8cac5a11`, package 0.19.0
  with some then-unreleased upstream work.
- Validated variant: `0.19.0-dx12.3`, Windows x64. Graphics, pairing recovery,
  and performance changes are documented in `docs/`.
- The original protocol, features, license, and notices are retained.
- Pairing checks received and renewed QR codes using a temporary profile.
  Account authorization and real message delivery require the owner's phone.
  Performance numbers are synthetic, isolated-stage measurements from a
  debug test executable, not whole-application release timings.

## Repository and history

This is a GitHub fork of `crmne/zapfast` under `SSParzival`. Its `main` branch
preserves the upstream history and pins the source tree validated for this
PC. Upstream changes after the tested baseline remain in history but are not
incorporated into this compatibility snapshot. Publication adds a normal
commit to the fork's main branch without rewriting its remote history.

The local checkout uses `origin` for this fork and `upstream` for the original
project. To obtain another checkout:

```powershell
git clone https://github.com/SSParzival/zapfast-whatsapp-windows-no-opengl21.git
cd zapfast-whatsapp-windows-no-opengl21
git remote add upstream https://github.com/crmne/zapfast.git
git remote -v
```

Keep compatibility changes on `main` and preserve the existing history.
Retain `LICENSE`, `THIRD-PARTY-NOTICES.md`, and the notices for distributed assets.

## Simple manual publication

Inherited automation is preserved, inactive, in `.github/upstream-workflows/`.
Build and validate with `scripts/build-windows-compat.ps1`. This fork uses
manual releases, and GitHub Actions is disabled for this repository.

After pushing validated source to `main`, create a manual release identifying
the local version, tested hardware, and validation limits. Package the contents
of `dist/windows-compat` and attach the ZIP and its SHA256.

The validated executable has this SHA256, unchanged by presentation or
English documentation updates:

```text
1eddcb1af1e957d0766b38154ba1ea31a6fbad9a1ad064ee797dc5b075023235
```

Include only the executable, launcher, README, licenses, public reports, and
checksums. Keep caches, backups, account data, logs, and local captures out of
the source history. Review staged files even when exclusions are present in
`.gitignore`. Release screenshots must contain synthetic data only.

The inherited website and packaging recipes remain upstream references;
their official downloads belong to the original project. Use this fork's
README and Releases for the Windows compatibility adaptation.
