<!--
## output into next input
print -z "echo"

## todo
- search the way how to treat raw string on zsh print
    -> u can ignore the escape sequence with a argument `-r`.
-->

# cmd-drawer

A drawer to store ur commands.  
Recommended for people who write a alias but forget about it.

## setup

```zsh
function drawer() {
  if local output=$(cmd-drawer); then
    print -z -r "${output}" #INFO -z: output into next prompt, -r: ignore escape sequence
  else
    echo "Err: failed to take out a command."
  fi
}
```

## config

`~/.config/cmd-drawer/default-config.toml`

```toml
[[zsh]]
cmd = '''sudo nixos-rebuild switch'''
desc = '[nixos] update'

[[zsh]]
cmd = '''sudo -e /etc/nixos/configuration.nix'''
desc = '[nixos] edit config'

[[zsh]]
cmd = '''cd ~/.config/nixpkgs'''
desc = '[nixos] cd to nixpkgs'
```
