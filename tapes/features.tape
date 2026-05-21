# features.tape — a 25-second feature montage. Good for the website hero,
# Product Hunt gallery, and the "what does it do?" section of long posts.
#
# Story: filter → search → join. No AI in this tape — deterministic.
#
# Render:
#   cd /Users/josharsh/Development/xeli
#   vhs tapes/features.tape    # writes tapes/out/features.gif

Output tapes/out/features.gif

Set Theme "Dracula"
Set FontSize 16
Set FontFamily "JetBrainsMono Nerd Font"
Set Width 1280
Set Height 720
Set Padding 24
Set Framerate 30
Set PlaybackSpeed 1.0
Set TypingSpeed 50ms
Set Shell zsh

Hide
Type "cd /Users/josharsh/Development/xeli && clear"
Enter
Sleep 400ms
Show

Type "./target/release/xeli examples/employees.csv"
Sleep 200ms
Enter
Sleep 1200ms

# ---- Filter: only show Engineering ----
Type "f"
Sleep 600ms
# Column "department" is idx 3
Type "jjj"
Sleep 300ms
Enter           # advance to operator
Sleep 500ms
Enter           # default operator is "="
Sleep 500ms
Type "Engineering"
Sleep 400ms
Enter           # apply filter
Sleep 1500ms

# Clear filter
Type "F"
Sleep 1000ms

# ---- Regex search: anyone starting with "C" ----
Type "/"
Sleep 400ms
Type "^C"
Sleep 800ms
Enter
Sleep 1200ms
Escape
Sleep 600ms

# ---- Sort salary descending ----
Type "lll"
Sleep 200ms
Type "s"
Sleep 400ms
Type "s"
Sleep 1500ms

# ---- Join with departments.csv ----
# Reset cursor first
Type "gg"
Sleep 300ms

Type "J"
Sleep 600ms
Type "examples/departments.csv"
Sleep 800ms
Enter            # load second file
Sleep 1200ms
Enter            # accept INNER join (default)
Sleep 800ms
# Pick column "department" on the left (idx 3)
Type "jjj"
Sleep 300ms
Tab              # switch side
Sleep 400ms
# departments.csv column 0 is "department"
Enter            # apply join
Sleep 2500ms

# Linger
Sleep 1500ms

Type "q"
