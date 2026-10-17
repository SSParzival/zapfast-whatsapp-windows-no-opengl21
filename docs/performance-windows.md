---
title: Windows performance checks
---

# Local Windows performance changes

The local `0.19.0-dx12.3` variant keeps the DX12/WARP rendering path and the
pairing recovery from `0.19.0-dx12.2`. It does not require an OpenGL 2.1
context. Dependency versions, release compiler settings, image quality and
network deadlines are unchanged by this performance work.

## Changes

- The normal chat list, authenticated locked folder and collapsed avatar
  list select and sort indices into the active account's chats. They copy
  chat data only for the rows requested by egui's existing virtual viewport.
  Previously each frame copied every selected chat, including full preview
  text, group descriptions and participant lists, before drawing visible rows.
- Contact search builds a temporary set of existing chat identifiers once
  per search. Previously every contact scanned the entire chat vector to
  exclude existing chats. The membership stage changes from quadratic to
  expected linear work in the combined contact/chat counts.

Indices and the identifier set are rebuilt for each invocation and never
persist across frames or accounts. Chat updates and account changes cannot
leave a cached index stale. Existing filtering and sorting still govern
pins, favorites, archives, labels, unread preservation and locked chats.
Rendering still queues actions for application after the frame. No database
schema, stored message, credential or protocol behavior changes here.

## Reproducible measurement

`app::tests::synthetic_chat_list_performance` is an explicitly ignored,
offline microbenchmark. Run it with `--ignored --nocapture --test-threads=1`.
It creates 100, 1,000 and 10,000 fictitious chats, each with 24 member
identifiers and a 1,040-byte group description, and compares:

- The old strategy of copying every selected chat against selecting indices
  and copying 12 visible rows. Both use the same current selection routine;
  this isolates the strategy, rather than comparing entire historical builds.
- The old repeated linear membership scan against building a set and checking
  the same synthetic identifiers, including the set construction cost.

Each strategy is warmed up and measured 17 times; the median is reported.
Timings are informational, never test assertions. These measurements cover
the selection/copy and membership stages, not complete frame rendering,
startup time, network performance, or total application memory. Actual
benefits depend on chat count, group metadata and viewport size.

New regression tests check that reordering, updated chat metadata, removal
and locking are immediately reflected, and that contact search excludes
hidden existing chats and reflects deletion immediately. The existing suite
also exercises list scrolling, keyboard navigation, favorites, locked-folder
privacy and multiple accounts. Validation results and synthetic-only graphics
captures are kept outside the source tree in the adjacent `qa` directory.

## Results on this PC, 2026-10-17

Windows 11 Pro x64 build 26200, Intel N95 (4 cores), 15.8 GiB RAM,
Microsoft Basic Display Adapter 10.0.26100.1, desktop 1280x800 at 96 DPI.
The measurement ran after compilation finished. Application code in the
test executable is **debug**, with optimized dependencies; the installed
application is a separately built optimized release. Ratios below describe
these isolated strategies in the test executable, not whole-application speed.

Selection and chat copies, with 12 viewport rows (median of 17, milliseconds):

| Chats | Copy all rows | Copy viewport rows | Speedup |
| ---: | ---: | ---: | ---: |
| 100 | 0.762 | 0.155 | 4.93x |
| 1,000 | 7.387 | 1.270 | 5.82x |
| 10,000 | 88.893 | 18.532 | 4.80x |

Identifier membership only, equal chat/candidate counts, including building
the temporary set (median of 17, milliseconds):

| Chats / candidates | Repeated scan | Set lookup | Speedup |
| ---: | ---: | ---: | ---: |
| 100 | 0.094 | 0.091 | 1.03x |
| 1,000 | 7.328 | 0.763 | 9.60x |
| 10,000 | 773.802 | 7.881 | 98.19x |

The latter measures identifier exclusion using synthetic strings, separately
from contact eligibility, name matching, sorting and painting. It is not an
end-to-end contact search measurement. The temporary set needs O(chat count)
borrowed identifier entries; total application memory was not benchmarked.

Validation passed: formatting, Clippy with warnings denied for both feature
configurations, default tests (1,867 passed), all-feature tests (1,869 passed),
and Rust documentation with warnings denied. Both suites had zero failures
and 13 explicitly ignored tests; the synthetic measurement was then run
explicitly and passed in 16.79 seconds. The optimized Windows release built
in 19m 16s without compiler or linker warnings.

Seven real-window synthetic graphics checks passed and their screenshots
were visually reviewed: normal startup, forced WARP, compact list, authenticated
locked folder, narrow window, login and an environment requesting GL. All
confirmed DX12; forced-software cases confirmed a CPU adapter. These checks
retain the existing renderer and do not establish phone authorization.

The optimized release was installed and opened successfully. Its visible,
responsive maximized window fits the desktop (client 1280x777 below the native
title bar). The startup working-set snapshot was 108.1 MiB, not a memory bound
or a before/after comparison. Installed and portable executable SHA256:
`1eddcb1af1e957d0766b38154ba1ea31a6fbad9a1ad064ee797dc5b075023235`.
The previous `.2` executable and opaque profile files were backed up after a
normal shutdown. Account contents and real QR payloads were not inspected.

Windows is the execution target for this work. Linux and macOS have not
been built or run here. Real account authorization and message delivery
still require the account owner's phone and are not established by these
synthetic performance checks. See [pairing recovery](pairing-recovery.md)
and [Windows compatibility](windows-compatibility.md).
