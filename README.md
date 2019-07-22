# jump-kun
Directory jumper made by Rust

## Why jump-kun?

Can you count how many times did you typed ```cd``` on your teminal?

Typing..

```cd \where\you\wanna\go ```

everytime is cumbersome.

Just use jump-kun.

```
TODO: ADD GIF ANIMES HERE...
```

(This crate is heavyly inspired by [b4b4r07's enhancd](https://github.com/b4b4r07/enhancd))

# How to install

## Run cargo install

```cargo install jump-kun```

### (or clone this repo then...) 
```cargo install --path .```


## Add following script to your zsh.rc

(Might work on fish or bash, but not sure...)

```
function jump-kun-jump(){
    local selected=$(jump-kun)
    if [[ -n $selected ]]; then
        \cd $selected
    fi
}
```

Then type ``` jump-kun-jump ``` on your terminal.
