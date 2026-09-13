# bookrack source baseline

Fork: https://github.com/chewwwwwwwwww/readest

Baseline upstream revision: `c3a95ba7b35c013baa644358669d28bd55efaba8`.

## Build the web client

Use Node 24 and the repository-pinned pnpm 11.1.1. From this repository root:

```sh
git submodule update --init --recursive
pnpm install --frozen-lockfile
pnpm --filter @readest/readest-app setup-vendors
NEXT_PUBLIC_SELF_HOSTED=true pnpm --filter @readest/readest-app build-web
```

The checked-in `.env.web` selects the web platform. No live credentials are
needed for the local library and reader. Do not add operator secrets to tracked
files. Sync only works after the separate bookrack bring-up procedure is wired.

## Build a local Docker image

Initialize submodules and run from this fork’s root:

```sh
docker build --target production-stage -t bookrack-readest:local .
```

This uses upstream’s standalone runtime image. Its `SELF_HOSTED=true` default
injects runtime configuration for browser clients. Use a distinct tag per
validated commit for repeatable deployment, for example
`bookrack-readest:<validated-fork-commit>` rather than overwriting a deployed tag.

On the machine running Docker, use the existing bookrack operations checkout.
Set `READEST_IMAGE=bookrack-readest:local` in its gitignored `.env`, keeping every
other generated secret and environment value intact. Then, after the stack has
already been brought up using `docs/BRINGUP.md`, replace only the client:

```sh
./scripts/compose.sh up -d --no-deps --pull never client
./scripts/compose.sh logs --tail=100 client
```

The compose wrapper preserves the project name, environment file and both
upstream/bookrack overrides. `--pull never` keeps the local image local. The
image must exist on the Docker host; transfer it using `docker save` / `docker
load` or build directly on the Mac mini. Do not run the operations wrapper's
blanket `pull` against a local-only image tag. Registry publishing is a separate
operator step after registry credentials are wired.

The self-host unlock reaches our served browser client, not the official
App Store app. Native clients require our own build-time flag and iOS signing.

## Verification record

Initial host: Apple silicon macOS, Xcode 26.6, pnpm 11.1.1, Rust 1.95.0.
On 2026-09-13 the commands above passed on this machine: frozen installation
(1,653 packages), vendor preparation, and `NEXT_PUBLIC_SELF_HOSTED=true pnpm
--filter @readest/readest-app build-web` exited 0. Next.js 16.3.3 compiled in
3.2 minutes, completed TypeScript in 43 seconds and generated all 44 static pages.
This validates the unmodified upstream web source at the pinned revision plus
this fork’s documentation delta. It does not claim an iOS or Docker image build.

The registry needed a second install attempt due to slow downloads. The successful
retry used `pnpm install --frozen-lockfile --network-concurrency=8
--fetch-timeout=180000`. No dependency versions or lockfile entries changed.
Next reported existing Serwist/Turbopack and deprecated middleware warnings;
the build still exited successfully.
