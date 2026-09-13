# bookrack design system

This fork uses a restrained product register: paper-light and graphite-dark
surfaces, moss primary actions, native sans chrome, and literary serif titles.
The authoritative design lock, palette, spacing, accessibility principles and
surface inventory are in [docs/design.md](docs/design.md).

Tokens live in `apps/readest-app/src/styles/bookrack-tokens.css`. They alias
upstream semantic theme values so user-selected colors and e-ink remain valid.
The fork changes library grid, reader chrome and the TTS sheet only; reading
fonts, speech engines, keyboard behavior and safe-area logic remain upstream.
