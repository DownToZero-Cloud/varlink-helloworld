# varlink hello world

## running the server

```bash
cargo run
```

## testing via varlinkctl

**generic information about the service**
```bash
varlinkctl info /tmp/hello.varlink 

    Vendor: DownToZero
   Product: hello-world
   Version: 1.0.0
       URL: https://github.com/DownToZero-Cloud/varlink-helloworld
Interfaces: org.varlink.service
            rocks.dtz.HelloWorld
```
**calling Hello without a paramter**
```bash
varlinkctl call /tmp/hello.varlink rocks.dtz.HelloWorld.Hello {}
{
        "message" : "Hello, World!"
}
```
**calling NamedHello with a parameter**
```bash
varlinkctl call /tmp/hello.varlink rocks.dtz.HelloWorld.NamedHello '{"name":"jens"}'
{
        "message" : "Hello, jens!"
}
```
