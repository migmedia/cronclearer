# cronclearer

This is the command execution wrapper called "cronclearer" that helps monitor and analyze
command execution, particularly useful for debugging cron jobs.

This tool would be particularly useful for:
- Debugging cron jobs that fail
- Analyzing command execution problems
- Getting detailed execution information when commands fail
- Capturing and organizing command output in a structured way

The name "cronclearer" should suggest it's intended for making cron job outputs clearer and
easier to debug, as cron job failures can sometimes be difficult to diagnose due to limited
logging and output capture.

## Command Line Options:

   - `-h` or `--help`: Shows usage information
   - `-i` or `--ignore-text`: Ignores stderr when exit-code is zero.


## Author
Micha Glave

## Prior Art

* [cronic](https://habilis.net/cronic/): A bash-script by Chuck Houpt. I took the basic design idea
  and most implementation details from.
* `2>/dev/null`: The evil cousin. Ignoring all failures.

