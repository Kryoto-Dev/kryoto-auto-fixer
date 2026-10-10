# Changelog

What changed in Kryoto Auto-fixer, newest first.

- **Before writing:** bump `version` in `package.json`, `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml` together; the release workflow publishes a version once its section is here.
- **Sections:** `### Added`, `### Changed`, `### Fixed`, `### Removed`. A bold lead and a sentence or two.

## [0.1.0] - 2026-10-10

### Added

- **Repair a Kryoto game without Kryoto Desktop.** Put `auto-fixer-gbef.exe` in the game's folder, close the game, pick a source and press the K// logo. Offline: GBE Fork, or RUNE's Regular, Steakclient or Steamclient profile. Online: Kryoto Online, with Steam running.
- **Undo.** Your files are backed up before each repair; Undo puts them back, also after a repair that was interrupted.
- **Working or Not working.** Tell the fixer how the game runs afterwards. Working: you can delete the fixer. Not working: it copies a report and opens a support ticket, so you only have to paste it.
- **Your Kryoto look.** It follows Kryoto Desktop's palette, corners and font when Desktop is installed.

Windows 10 or 11 with the Microsoft Edge WebView2 Runtime. No installer, no administrator account.
