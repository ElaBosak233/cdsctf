---
name: frontend-design
description: Design or modify React/Tailwind interfaces in this CdsCTF repository while preserving its warm-neutral, compact, icon-led visual system and reusable component conventions. Use for new pages, feature UI, responsive layouts, and visual refinements; do not use it for backend-only changes.
---

# CdsCTF Frontend Design

Use this skill for any user-facing work under `web/`. The product is a focused CTF platform: interfaces should feel calm, technical, dense, and dependable rather than promotional or decorative. Preserve the existing visual language before introducing new patterns.

## Source Of Truth

Read the nearby implementation before changing it. Reuse the existing primitives and conventions from:

- `web/src/styles/main.css` for tokens, fonts, light/dark values, radii, and global transitions.
- `web/src/components/ui/` for buttons, fields, cards, dialogs, badges, navigation, tables, charts, overlays, and typography.
- `web/src/components/widgets/` for CTF-specific challenge cards and challenge dialogs.
- `web/src/pages/_blocks/navbar/` and `web/src/pages/_blocks/background/` for the application shell.

Prefer `cn(...)`, Tailwind classes, CSS variables, and composition over one-off CSS. Do not replace an existing primitive with a new local implementation unless the primitive cannot express the required behavior.

## Visual Direction

- Keep the tone operational and approachable: warm off-white light mode, near-black dark mode, strong readable foreground, restrained borders, and small elevation changes.
- Use Ubuntu Sans Variable for UI text and Ubuntu Sans Mono Variable for IDs, flags, code, timestamps, points, and other machine-like values. These fonts are already imported in `main.css`.
- Treat the palette as semantic. Use `primary` for neutral emphasis, `info` for blue informational states, `success` for green positive/active states, `warning` for amber caution/frozen states, and `error` for red destructive/failure states. Prefer low-opacity tonal fills such as `bg-info/10` with `border-info/20` and `text-info`.
- Use the theme tokens (`background`, `foreground`, `card`, `popover`, `muted`, `muted-foreground`, `input`, `border`, `ring`) instead of hard-coded colors. Use arbitrary color values only when a domain object already supplies a meaningful color, such as a challenge category.
- Keep letter spacing at the browser default. Favor `text-sm` for dense controls, `text-xs` for metadata, `text-base` for compact headings, and `text-xl` or larger only for page-level emphasis.

## Geometry And Spacing

- The base radius is `0.5rem`; use `rounded-md` for controls and `rounded-lg` for standard cards. Reserve `rounded-elevated` (`1rem`) for large dialogs or feature surfaces and `rounded-badge` (`0.75rem`) for icon containers. Pills use `rounded-full` only for badges, status chips, avatars, or intentionally compact contributor tags.
- Standard controls are 40px high (`sm`), 48px high (`md`), or 44px high for large buttons. Keep icons at 16px in controls and about 20px in headings or icon containers.
- Use compact, regular spacing: `gap-2`/`gap-3` for inline controls, `gap-4`/`gap-5` for sections, and `gap-8` for separated form groups. Common page padding is `p-4`, `sm:p-6`, and `lg:p-8` or `lg:p-10`.
- Keep content in flexible columns with `min-w-0`, `min-h-0`, and scroll containers where needed. The global shell uses a 64px navbar and `--app-content-height`; do not create a second competing viewport model.
- Do not stack cards inside cards without a clear framing need. Use separators, whitespace, or a tonal surface to divide content inside an already-framed card.

## Application Shell

- The top navbar is sticky, 64px tall, border-bottomed, and translucent: `bg-card/80` with a light backdrop blur. It contains the brand, route tabs, appearance control, admin affordance, and auth/avatar actions.
- Use Lucide icons consistently. Icon-only controls must be square, have an `aria-label`, and expose a tooltip when the meaning is not obvious. Text buttons may include a leading icon through the shared `Button` `icon` prop.
- Active navigation is quiet but clear: a primary-tinted background (`bg-primary/10` or `bg-primary/7.5`) and `text-primary`. Avoid loud fills for ordinary navigation.
- Desktop sidebars are translucent (`bg-card/30`), lightly blurred, border-separated, and either about 16rem wide or collapsed to a 4rem icon rail. Use the existing sidebar primitives so mobile can switch to the dialog-based drawer automatically.
- The ambient background is intentionally subtle: fixed, behind content, and made from very low-opacity circles with slow 4-6 second float motion. It must never compete with text, tables, or forms.

## Component Rules

### Buttons

Use the shared `Button` and its variants:

- `solid` for the primary action in a region.
- `outline` for a secondary action that still needs a boundary.
- `tonal` for low-emphasis actions and selected pagination/navigation states.
- `ghost` for toolbar, icon, and tertiary actions.
- `link` for inline navigation.

Set `level` to `primary`, `secondary`, `info`, `success`, `warning`, or `error` rather than inventing button colors. Preserve disabled opacity and `loading` spinner behavior. Do not put long explanations inside buttons.

### Fields And Forms

Compose fields from `Field`, `FieldIcon`, `FieldButton`, and `TextField` where possible. The icon segment is a compact `bg-primary/20` block; the input uses `bg-input`, a subtle border, and a 2px ring with offset on focus. Use the `sm` field for dense admin/filter UI and `md` for ordinary forms.

Keep labels above controls, use `FormMessage` for validation, and show loading, disabled, success, and error states without shifting the layout. Use familiar Lucide icons for field intent (type, mail, lock, calendar, key, upload). Never rely on placeholder text as the only label.

### Cards, Badges, And Status

- A normal `Card` is a `rounded-lg border bg-card shadow-xs` surface. Add `shadow-sm` or `shadow-md` only for interaction or elevation feedback.
- `Badge` is pill-shaped and compact. Use `solid` for strong categorical emphasis, `tonal` for low-emphasis labels, and `outline` for status combinations.
- Status chips should pair semantic text with an icon when useful. For example, disabled uses muted styling, upcoming/visibility uses info, active/ongoing uses success, and paused/frozen uses warning.
- Challenge cards use generous internal padding (`p-5`), a category badge, a restrained separator, a small stats row, and a barely visible category icon in the background. Solved and cheated markers sit in the upper-right and must remain discoverable through a tooltip.
- Avatar and image components should provide a deterministic fallback and preserve a stable aspect ratio. Use the shared `Image`/`Avatar` components instead of raw image loading logic.

### Dialogs And Overlays

Use the shared `Dialog`. The overlay is opaque enough to isolate the task (`bg-black/80`); the content animates with a short fade/zoom and uses `w-[calc(100%-2rem)]` with an appropriate max width (`max-w-xl`, `max-w-2xl`, or `max-w-5xl`).

Large task dialogs should be a single elevated card with a clear header, separator-delimited sections, a bounded scrollable description, and fixed action sections. Challenge dialogs follow the order: category/title header, markdown description, optional attachments, optional instance controls, then submission controls.

### Tables And Admin Surfaces

Admin UI is information-dense and scan-friendly. Use the shared table primitives, small text, muted metadata, and semantic badges. For wide tables, keep a stable `min-width` inside a `ScrollArea`; use a sticky header with `bg-muted/95` and `backdrop-blur-sm`, and keep action columns sticky on the right when horizontal scrolling is possible. Rows use subtle `hover:bg-muted/50`; do not use dramatic row transforms or alternating decorative colors.

Pagination, filters, and page-size controls belong in a compact footer with result counts in muted text. Empty states should occupy real table height and explain the state without a wall of copy.

### Markdown, Code, And Charts

- Render long-form challenge text through `Typography` and `MarkdownRender`. Preserve the existing prose treatment, inline code tint, line-numbered code blocks, and readable link underlines.
- Keep code, IDs, timestamps, invite tokens, and point values in the mono font with tabular numerals where alignment matters.
- Charts use the shared `ChartContainer` and theme-aware variables. Keep grids muted, labels small, tooltips compact, and colors distinguishable in both themes. Prefer responsive dimensions and do not force a fixed canvas wider than its container.

## Page Patterns

- **Public/auth pages:** center a bounded surface with generous breathing room. Login and registration use a split card on wide screens: form and validation on the left, brand/logo context on the right, with a vertical divider. Collapse cleanly to one column on small screens.
- **Landing/about pages:** keep the brand mark and product title prominent but restrained. A small staggered fade/float is acceptable; the page must remain useful and legible with reduced motion.
- **Game entrance:** use a responsive two-column composition with media/poster context on the left and markdown/attention content on the right. Keep the primary participation action visible and full-width within its column.
- **Game workspaces:** use the game navbar plus sidebar or tab navigation. Challenges are grid-based and scannable; team pages use a clear section heading, separator, field groups, and compact member cards; scoreboard pages favor readable tables and responsive charts.
- **Admin workspaces:** keep navigation persistent on desktop, switch to a horizontal scrollable nav on mobile, and prioritize tables, filters, dialogs, and optimistic status feedback over decorative panels.

## Motion And Feedback

- Global color and surface transitions are short (about 200ms). Dialogs and menus use brief fade/zoom transitions around 100-200ms. Page entrances may use 0.2-0.4s fade/translate with restrained staggering.
- Ambient floating and brand motion may be slow (about 4-6s), but it must remain subtle. Use `useReducedMotion`, `motion-safe`, or `motion-reduce` for all non-essential motion.
- Loading uses the shared circular spinner, `LoadingOverlay`, button `loading`, skeletons, or Sonner loading toasts. Keep the underlying layout stable while loading.
- Use Sonner for async success, warning, error, and progress feedback. Give long-running operations stable toast IDs so progress updates replace rather than duplicate notifications.
- Hover states should change color, opacity, shadow, or underline; avoid scale jumps that move neighboring content.

## Responsive And Accessibility Requirements

- Design mobile first. Check narrow widths, tablet widths, and a wide desktop before considering the UI complete. Prefer wrapping, scroll areas, and stacked controls over clipped text.
- Preserve keyboard focus rings, semantic headings, form labels, button types, link semantics, and disabled behavior. Tooltips supplement but do not replace accessible names.
- Respect `prefers-reduced-motion`; no essential information may depend on animation. Ensure status is conveyed by text or icon plus color, never color alone.
- Keep all copy translation-ready through the existing i18n system. Avoid hard-coded user-facing strings when the surrounding page already uses `react-i18next`.

## Implementation Checklist

1. Inspect the nearest page and reusable component before writing markup.
2. Reuse the existing primitive and token; add a new primitive only when a repeated gap cannot be solved compositionally.
3. Implement all meaningful states: default, hover, focus, disabled, loading, empty, error, and success where applicable.
4. Verify light and dark themes, narrow and wide layouts, long translated text, and reduced motion.
5. Run the relevant checks from `web/` (`pnpm check` and/or `pnpm tsc:build`) and inspect the rendered result when the change is visual.

## Avoid

- New gradients, neon palettes, heavy glass effects, oversized marketing heroes, or decorative cards unrelated to the CTF workflow.
- Hard-coded hex colors that bypass semantic tokens, inconsistent radii, or a second icon library.
- Replacing dense operational views with large empty cards or excessive explanatory copy.
- Hiding important actions behind unfamiliar icon-only controls without labels/tooltips.
- Motion that continues to distract, shifts layout, or ignores reduced-motion preferences.
