# ai.tape — the AI-bar money shot for the README + Twitter.
#
# Story: open employees.csv, press Ctrl+K, type a question in English,
# Enter, watch the generated SQL appear under the bar and the table flip
# to the answer.
#
# Requires an API key. We default to Anthropic for speed; export your
# key before rendering:
#
#   export ANTHROPIC_API_KEY=sk-ant-...
#   cd /Users/josharsh/Development/pipemagic
#   vhs tapes/ai.tape          # writes tapes/out/ai.gif
#
# If you don't want to spend real tokens, set OPENAI_API_KEY pointing at a
# stub server that returns "SELECT * FROM data LIMIT 5" — but the real call
# tells a more honest story.

Output tapes/out/ai.gif

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
Type "cd /Users/josharsh/Development/pipemagic && clear"
Enter
Sleep 400ms
Show

Type "./target/release/xeli examples/employees.csv"
Sleep 200ms
Enter
Sleep 1500ms

# Open AI bar
Ctrl+K
Sleep 800ms

# Type the question naturally — slower so viewers can read along
Set TypingSpeed 80ms
Type "show me top 5 cities by total salary for active employees"
Sleep 800ms
Set TypingSpeed 50ms

# Send it. Sleep covers ~1-2s of model latency.
Enter
Sleep 3500ms

# Linger on the result so it lands
Sleep 2000ms

Type "q"
