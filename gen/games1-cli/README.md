<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `games1` command-line interface *(CLI)* allows to use most features of the *Google Games* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Games* API can be found at the
[official documentation site](https://developers.google.com/games/services/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-games1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/games1-cli).

# Usage

This documentation was generated from the *Games* API at revision *20190627*. The CLI is at version *1.0.11*.

```bash
games1 [options]
        achievement-definitions
                list [-p <v>]... [-o <out>]
        achievements
                increment <achievement-id> <steps-to-increment> [-p <v>]... [-o <out>]
                list <player-id> [-p <v>]... [-o <out>]
                reveal <achievement-id> [-p <v>]... [-o <out>]
                set-steps-at-least <achievement-id> <steps> [-p <v>]... [-o <out>]
                unlock <achievement-id> [-p <v>]... [-o <out>]
                update-multiple (-r <kv>)... [-p <v>]... [-o <out>]
        applications
                get <application-id> [-p <v>]... [-o <out>]
                played [-p <v>]...
                verify <application-id> [-p <v>]... [-o <out>]
        events
                list-by-player [-p <v>]... [-o <out>]
                list-definitions [-p <v>]... [-o <out>]
                record (-r <kv>)... [-p <v>]... [-o <out>]
        leaderboards
                get <leaderboard-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
        metagame
                get-metagame-config [-p <v>]... [-o <out>]
                list-categories-by-player <player-id> <collection> [-p <v>]... [-o <out>]
        players
                get <player-id> [-p <v>]... [-o <out>]
                list <collection> [-p <v>]... [-o <out>]
        pushtokens
                remove (-r <kv>)... [-p <v>]...
                update (-r <kv>)... [-p <v>]...
        quest-milestones
                claim <quest-id> <milestone-id> <request-id> [-p <v>]...
        quests
                accept <quest-id> [-p <v>]... [-o <out>]
                list <player-id> [-p <v>]... [-o <out>]
        revisions
                check <client-revision> [-p <v>]... [-o <out>]
        rooms
                create (-r <kv>)... [-p <v>]... [-o <out>]
                decline <room-id> [-p <v>]... [-o <out>]
                dismiss <room-id> [-p <v>]...
                get <room-id> [-p <v>]... [-o <out>]
                join <room-id> (-r <kv>)... [-p <v>]... [-o <out>]
                leave <room-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                report-status <room-id> (-r <kv>)... [-p <v>]... [-o <out>]
        scores
                get <player-id> <leaderboard-id> <time-span> [-p <v>]... [-o <out>]
                list <leaderboard-id> <collection> <time-span> [-p <v>]... [-o <out>]
                list-window <leaderboard-id> <collection> <time-span> [-p <v>]... [-o <out>]
                submit <leaderboard-id> <score> [-p <v>]... [-o <out>]
                submit-multiple (-r <kv>)... [-p <v>]... [-o <out>]
        snapshots
                get <snapshot-id> [-p <v>]... [-o <out>]
                list <player-id> [-p <v>]... [-o <out>]
        turn-based-matches
                cancel <match-id> [-p <v>]...
                create (-r <kv>)... [-p <v>]... [-o <out>]
                decline <match-id> [-p <v>]... [-o <out>]
                dismiss <match-id> [-p <v>]...
                finish <match-id> (-r <kv>)... [-p <v>]... [-o <out>]
                get <match-id> [-p <v>]... [-o <out>]
                join <match-id> [-p <v>]... [-o <out>]
                leave <match-id> [-p <v>]... [-o <out>]
                leave-turn <match-id> <match-version> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                rematch <match-id> [-p <v>]... [-o <out>]
                sync [-p <v>]... [-o <out>]
                take-turn <match-id> (-r <kv>)... [-p <v>]... [-o <out>]
  games1 --help

Configuration:
  [--scope <url>]...
            Specify the authentication a method should be executed in. Each scope
            requires the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to
            a user-writable directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
  --debug
            Output all server communication to standard error. `tx` and `rx` are placed
            into the same stream.
  --debug-auth
            Output all communication related to authentication to standard error. `tx`
            and `rx` are placed into the same stream.

```

# Configuration

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `games1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

More information about the various kinds of persistent data are given in the following paragraphs.

# Authentication

Most APIs require a user to authenticate any request. If this is the case, the [scope][scopes] determines the 
set of permissions granted. The granularity of these is usually no more than *read-only* or *full-access*.

If not set, the system will automatically select the smallest feasible scope, e.g. when invoking a
method that is read-only, it will ask only for a read-only scope. 
You may use the `--scope` flag to specify a scope directly. 
All applicable scopes are documented in the respective method's CLI documentation.

The first time a scope is used, the user is asked for permission. Follow the instructions given 
by the CLI to grant permissions, or to decline.

If a scope was authenticated by the user, the respective information will be stored as *JSON* in the configuration
directory, e.g. `~/.google-service-cli/games1-token-<scope-hash>.json`. No manual management of these tokens
is necessary.

To revoke granted authentication, please refer to the [official documentation][revoke-access].

# Application Secrets

In order to allow any application to use Google services, it will need to be registered using the 
[Google Developer Console][google-dev-console]. APIs the application may use are then enabled for it
one by one. Most APIs can be used for free and have a daily quota.

To allow more comfortable usage of the CLI without forcing anyone to register an own application, the CLI
comes with a default application secret that is configured accordingly. This also means that heavy usage
all around the world may deplete the daily quota.

You can workaround this limitation by putting your own secrets file at this location: 
`~/.google-service-cli/games1-secret.json`, assuming that the required *games* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print all client-server communication to standard error, whereas the `--debug-auth` flag
will cause all communication related to authentication to standard error.
If the `--debug` flag is set, error-results will be debug-printed, possibly yielding more information about the 
issue at hand.

You may consider redirecting standard error into a file for ease of use, e.g. `games1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/