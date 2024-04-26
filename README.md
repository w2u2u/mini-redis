# Mini Redis

Implementation for mini in-memory storage server.

## Preview
```
->> [Server]   Running on 127.0.0.1:8080
->> ==================================================
->> [Server]   Accept     TCP Connection
->> [Handler]  Received   b"SET apple banana\n"
->> [Database] LocalDB    Set
->> [Database] Key        apple
->> [Database] Value      banana
->> [Database] {
    "apple": "banana",
}
->> [Handler]  Response   OK
->> ==================================================
->> [Server]   Accept     TCP Connection
->> [Handler]  Received   b"GET apple\n"
->> [Database] {
    "apple": "banana",
}
->> [Database] LocalDB    Get
->> [Database] Key        apple
->> [Database] Value      banana
->> [Handler]  Response   banana
```

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
