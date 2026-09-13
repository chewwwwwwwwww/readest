# bookrack design direction

Brandon reads on the Mac in daylight and listens on his iPhone while travelling;
a paper-light default and graphite dark mode serve those two contexts. This is
a restrained product interface: tinted neutrals with moss used for selected
state and primary listening/download actions.

## Palette and tokens

The CSS custom property file is `src/styles/bookrack-tokens.css` in readest-app.
It aliases the active upstream theme, preserving custom themes and e-ink mode.
The default theme gets paper/moss values; other user-chosen themes remain valid.

| Role | Light | Dark |
| --- | --- | --- |
| Paper | #f6f5f0 | #202521 |
| Ink | #252c27 | #e8eee9 |
| Moss accent | #456454 | #a3c4b0 |
| Secondary surface | derived from active theme | derived from active theme |
| Muted ink | derived from active theme | derived from active theme |

The implementation stores palette values in OKLCH. Test text against its actual
surface, including selected/disabled states. No pure black/white decorations.

## Type

UI stack: `ui-sans-serif, system-ui, sans-serif`; native platform text stays
crisp without fetching a font. Literary headings: `Literata, Charter, Georgia,
serif`, with existing reading font preferences preserved. Actual book typography
remains user-controlled; no forced CSS enters the reader document.

Fixed UI scale: 12, 14, 16, 20, 24px. Labels use sans and medium weight; book
titles use a quiet serif hierarchy where sufficient room exists. Reading text
should remain around 65–75ch where the user's settings permit it.

## Spacing and shape

Spacing tokens: 4, 8, 12, 16, 24, 32px. Library covers receive a predictable grid
with a wider horizontal rhythm and comfortable metadata. Keep responsive column
choices and user-selected fixed columns intact. Radius: controls 8px, panels
16px, covers 6px. No nested ornamental cards.

## Three surfaces

1. Library: cover imagery remains the visual anchor. Simplify metadata hierarchy
   and give titles more room. Keep virtualization, grouping and all import flows.
2. Reader chrome: quiet frame, consistent spacing and visible keyboard focus.
   Keep platform title-bar and safe-area handling intact.
3. TTS sheet: clear title/chapter hierarchy, generous transport controls, and a
   prominent full-width Offline Audio affordance. Keep lyrics, voice/rate/sleep
   and download states and callbacks unchanged.

Settings and dialogs inherit shared typography/palette only. No bespoke rebuild.

## Dated patterns to remove within scope

Crowded cover grids with almost no desktop gutter; mixed icon hit areas;
heavy cover shadows; small low-opacity metadata; blue competing with book art;
raised or over-rounded utility controls; download actions that look incidental.

## Motion and accessibility

Use 150–200ms state feedback with an ease-out curve. No entrance choreography,
hover scaling, parallax, layout animation or blur. E-ink and reduced-motion modes
remove optional motion. Every added custom control keeps a visible focus ring.
RTL uses logical properties. Preserve title-bar and bottom safe-area padding.

## Evidence

`bookrack/references/REFERENCES.md` annotates five official Readwise images and exact
provenance. These are publisher references, not fresh logged-in captures.
`bookrack/concepts/` contains three generated design concepts, one per surface, from the
built-in image generation tool. They guide hierarchy; they are not app screenshots
or exact feature specifications. Generated decorative copy and fake library
entries are not implementation requirements.

The compact vector identity is an open book in moss on paper. Its geometry is
shared by native icon and splash artwork; native generation/signing is separate.

## Token review

An independent design-lock review checked the actual OKLCH palette against
WCAG relative luminance. Ink/paper: 13.16:1 light and 13.18:1 dark; muted ink
across all three surfaces: 4.86:1 minimum; accent/paper: 5.98:1 light and 8.40:1
dark. These are token-pair checks, not a claim that every upstream component or
reduced-opacity text treatment has passed a rendered accessibility audit.

## Screen inventory

Fresh local web captures at the source baseline, 1440 × 1000 desktop viewport:

| Surface | Capture | Scope |
| --- | --- | --- |
| Library | [Library](bookrack/screenshots/before/library.png) | Cover grid and title hierarchy |
| Reader chrome | [Reader](bookrack/screenshots/before/reader.png) | Toolbar and menus around user-controlled text |
| Settings | [Settings](bookrack/screenshots/before/settings.png) | Inherit tokens, preserve settings primitives |
| Dialogs | [Dialog](bookrack/screenshots/before/dialog.png) | Inherit tokens, preserve focus and interaction |
| TTS player sheet | [TTS](bookrack/screenshots/before/tts.png) | Playback and prominent offline audio action |

These are actual application captures, separate from the generated concepts.
