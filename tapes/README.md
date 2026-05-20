# tapes/ — deterministic GIF + screenshot recording

This directory has [vhs](https://github.com/charmbracelet/vhs) scripts that produce reproducible recordings of xeli for the README, social posts, and Product Hunt gallery.

## Why vhs

Hand-recorded GIFs are subtly wrong every time — different typing speed, different mouse drift, different terminal width, different theme. vhs runs a virtual terminal, types your script at exact intervals, and writes a GIF (or PNG) you can regenerate in CI.

## One-time setup

```bash
brew install vhs

# vhs uses ttyd + ffmpeg under the hood; brew pulls them in.
# Make sure JetBrains Mono is installed if you want it to match:
brew install --cask font-jetbrains-mono-nerd-font
```

Also make sure xeli is built:

```bash
cd /Users/josharsh/Development/pipemagic
cargo build --release
```

## Tapes in this directory

| File | Output | Story | Needs API key? |
|---|---|---|---|
| `hero.tape` | `out/hero.gif` (15s) | File picker → open CSV → navigate → sort → group-by wizard → result | No |
| `ai.tape` | `out/ai.gif` (10s) | Open CSV → Ctrl+K → "show me top 5 cities by total salary for active employees" → generated SQL + result | **Yes — `ANTHROPIC_API_KEY`** |
| `features.tape` | `out/features.gif` (25s) | Filter → search → sort → join wizard | No |
| `stills.tape` | `out/stills/*.png` (6 screenshots) | Static frames for Twitter / PH gallery | No |

## Render everything

```bash
cd /Users/josharsh/Development/pipemagic

# Deterministic tapes (no API calls, free, repeatable)
vhs tapes/hero.tape
vhs tapes/features.tape
vhs tapes/stills.tape

# AI tape — one real Anthropic Sonnet call per render (~$0.001)
export ANTHROPIC_API_KEY=sk-ant-...
vhs tapes/ai.tape
```

Outputs land in `tapes/out/`. That directory is gitignored — commit GIFs you want to use by `git add -f tapes/out/hero.gif` or by symlinking to your social-media tooling folder.

## Tweaking

If a tape times wrong (the model takes 3s instead of 2s, your machine is slower, etc.), edit the `Sleep` values. The directives that matter:

- `Type "..."` — types literal characters at `TypingSpeed` (default 60ms/char)
- `Enter`, `Tab`, `Escape`, `Space`, `Ctrl+K`, `Ctrl+d`, `Ctrl+u` — key actions
- `Sleep <ms|s>` — pause (the only thing you'll edit when tuning)
- `Hide` / `Show` — wrap setup commands you don't want in the output
- `Screenshot path/to.png` — capture a still PNG of the current frame

Full reference: <https://github.com/charmbracelet/vhs/blob/main/README.md>

## Where the GIFs end up

Suggested embeds:

- **README.md** hero — `out/hero.gif`
- **Twitter thread tweet 1** — `out/hero.gif` (or `out/ai.gif` if leading with AI)
- **Twitter thread tweet 2** — `out/stills/02-ai-bar.png`
- **Twitter thread tweet 3** — `out/stills/03-groupby-pick.png`
- **xeli.vercel.app hero** — `out/hero.gif` (replace the ASCII placeholder block in `website/app/page.tsx`)
- **Product Hunt gallery** — all 6 PNGs from `out/stills/`

## Tip: trim GIFs for Twitter

Twitter prefers ≤15s video over GIF. If `out/hero.gif` is borderline, convert with:

```bash
ffmpeg -i tapes/out/hero.gif -movflags faststart -pix_fmt yuv420p \
       -vf "scale=trunc(iw/2)*2:trunc(ih/2)*2" tapes/out/hero.mp4
```

The MP4 will be ~10× smaller than the GIF and Twitter renders it as auto-playing video.
