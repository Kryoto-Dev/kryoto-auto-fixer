# Contributing

Keep the sibling `kryoto-desktop` checkout current. Change repair behavior and
sources in `crates/kryoto-repair`, which Forge, Desktop and this app share.
Change the shared UI in Desktop's `src/library/RepairPanel.tsx`.
Use Desktop's controls, logo and appearance tokens.

Run the shared engine tests, `pnpm build` and the Windows native build before
opening a pull request. Test repair and Undo with temporary files. Automated
tests must not change installed games. Let the player check whether the game
works after a repair.

Do not check in emulator DLLs, download archives, credentials, local logs or
game files. When preparing a release, keep the versions in `package.json`,
`src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json` the same.
