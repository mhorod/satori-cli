![satori-cli](logo.png)

# satori-cli

Simple tool for command line interaction with [Satori](https://satori.tcs.uj.edu.pl/) judge system at TCS, JU.

# usage (TODO)

In order not to spam the server with requests and to speed things up, a cache is used.

Every command that retrieves some state (thus caches it) can be used with `--force` option to perform the request anyway.


## contests
List available contests
```
$ satori contests
```
Additional flags:
- `--archived` includes archived contests

## problems
```
$ satori problems <contest>
```
`contest` can be any prefix of the contest name.
If this prefix is ambiguous an error occurs

## pdf
Download pdf for the file
```
$ satori pdf <problem> <contest>
```

## submit
Submit file to a problem
```
$ satori submit <file> <problem> <contest>
```

## status
Get status of solving the problem - either best (by default) or latest submission.
```
$ satori status <problem> <contest>
```
Additional flags:
- `--best` - prioritizes highest numerical value. If results are `OK/ANS/TLE` then prioritizes `OK` or the last result
- `--recent` - prioritizes status of the most recent submit

## results
Get results of recent submisions
```
$ satori results <contest>
```
```
$ satori results <contest> <problem>
```

Additional flags:
- `--limit <limit>` - how many results to return at maximum


## logout
If, for whatever reason you want to log out
```
$ satori logout
```
This logouts from the website as well as deletes any stored information.

# configuration (TODO)
You can create `satori-cli.toml` in working directory to simplify commands a bit.
Every named parameter can be set in the configuration file becoming the default value.


```
contest = "contest"
```
will set default value of `contest` option of every command to `"contest"`

Similarly you can write
```
problem = "problem"
file = "file"
```

### Example
Consider `satori.toml` set to
```
contest = "Python"
problem = "A"
file = "main.py"
```
Running
```
$ satori pdf
```
will successfully download pdf of problem `A` from `Python` contest.

and running
```
$ satori submit
```
will submit file `main.py` to problem `A` in contest `Python`