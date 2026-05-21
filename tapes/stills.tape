# stills.tape — captures static PNG screenshots at key moments. Use these
# for the Twitter thread (one screenshot per tweet), Product Hunt gallery,
# and embed images in launch posts.
#
# vhs's `Screenshot` directive writes a PNG of the current frame.
#
# Render:
#   cd /Users/josharsh/Development/xeli
#   vhs tapes/stills.tape       # writes screenshots into tapes/out/stills/

Output tapes/out/stills/_dummy.gif    # vhs requires an Output even when we only want stills

Set Theme "Dracula"
Set FontSize 16
Set FontFamily "JetBrainsMono Nerd Font"
Set Width 1280
Set Height 720
Set Padding 24
Set Framerate 30
Set Shell zsh

Hide
Type "cd /Users/josharsh/Development/xeli && clear"
Enter
Sleep 400ms
Show

Type "./target/release/xeli examples/employees.csv"
Sleep 200ms
Enter
Sleep 1500ms

# ---- 1. Normal table view ----
Screenshot tapes/out/stills/01-table.png

# ---- 2. AI bar awaiting input ----
Ctrl+K
Sleep 600ms
Type "average salary by city for active employees"
Sleep 800ms
Screenshot tapes/out/stills/02-ai-bar.png

# Cancel (don't actually send — keeps stills deterministic)
Escape
Sleep 400ms

# ---- 3. Group-by wizard — pick stage ----
Type "g"
Sleep 500ms
Type "jjj"        # cursor to "department"
Sleep 200ms
Space             # select
Sleep 200ms
Screenshot tapes/out/stills/03-groupby-pick.png

# ---- 4. Group-by wizard — aggregation stage ----
Enter
Sleep 500ms
Type "jjjj"       # salary
Sleep 200ms
Type "ll"         # AVG
Sleep 300ms
Screenshot tapes/out/stills/04-groupby-agg.png

# Apply
Enter
Sleep 1500ms

# ---- 5. Grouped result ----
Screenshot tapes/out/stills/05-groupby-result.png

# ---- 6. Persistent key hints close-up ---- (already in every shot
# but worth one explicit shot where nothing else is happening)
Type "q"
Sleep 300ms
Type "./target/release/xeli examples/employees.csv"
Enter
Sleep 1500ms
Screenshot tapes/out/stills/06-key-hints.png

Type "q"
