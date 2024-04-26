# Mini Redis

Implementation for mini in-memory storage server.

## Start development server
```
$ cargo watch -q -c -w src/ -x "run -q"

->> [Server]   Running on 127.0.0.1:8080

```

## Send command to the server
### Set value
```
$ curl telnet://127.0.0.1:8080/ <<< "SET apple banana"

OK
```

### Get value
```
$ curl telnet://127.0.0.1:8080/ <<< "GET apple"

banana
```
