# Kryoto Auto-fixer

A portable Windows app that repairs an installed Kryoto game: put it in the game folder, pick a
source, press the K// logo. Every repair can be undone.

**Kryoto** · Tauri 2 (Rust) + React · MIT · [Download](https://github.com/Kryoto-Dev/kryoto-auto-fixer/releases/latest) · [All repos](https://github.com/Kryoto-Dev)

Needs Windows 10/11 with the WebView2 Runtime; no installer or admin rights.
How to use it: [docs/usage.md](docs/usage.md).

## Quick start

Install Node 22+, pnpm 10 and Rust's MSVC toolchain. Clone
[`kryoto-desktop`](https://github.com/Kryoto-Dev/kryoto-desktop) next to this repo (it holds the
shared `crates/kryoto-repair` engine and `src/library/RepairPanel.tsx`), then:

```powershell
pnpm --dir ../kryoto-desktop install --frozen-lockfile
pnpm install --frozen-lockfile
pnpm app:dev       # run it
pnpm app:build     # target/release/auto-fixer-gbef.exe
```

## Layout

| Path | What's there |
|---|---|
| `src/` | the UI (reuses Desktop's repair panel and controls) |
| `src-tauri/` | the native app around the shared repair engine |
| `docs/usage.md` | how players use it |

## Checks

```powershell
pnpm build
pnpm test
```

## Release

Bump the version in `package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`, add a
`CHANGELOG.md` entry and push to `main`. CI publishes the executable with its SHA-256.

## More

[CONTRIBUTING.md](CONTRIBUTING.md) · [SECURITY.md](SECURITY.md) · [THIRD_PARTY.md](THIRD_PARTY.md) · [MIT license](LICENSE)
