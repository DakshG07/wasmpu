@echo off
git add .
FOR /F "tokens=* USEBACKQ" %%F IN (`gum choose "fix" "feat" "docs" "style" "refactor" "test" "chore" "revert"`) DO (
SET type=%%F
)
FOR /F "tokens=* USEBACKQ" %%F IN (`gum input --value %type%": " --placeholder "Summary of change"`) DO (
SET msg=%%F
)
FOR /F "tokens=* USEBACKQ" %%F IN (`gum write --placeholder "Description of change"`) DO (
SET desc=%%F
)
gum confirm "Commit changes?" && git commit -m "%msg%" -m "%desc%"


