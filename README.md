#OxyFuel-Scribe

## Installation
1. Clone repo
2. Use `cargo` to install the tool (make sure, that `/~.cargo/bin/` is in PATH)
```
cargo install --path .
```
## Usage 
Standalone scribe scripts can be written much like bash scripts:
```
#!/home/ivan/.cargo/bin/scribe

TYPE 'echo hello world' <Enter>;
...
```

Or commands can be serialized into one-liners and piped to the Scribe:
```
echo "TYPE 'echo hello world' <Enter>;" | scribe
```

See [examples](./examples) for integration in bash scripts or standalone scripts.

## Scribe Scripts
Scripts consist of commands, separated by either ';' or '\n'. Available commands are `TYPE`, `WAITFOR`, `SLEEP`.

### TYPE command 
Used for simulating user input, incl. pressing, releasing and clicking keys.

```
TYPE [string|<key>|vkeyv|^key^]...;
```
- Strings will be typed verbatim and must be in quotes/double quotes. Only printable characters
- <key> clicks a key or a modifier
- vkeyv presses a key or a modifier
- ^key^ releases a key or a modifier

Names of individual keys that can be pressed, released, or clicked, can be found by:
```
scribe [--keylist|--modlist] | grep '(T)'
```

### WAITFOR command
Halts script execution, until a specified keyboard shortcut is pressed.

```
WAITFOR string;
```
String is a valid combination of "modifier+modifier+...+modifier+key"
- modifiers must be as per https://w3c.github.io/uievents-key/#keys-modifier
- key must be as per https://w3c.github.io/uievents-code/#code-value-tables

Names of individual keys and modifiers that can be used to register a keyboard-shortcut can be found by:
```
scribe [--keylist|--modlist] | grep '(W)'
```

### SLEEP command
```
SLEEP <miliseconds>
```
