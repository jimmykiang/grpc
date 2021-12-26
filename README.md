##Run helloworld

cargo run --bin helloworld-server
cargo run --bin helloworld-client

##Run server side TLS

cargo run --bin serverside-tls-server
cargo run --bin serverside-tls-client

##Run mutual TLS

cargo run --bin mutual-tls-server
cargo run --bin mutual-tls-client

##Certificate generation.
from the /tls folder run:

```bash
./gen.sh
```