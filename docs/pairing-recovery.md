# Pairing recovery in the Windows compatibility variant

The reported screen was **Waiting for a code from WhatsApp...** with no QR.
This is the empty `Unlinked` state in `src/ui/login.rs`.

The pinned protocol library rotates QR references and eventually dispatches
`PairingQrCodesExhausted`. If no phone-code flow is outstanding, it calls
`Client::shutdown()`. That shutdown is permanent. Previously the application
cleared the QR and called `reconnect_immediately()` on that same client,
leaving the empty waiting screen with no client capable of producing a code.
The manual reconnect command also reused that client.

Sources reviewed:

- [Pinned QR expiration implementation](https://github.com/crmne/whatsapp-rust/blob/70ddc5f1b32297da52233d0a69c11943555ba0d2/src/pair.rs)
- [Pinned client lifecycle implementation](https://github.com/crmne/whatsapp-rust/blob/70ddc5f1b32297da52233d0a69c11943555ba0d2/src/client/lifecycle.rs)
- [Pinned phone-code implementation](https://github.com/crmne/whatsapp-rust/blob/70ddc5f1b32297da52233d0a69c11943555ba0d2/src/pair_code.rs)

Version 0.19.0-dx12.2 changes the following behavior:

- Expired QR references show an actionable failure. Try again creates a new
  Bot over the same device store. It does not unlink the account or erase its
  archive. An outstanding phone-code flow survives QR expiration when the
  library explicitly leaves that connection open.
- Pairing and connection callbacks carry a client generation. An event from
  an old client cannot overwrite a newer attempt's link state. Existing
  privacy-generation handling for message deletion remains separate.
- Initial connection, reconnection and empty-code waits have a 60-second
  deadline. Phone-code requests have a 30-second timeout and request generation.
  Errors without a remaining QR show a retryable failure.
- Displayed phone codes use the validity duration supplied by the library.
  Expiration and `PairingCodeRefresh` retire the old code and offer a fresh
  attempt; a late result cannot restore a retired code.
- The waiting screen offers Request a new code. A disconnected login screen
  no longer claims that the device has already linked.
- Device-version parsing strips prerelease and build metadata, so the custom
  version still sends the three numeric device-version components.
- Direct network dialing stops polling an expired scheduling timer once
  every address has started. A silent last socket then waits for its answer
  or cancellation without continuously waking the runtime.

Offline regression tests cover expired QR recovery state, preservation of
phone codes, late phone-code results, late QR callbacks, connection deadlines,
prerelease versions, and silent network dialing.

An additional ignored test, `live_pairing_expiration_and_retry`, creates a
fresh synthetic profile and checks QR receipt, a full expiration cycle, and
QR receipt after retry. It never prints a QR or phone code, links an account,
or sends messages. Run it explicitly with:

```text
cargo test --locked --lib live_pairing_expiration_and_retry -- --ignored
```

Evidence and execution results belong outside this
source tree, in the adjacent `qa` directory. Receiving a QR establishes linking
readiness; successful scanning, account authorization, and message delivery
still require the account owner and are distinct from this probe.

On 2026-10-16 this live probe passed on the local Windows 11 PC: one test
passed, zero failed, in 163.47 seconds. It received the initial QR, observed
expiration, and received a fresh QR after the application rebuilt the client
through its normal Reconnect command. The output contained only test status
and the fact that codes were received. A separate WebSocket handshake to
`web.whatsapp.com` also succeeded in 644 ms. No real account was linked.
