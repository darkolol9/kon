---

name: ui_and_tui
description: Design and implement Ratatui terminal user interfaces, including window layouts, panels, navigation flows, keyboard interactions, responsive layouts, focus management, and UI/UX decisions for Rust TUI applications.
license: MIT
compatibility: opencode
metadata:
framework: ratatui
language: rust
category: ui
------------

# What I do

* Design Ratatui application layouts before implementation.
* Create window, panel, sidebar, modal, tab, and dashboard architectures.
* Define keyboard-first navigation patterns.
* Design focus management and selection behavior.
* Create responsive layouts that adapt to terminal size changes.
* Establish information hierarchy and screen organization.
* Recommend status bars, command palettes, dialogs, and feedback patterns.
* Review and improve TUI usability and consistency.

# Design Principles

## Layout First

Always design the screen structure before choosing widgets.

Prefer nested Ratatui layouts over hardcoded coordinates.

Organize screens into:

* Header
* Main content area
* Optional sidebars
* Footer or status bar

## Single Responsibility Panels

Each panel should serve one purpose.

Examples:

* File explorer
* Log viewer
* Process list
* Chat window
* Metrics dashboard

Avoid panels that mix unrelated functionality.

## Information Hierarchy

Allocate space according to importance.

The primary workflow should receive the majority of the screen area.

Secondary controls should consume minimal space.

## Keyboard First

Prefer keyboard-driven interaction.

Recommended defaults:

* ↑ ↓ for navigation
* ← → for panel switching
* Tab for focus changes
* Enter for confirmation
* Esc for cancellation
* / for search
* ? for help
* q for quit

Keep shortcuts consistent throughout the application.

## Focus Management

Interactive elements must have a visible focus state.

Users should always know:

* Which panel is active
* Which item is selected
* Which action is available

## Responsive Design

Design for:

* Small terminals
* Split tmux panes
* Fullscreen terminals

When space is limited:

* Preserve primary workflows
* Collapse secondary panels
* Reduce visual detail
* Avoid horizontal scrolling when possible

# Architecture Guidelines

Prefer separating:

* Application state
* Event handling
* Layout generation
* Widget rendering

Treat each major screen as an independent module responsible for its own rendering and interactions.

# When To Use Me

Use this skill whenever building or designing:

* Ratatui applications
* Rust terminal dashboards
* Terminal file managers
* Monitoring tools
* Log viewers
* Process managers
* Terminal chat applications
* Multi-panel TUIs
* Keyboard-driven interfaces

Use this skill when UI structure, usability, navigation, or layout decisions are more important than business logic.

