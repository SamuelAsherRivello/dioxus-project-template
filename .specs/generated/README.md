# Dioxus Project Template

Dioxus 0.7 app generated from the reusable Dioxus Project Template. The starter UI uses Tailwind-first component styling with only minimal plain CSS for theme variables and document-level behavior.

Replace this README with project-specific goals, screenshots, deployment links, and setup notes as the app takes shape.

## Scripts

| Command | Required? | Description |
| ------- | --------- | ----------- |
| `.\Scripts\Common\InstallDependencies.ps1` | ✅ | Installs repository dependencies and builds Tailwind CSS output. |
| `.\Scripts\Common\RunWeb.ps1` | ✅ | Starts the web app on a concrete local IPv4 address. |
| `.\Scripts\Other\RunDesktop.ps1` | ❌ | Starts the desktop app with Dioxus desktop. |
| `.\Scripts\Other\RunTests.ps1` | ❌ | Runs UI crate tests. |

## Architecture

| Path | Description |
| ---- | ----------- |
| `packages/ui` | Shared Dioxus UI crate for routes, components, services, assets, and tests. |
| `packages/web` | Web entrypoint and web assets. |
| `packages/desktop` | Desktop entrypoint and desktop assets. |
| `specs` | Project specs. Start by replacing the generated purpose placeholder. |

## Dioxus Features

Keep this section updated as routes, cache behavior, platform support, Dioxus feature usage, or Dioxus Components usage changes.

| Area | Initial status | Notes |
| ---- | -------------- | ----- |
| Dioxus 0.7 components, RSX, signals, context, routing, assets, async resources, effects, error boundaries, web, and desktop | ✅ | Present in the generated starter app. |
| Browser localStorage snapshots and native SQLite cache | ✅ | Present unless the generated project intentionally changes cache architecture. |
| Dioxus Components gallery usage | ✅ | Alert Dialog and Aspect Ratio are present in the starter app. |
| Forms, server functions, and dynamic routes | ❌ | Add when the generated project needs those workflows. |

## Next Steps

| # | Task |
| - | ---- |
| 1 | Replace placeholder route copy and imagery with project-specific content. |
| 2 | Update `specs/001-project-purpose/spec.md` with the real product purpose. |
| 3 | Update the root `README.md` Dioxus Features section when routes, cache behavior, platform support, Dioxus feature usage, or Dioxus Components usage changes. |
