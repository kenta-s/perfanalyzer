## Perfanalyzer

Perfanalyzer analyzes Rails' log and find slow pages.

## Usage

```
$ cargo run src/main.rs /path/to/development.log > result.json
```

then you can find slow pages:

```
$ cat result.json | jq '.slow_pages'
```
