# Security

Report vulnerabilities privately through [Kryoto support](https://kryo.to/support).
Include the affected version and steps to reproduce. Do not post account
credentials or private support logs in public issues.

The app repairs the folder containing its executable. The UI cannot choose
another folder or download URL. Downloads come from each source's GitHub
releases and use published checksums when available. These checks depend on
the upstream project's security.

Archive entries that escape the destination, symlinks, oversized archives and
changed cached tools are rejected. Undo checks paths and backup hashes before
restoring game files.

Clicking **Not working** or **Copy support report** copies the game, source,
release hash, operating system and recent repair logs. User directory names
are redacted. Logs stay on your device until you paste them into a ticket.
The support link contains the game slug and ticket subject.
