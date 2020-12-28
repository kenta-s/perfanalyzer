## Perfanalyzer

Perfanalyzer analyzes Rails' log and finds slow pages.

## Usage

```
$ cargo run /path/to/development.log > result.json
```

Then you can see the result:

```
$ cat result.json | jq
```


Output values are sorted by `duration` in descending order

```
[
  {
    "page": "GET /foo/8n95a8nswff3dnc3zjjq06xvh",
    "controller": "FooController",
    "action": "show",
    "duration": 22407.42,
    "view": 0,
    "db": 316.04,
    "count": 1
  },
  {
    "page": "GET /foo/djvplfcb0jd5ogvvrxmeova1e",
    "controller": "FooController",
    "action": "show",
    "duration": 9337.97,
    "view": 15.68,
    "db": 409.32,
    "count": 2
  },

  ...

  {
    "page": "POST /bar",
    "controller": "BarController",
    "action": "create",
    "duration": 0.47889894,
    "view": 0,
    "db": 0,
    "count": 13
  }
]
```
