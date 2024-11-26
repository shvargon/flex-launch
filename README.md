# flexlaunch

A description of this project.

```bash
flatpak install org.freedesktop.Sdk.Extension.rust-stable//24.08

flatpak-builder --force-clean build-dir io.github.shvargon.flexlaunch.json
flatpak-builder --run build-dir io.github.shvargon.flexlaunch.json flexlaunch
```