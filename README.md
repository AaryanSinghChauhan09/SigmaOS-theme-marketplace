# SigmaOS Theme Marketplace

> Forked from [AaryanSinghChauhan09/SigmaOS-theme-marketplace](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-marketplace) (MIT). See `SIGMAOS_FORK.md`.

The website for [themes.sigmaos.org](https://themes.sigmaos.org): browse, preview and install community themes for [SigmaOS](https://sigmaos.org).

Themes are registered, validated and published by [AaryanSinghChauhan09/SigmaOS-theme-registry](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-registry); this app fetches `catalog.json` from the CDN, caches it for five minutes, and keeps serving the last catalog it fetched if the CDN becomes unreachable. The committed snapshot in `data/catalog.json` is used only when the app has not reached the CDN since it started.

## Running locally

Requires Ruby 3.4 (see `.ruby-version`; `mise install` sets it up).

```sh
bundle install
bin/dev            # Rails on http://localhost:3000 + Tailwind watcher
bin/rails test
bundle exec rubocop
```

Environment:

| Variable       | Purpose                                                                                       | Default                          |
| -------------- | --------------------------------------------------------------------------------------------- | -------------------------------- |
| `CDN_BASE_URL` | Base URL of the registry CDN; `/v1/catalog.json` is appended                                  | `https://cdn.themes.sigmaos.org` |
| `CATALOG_URL`  | Full catalog URL, overrides `CDN_BASE_URL` (test env sets it to nil to use the snapshot only) | derived                          |

Copy `.env.example` to `.env` for local development.

To refresh the local catalog snapshot: `curl -s "$CDN_BASE_URL/v1/catalog.json" -o data/catalog.json`.
## Documentation

Detailed Wiki documentation is available in the standalone Wiki repository:

- [SigmaOS-theme-marketplace-Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-marketplace-Wiki) - Full Wiki documentation

For the complete SigmaOS Wiki catalog, see:
- [SigmaOS Wiki Staging](https://github.com/AaryanSinghChauhan09/sigmaos-wiki-staging)

## See Also

Related SigmaOS **Desktop** repositories:

- [SigmaOS-shell](../SigmaOS-shell/README.md) — SigmaOS desktop shell
- [SigmaOS-shell-config](../SigmaOS-shell-config/README.md) — SigmaOS shell configuration
- [SigmaOS-deskland](../SigmaOS-deskland/README.md) — SigmaOS Deskland desktop environment
- [SigmaOS-hyprland-titlebars](../SigmaOS-hyprland-titlebars/README.md) — SigmaOS Hyprland titlebars
- [SigmaOS-sddm](../SigmaOS-sddm/README.md) — SigmaOS display manager theme & config
- [SigmaOS-ui](../SigmaOS-ui/README.md) — SigmaOS UI toolkit
- [SigmaOS-theme-registry](../SigmaOS-theme-registry/README.md) — SigmaOS theme registry
- [SigmaOS-themes-loops](../SigmaOS-themes-loops/README.md) — SigmaOS animated theme loops
- [SigmaOS-themes-intros](../SigmaOS-themes-intros/README.md) — SigmaOS boot & login theme intros
