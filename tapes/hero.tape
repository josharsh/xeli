# hero.tape — the 15-second landing GIF for the README + Twitter.
#
# Story: open the file picker, fuzzy-find the sample, navigate, run the
# group-by wizard, see the answer. No AI in this tape — fully deterministic,
# no API key required.
#
# Render:
#   cd /Users/josharsh/Development/pipemagic
#   vhs tapes/hero.tape          # writes tapes/out/hero.gif

Output tapes/out/hero.gif

# Terminal look — matches the website's Dracula palette
Set Theme "Dracula"
Set FontSize 16
Set FontFamily "JetBrainsMono Nerd Font"
Set Width 1280
Set Height 720
Set Padding 24
Set Framerate 30
Set PlaybackSpeed 1.0
Set TypingSpeed 60ms
Set Shell zsh

# Use the freshly built binary, not whatever's in PATH
Hide
Type "cd /Users/josharsh/Development/pipemagic && clear"
Enter
Sleep 500ms
Show

# Open with no args → file picker
Type "./target/release/xeli"
Sleep 300ms
Enter
Sleep 1500ms

# Fuzzy-search the sample
Type "empl"
Sleep 1s

# Open it
Enter
Sleep 1500ms

# Quick tour: move down, right, scroll a page
Type "jjjjj"
Sleep 400ms
Type "lll"
Sleep 400ms
Ctrl+d
Sleep 600ms
Ctrl+u
Sleep 400ms

# Sort the salary column descending — first go to it
Type "ll"
Sleep 200ms
Type "s"
Sleep 600ms
Type "s"      # cycle to DESC
Sleep 1200ms

# Open group-by wizard
Type "g"
Sleep 800ms

# Pick "department" (idx 3 — id=0, name=1, city=2, department=3)
Type "jjj"
Sleep 300ms
Space          # toggle select
Sleep 500ms
Enter          # advance to aggregation stage
Sleep 800ms

# In aggregation stage: salary column is at idx 4
Type "jjjj"
Sleep 300ms
# AVG is at idx 2 (COUNT, SUM, AVG, MIN, MAX) — cycle right twice
Type "ll"
Sleep 400ms
Enter
Sleep 2500ms

# Linger on the grouped result
Sleep 1500ms

# Bow out cleanly
Type "q"
