### Usage of command:
```
rpar [RPAR ARGUMENTS] -- COMMAND [COMMAND ARGUMENTS]
```

### Examples of usage:

1)
```bash
    ./rpar --times 10 -silent -- ls -l
```
or
```bash
    cargo run -- -t 10 -s -- ls -l
```
2)
```bash
    ./rpar -t 1 -u -- curl www.google.com
```

### rpar arguments:
```
arg  --times [TIMES] /  -t     - to execute command [TIMES] times
                                 Default: command is executed 1 time

arg  --silent        /  -s     - to suppress output.
                                 Default: output is not suppressed

arg  --unparallel    /  -u     - to execute command in sequential way.
                                 Default: command is executed in a parallel way
```
