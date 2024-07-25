A reimplementation of Walter Fontana's Alchemy. Pipe lambda expressions into 
`stdin` to start a default simulation. 

Usage:

`alchemy`

Build: 

`cargo build`

Testing:

`cargo run -- {args}`

With the binary tree generators from the 
[lambda-btree](https://github.com/AgentElement/lambda-btree) crate, you can
run a simple alchemy simulation with the following command.

`python /path/to/src/fontana_generator.py | cargo run -- {args}`


Documentation:

* Full documentation: `cargo doc --open`
* Help: `cargo run -- --help`

The documentation for the configuration file is in the `Config` object.

# Python Library WIP

There is a python library for this code. To develop run the following commands:

1) Make a virtual env `python -m venv .env`
2) Activate venv `source .env/bin/activate`
3) install maturin `pip install maturin`
4) develop `maturin develop`

If this all goes according to plan you can import the library like this

```
import alchemy
alchemy.sum_as_string(20,5) # '25'
