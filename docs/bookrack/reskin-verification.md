# Top-three surface verification

The library grid, reader toolbar/menus and Read Aloud sheet use the shared
Bookrack tokens. Settings and dialogs inherit palette/type only. EPUB document
styling, engine implementations, virtualization and safe-area calculations remain
upstream-owned. The signed-out Premium badge now follows the existing self-hosted
entitlement result; no subscription gate or TTS engine was changed.

## Rendered evidence

Captured from the final production web build on 2026-09-13. Desktop is
1440 × 1000; the mobile TTS and chapter screens are 390 × 844. A fresh browser
profile blocks service workers to prevent stale application assets. These are
real app captures, separate from the design concepts.

| Surface | Before | After |
| --- | --- | --- |
| Library | [Before](screenshots/before/library.png) | [After](screenshots/after/library.png) |
| Reader chrome | [Before](screenshots/before/reader.png) | [After](screenshots/after/reader.png) |
| Read Aloud | [Before](screenshots/before/tts.png) | [After](screenshots/after/tts.png) |

[Mobile TTS](screenshots/after/tts-mobile.png) retains every transport control and
the full-width Offline Audio action. With self-hosting enabled and no signed-in
user, it shows download status instead of Premium. Activating it opens the
[chapter download screen](screenshots/after/offline-chapters-mobile.png) with
Download All available and keeps the reader route. No live audio download or
cross-device sync is claimed.

[Settings](screenshots/after/settings.png) and [Dialog](screenshots/after/dialog.png)
are settled token-inheritance captures. Their original baseline screenshots were
captured during opening transitions, so they are inventory rather than reliable
visual comparisons.

[E-ink mobile TTS](screenshots/after/eink-tts-mobile.png) was checked using the
actual app setting. The Offline Audio row uses paper, ink and a crisp border;
its label and icons remain readable and it opens the chapter list. The final
e-ink correction was rebuilt and visually verified.

## Checks and limits

The final `NEXT_PUBLIC_SELF_HOSTED=true pnpm --filter @readest/readest-app
build-web` production build and `pnpm lint` (TypeScript/Biome) passed.
`pnpm --filter @readest/readest-app test:pr:web:unit` completed in 271.10s:
919 files passed, 4 skipped; 11,086 tests passed, 16 skipped.
The focused TTS sheet suite passed all 26 tests, including the signed-out
self-hosted affordance regression. A fresh independent reviewer found no
blocking defects in the source delta or desktop/mobile screenshots.

[Axe summary](reskin-a11y-summary.json) records observed violations and affected
node counts. Library contrast findings fell from 1 to 0 and TTS desktop contrast
findings from 17 to 13. Existing viewport, reader tree ARIA/TOC contrast, nested
library interaction, and settings-label findings remain. The TTS sheet also
retains the upstream focusability/low-opacity findings. This is a scoped visual
and regression check, not a clean whole-app accessibility certification. Dark
mode has token contrast analysis in the design record, but no fresh rendered
dark-mode audit is claimed here.
