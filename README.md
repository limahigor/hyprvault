# HyprVault

HyprVault is a terminal user interface for the Hyprland/Omarchy ecosystem focused on viewing secrets, credentials, and metadata in a clean, fast, and native terminal experience.

## Status

This project is in its early stage.

The first goal is to deliver an MVP with:

- a main panel
- a mocked item list
- keyboard navigation
- a details panel
- a default theme with room for future Omarchy theme integration

## Initial Roadmap

1. Create the basic TUI application structure
2. Render a mocked list in the main panel
3. Add a details panel for the selected item
4. Organize state, domain, and theme into dedicated modules
5. Integrate real data sources after the visual MVP is stable

## Running

```bash
cargo check
cargo test
```
