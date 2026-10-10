# Kryoto Auto-fixer

A Windows repair app for installed Kryoto games, using Forge's repair engine
and Desktop's controls.

Put **auto-fixer-gbef.exe inside the game folder**, close the game, choose a
source and press the K// logo. Offline offers GBE Fork and RUNE's Regular,
Steakclient and Steamclient profiles. Online offers Kryoto Online and requires
Steam running.

After repair, start the game. Choose **Working** if it works; you can then
delete the executable. Choose **Not working** to copy a report, then
**Contact support** to open a ticket. Paste the report with Ctrl+V and describe
the problem.
Deleting the executable leaves the game configuration and Undo backup in place.

Changing a working game's source can break it. **Undo repair** restores the
files from before that repair, including existing emulator configuration. Undo
before trying another source. An interrupted repair also offers Undo when the
fixer is reopened. Keep `.kryoto-repair` while you need its backup.

The app checks release metadata, Forge DLL backups, emulator settings, folder
names and Kryoto shortcuts against the Kryoto catalog. It needs more than a
Steam ID to identify a Kryoto game. Those local files can be deleted or copied.
Conflicting game IDs, unsupported DLLs or missing required originals stop the
repair before it changes game files.

Windows 10/11 with Microsoft Edge WebView2 Runtime is required. No installer or
administrator account is needed. The app downloads emulator releases from
their GitHub projects and caches them locally. See [SECURITY.md](SECURITY.md)
for download checks and support report contents.

## Build

Install Node 22+, pnpm 10 and Rust's Windows MSVC toolchain with the Visual
Studio C++ build tools. Clone this project and
[`official-kryo-to/kryoto-desktop`](https://github.com/official-kryo-to/kryoto-desktop)
as sibling directories. Desktop must include `crates/kryoto-repair` and
`src/library/RepairPanel.tsx`. CI fetches Desktop from GitHub, so those files
must be available there before CI can build this app.

```powershell
pnpm --dir ../kryoto-desktop install --frozen-lockfile
pnpm install --frozen-lockfile
pnpm build
pnpm app:build
pnpm test
```

The executable is `target/release/auto-fixer-gbef.exe`. Development uses
`pnpm app:dev`. Appearance follows Desktop's saved palette, corners and font
when available. It reads those appearance settings only.

## Source and releases

MIT licensed. CI tests the shared engine and uploads the Windows executable
as a build artifact. A version bump with its CHANGELOG entry publishes a
release with the executable and its SHA-256
([releases](https://github.com/official-kryo-to/kryoto-auto-fixer/releases/latest)).
See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md) and
[THIRD_PARTY.md](THIRD_PARTY.md).
