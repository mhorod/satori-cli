![satori-cli](logo.png)

# satori-cli

Simple tool for command line interaction with [Satori](https://satori.tcs.uj.edu.pl/) judge system at TCS, JU.

### Stored data
The app stores only token and cached data about contests, problems, and results.

In particular your login and password are **not** stored anywhere on your computer. You will be asked to enter them every time the token expires.

# usage 
## list joined contests
```
$ satori-cli contests
```

## list problems in a contest
```
$ satori-cli problems -c <contest>
```
`<contest>` can be any prefix of either id (number in the url) or short name (found in the contests panel).

In case of ambiguity you will be prompted to choose from matches.

## list results of submits
```
$ satori-cli results -c <contest> [-p <problem>] [-l <limit>]
```

If no problem is specified results of all problems are displayed.
If limit is not specified default satori limit is used.

Again, in case of ambiguity you will be asked to resolve it manually.

## view details of a submit
```
$ satori-cli details -c <contest> -s <submit id>
```

`submit id` has to be exact since it's not searched from all submits.

## logout
```
$ satori-cli logout
```
Removes all stored data, including token.

## username
```
$ satori-cli username
```

Shows your username (probably name and surname) if you are currently logged in.

## help
```
$ satori-cli help
```


# TODO
## cache
Satori is so slow we should better cache what we know to execute fewer requests.

## commands
- `pdf` - download pdf of the problem statement
- `status` - search submits and determine raw score on a problem
- `submit` - send a solution to verify

## configuration
For convenience user should be able to create a `satori-cli.toml` configuration file in working directory and set default values for command fields.

It would be particularly useful while submitting a solution -- instead of typing contest, problem, and file they could be read from the configuration.