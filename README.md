<!--
## output into next input
print -z "echo"

## todo
- search the way how to treat raw string on zsh print
    -> u can ignore the escape sequence with a argument `-r`.
-->

# cmd-drawer

## setup

```zsh
function drawer() {
  if local output=$(cmd-drawer); then
    print -z -r "${output}" #INFO -z: output into next prompt, -r: ignore escape sequence
  else
    echo "Err: failed to generate a commit message"
  fi
}
```
