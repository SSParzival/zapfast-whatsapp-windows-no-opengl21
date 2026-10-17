# Windows compatibility source variant

The current validated fork is **0.19.0-dx12.3**; see its
[performance and validation report](performance-windows.md). It is a personal
compatibility adaptation, with assistance credited to GPT-6.1 Sol (medium).
The renderer and pairing history below describe the preceding variants.

Version **0.19.0-dx12.2** is a local compatibility variant against upstream commit
`021c568639159b8dca7a76eb4adc6f5d8cac5a11` (package version 0.19.0;
the commit also contains unreleased work). It is not an official release.
The preceding 0.19.0-dx12.1 executable was built and installed locally.
Its debug demo build passed 13 real-window rendering checks on this PC.
That installed release opened a visible, responsive window and passed a close
and reopen cycle in the same background process. It did not validate pairing.
The current source also fixes pairing recovery; see [pairing recovery](pairing-recovery.md).
A fresh synthetic profile passed a live QR receipt, expiration and retry cycle
in 163.47 seconds. Version 0.19.0-dx12.2 passed 1,865 default-feature tests and
1,867 all-feature tests, with zero failures. Each suite has 12 tests ignored
by default, including the separately executed live probe. Documentation with
warnings denied passed. Three updated synthetic login windows also passed
Direct3D 12 CPU rendering and visual review. The optimized Windows x64 release
completed in 26m 43s without compiler or linker warnings. It contains 61,644,288
bytes, reports `0.19.0-dx12.2`, and has SHA256
`d3ce4eb659b263aafba9a120edc660f646d4f8b2802fa363925a5bfbe4928e4c`.
The installed executable and the independent app copy were verified against
that checksum. The installed release opened a visible, responsive native
window on 2026-10-17; maximized client bounds were x=0, y=23, width=1280,
height=777. A fresh opaque profile backup and the preceding executable were
preserved before replacement. Actual account authorization from the owner's
phone and real message delivery remain outside this validation.

On Windows the patch selects WGPU explicitly and restricts its runtime
backends to Direct3D 12, overriding `WGPU_BACKEND`. It never retries through
OpenGL. The OpenGL frosted header is replaced by an opaque egui rectangle,
so the Messages theme cannot submit a glow callback to the WGPU painter.
Linux and macOS keep their original OpenGL renderer and blur.

`glow` remains compiled because fastframe 0.6.0 enables it and eframe's
`App::on_exit` signature depends on that feature. It is not the Windows
renderer. This patch removes the runtime requirement, not all GL code from
the dependency graph.
The executables still import Windows' system `opengl32.dll` through
those shared dependencies. That does not establish or request an OpenGL 2.1
context. The real-window tests confirmed actual Direct3D 12 rendering.

For CPU rendering, set `ZAPFAST_SOFTWARE_RENDERING=1` before starting the
compatibility executable. The selector requires a CPU adapter compatible
with the window's surface (normally Windows WARP). If none is exposed by
WGPU, startup fails with details; it does not silently use hardware or GL.
WARP rendering was confirmed on this PC. CPU rendering is slower than a
supported hardware driver; no frame-rate guarantee is made.

## Build and validate

Use Windows x64 with the Rust toolchain from `rust-toolchain.toml`, Visual
Studio Build Tools with C++ and a Windows SDK, CMake, and Perl on PATH.
Open a Developer PowerShell for Visual Studio, then run:

```powershell
.\scripts\build-windows-compat.ps1
```

The script checks formatting, Clippy with default and all features, tests
with default and all features, documentation, and a release build. It stops
on any failure. Missing tools cause an actionable error; nothing is installed.
The existing lockfile is retained. A `--locked` error must be investigated
and the lock regenerated with Cargo before distribution, never bypassed.

The output directory is `dist/windows-compat`; its software launcher scopes
the environment variable to its own process and starts the app in the launcher's
directory. No portable-update marker is
included. This variant disables official update checks, downloads and
installation on Windows, even when an existing settings file opts in:
upstream release packages have the original OpenGL requirement and must
not replace this custom build. Compatibility updates must be installed manually.

For repeatable graphics checks, build with `--features demo` and run
`target\debug\zapfast.exe --demo` with and without the software variable.
Local real-window checks passed for default startup, forced CPU rendering,
light/dark palettes, Messages light/dark, login, settings, video, expanded
video, narrow layout, a 1180x700 client, custom caption buttons and a stale
`WGPU_BACKEND=gl` environment. Every process exited successfully and produced
a PNG; diagnostic logs confirmed DX12, with CPU selection in software cases.
All 13 captures were visually reviewed. These used a debug demo build while
compilation continued, so timings are not release performance measurements.
Evidence is stored outside the source repository in the standalone `qa` folder.

Live account linking, sending/receiving, real attachments, notifications,
archive recovery, camera/microphone operation and multiple monitors have not
been validated by these offline graphics checks.

## Local PC diagnosis

Read-only WMI inspection found Windows 11 Pro x64 build 26200, an Intel N95
(four cores/four threads), about 15.8 GiB RAM and initially 132.9 GiB free on C:
before installing build tools and compiling.
The desktop API reports one 1280x800 display at 96 DPI; WMI reports the panel's
native dimensions as 800x1280. The active adapter is **Microsoft Basic Display Adapter**, driver
`10.0.26100.1`; it is not an Intel graphics driver.

Intel lists UHD Graphics, DirectX 12.1 and OpenGL 4.6 for the N95:
[Intel N95 specifications](https://www.intel.com/content/www/us/en/products/sku/231800/intel-processor-n95-6m-cache-up-to-3-40-ghz/specifications.html).
Hardware capability does not mean the currently installed driver exposes
those APIs. The missing vendor driver is a strong explanation for the
official build's startup problem, but no startup error was reproduced here.
Use the PC manufacturer's driver or
[Intel Driver & Support Assistant](https://www.intel.com/content/www/us/en/support/detect.html)
to identify the matching package. No driver was installed or changed.

## Review limits

Reviewed Cargo features, pinned eframe and fastframe implementations,
renderer selection, graphics error handling, the custom blur callback,
build workflows and update packaging. This is not an exhaustive audit of
the messaging protocol, encryption, account isolation, media or all bugs.
The initial environment lacked build tools. Rust 1.98.0, Visual Studio 2022
Build Tools with the Windows SDK, its bundled CMake, and portable Strawberry
Perl have now been installed. For 0.19.0-dx12.1, formatting and Clippy with warnings denied passed
for default and all features. The default-feature test run passed 1,857 tests
with zero failures and 11 explicitly ignored tests. The all-feature run passed
1,859 tests with zero failures and the same 11 explicitly ignored tests.
Linux and macOS dependency
feature checks retain Glow without enabling WGPU; those platforms were not
cross-compiled. Thirteen real-window graphics checks also passed.
Documentation with warnings denied also passed. The optimized Windows x64
release build completed successfully in 52m 17s. Its executable is 61,643,776
bytes and reports `0.19.0-dx12.1` in both CLI and Windows version resources.
MSVC emitted LNK4099 because optional OpenSSL debug symbols were not beside
the release linker inputs; it linked those native objects without symbols.
The matching PDB has been preserved in that build cache for future links.
This warning does not change the executable's runtime requirements.

The installed release was verified by SHA256 against the compiled binary:
`173ed867aaecf408f12e27f93bfb2404228393291e00afdd7dd93d75c0f1c338`.
Its software launcher opened a responsive native window. Closing and reopening
retained the same process and restored a responsive window. Maximized client
bounds were x=0, y=23, width=1280, height=777, fitting the current desktop.
The measured working set after reopening was about 222 MiB; this is a snapshot,
not a general memory or performance bound. Existing configuration and state
were backed up as opaque files before installation. No real conversations or
QR payloads were inspected, and no messages were sent as part of validation.

The first DX12 compilation exposed an upstream dependency-resolution
conflict: `gpu-allocator` 0.28.0 selected Windows bindings 0.61.3, while
`wgpu-hal` 30.0.1 uses 0.62.2. The manifest pins the application's Windows
bindings to 0.62.2 and the Cargo-generated lock aligns the allocator with
WGPU. The notification library keeps its separate, compatible 0.61 bindings.
No dependency source was patched or copied. The same class of conflict is
documented in [egui issue 8069](https://github.com/emilk/egui/issues/8069).

`cargo-audit` 0.22.2 reported zero known vulnerabilities against RustSec
database commit `f28da8c5d386e507fac79153655184056a20c054` (updated
2026-10-16). It reported maintenance warnings for `paste` 1.0.15
([RUSTSEC-2024-0436](https://rustsec.org/advisories/RUSTSEC-2024-0436.html))
and `ttf-parser` 0.25.1
([RUSTSEC-2026-0192](https://rustsec.org/advisories/RUSTSEC-2026-0192.html)).
These are transitive dependencies; no source override was introduced to
replace them. The audit result is not a proof of absence of vulnerabilities.

The upstream native provider is OpenSSL 3.6.3. OpenSSL's official
[3.6 release notes](https://openssl-library.org/news/openssl-3.6-notes/)
list further security fixes in 3.6.4 and 3.6.5 (the latter dated 2026-09-29).
The RustSec result must not be presented as covering every native-library CVE.
The Windows reverse dependency graph reaches OpenSSL only through
`libsqlite3-sys`; SQLCipher's provider uses AES-256-CBC, HMAC, PBKDF2 and random
bytes. The reviewed later fixes concern other protocols or primitives.
No matching exposed path was identified in that provider review, which is
an inference rather than a comprehensive reachability audit. This graphics
variant retains the upstream native version; it is not a security certification.

A native DXGI/D3D12 probe successfully created a WARP device at feature
level 11_0 on this PC (`HRESULT 0x00000000`). This checks Windows' software
device support. The app's surface and rendering checks subsequently confirmed
DX12 on **Microsoft Basic Render Driver**, software driver `10.0.26100.7309`.
A perfect or bug-free version cannot be certified from source inspection.
