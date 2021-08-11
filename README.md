# i_said_we_should_date
A utility to run executables as if it were a given date/time

Uses `LD_PRELOAD` to load this library and hijack `time()` syscalls to feed the caller the specified timestamp.

There are 2 components: `preload` is the binary to be run directly (per usage below) and `libld_preload_rust.so` is the dynamic library that is loaded before your desired child executable is run.  The `.so` must be in the same directory as `preload`.

The desired date/time is passed to `preload` as the first argument and can be a Unix epoch timestamp or an RFC3339 string.

The second argument is the path to the child executable.

### Usage

```
./preload timestamp child_binary

e.g.:

    ./preload 2147483647 /path/to/my/binary
    ./preload 2038-01-19T03:14:07Z /path/to/my/binary
    ./preload 2001-12-31T23:59:59-05:00 /path/to/my/binary
```

`simple.c` is included as a simple executable that prints the current date and time, for testing.
