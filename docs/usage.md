# Using the Auto-fixer

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
