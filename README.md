# http-bellhop

http-bellhop 0.1.0
HTTP Bellhop CLI tool for API testing

USAGE:
app [OPTIONS]

FLAGS:
-h, --help Prints help information
-V, --version Prints version information

OPTIONS:
-d, --dir <dir> location of dir with json files to run
-e, --env <env> what env setup should be used
-f, --file <file> location of json file to run

```
bellhop -e dev -d ./requests
```

```
bellhop --env dev --dir ./requests
```

```
bellhop --env dev --file ./requests/localhost/test.json
```

Refactoring 4. **dialoguer** - For interactive user prompts, confirmations, and selections

```toml
dialoguer = "0.10"
```

5. **indicatif** - For beautiful progress bars and spinners

   ```toml
   indicatif = "0.17"
   ```

6. **console** - For working with terminal colors and styling

   ```toml
   console = "0.15"
   ```

7. **log** with **env_logger** - For simple logging
   ```toml
   log = "0.4"
   env_logger = "0.10"
   ```
