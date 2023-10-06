# chameleon-rs

Prototypes on polymorphic, metamorphic and poly-metamorphic malwares in Rust.

## Disclaimer

This project is for educational purposes only.
It has been made to get a better understanding of polymorphic and metamorphic code concepts.

Quartz and its members are not responsible for your usage of this project.

## Acknowledgments

This project is inspired by the great work of the PoC Innovation team on the [WhiteComet-Research](https://github.com/pocinnovation/whitecomet-research) 
project.

Go check their work as well as their other projects to learn more about innovative topics!

## Introduction

A unique yet very elegant technique for a malware to avoid being detected by anti-viruses is to 
alter its signature.

There are many ways to achieve this:
- **Polymorphism**: The dedicated page on Wikipedia explains the concept very well, "_the code changes itself 
  every time it runs, but the function of the code (its semantics) will not change at all_".


- **Metamorphism**: When the malware edits and rewrites its own code each time it is run. Here 
  the semantics changes, but the injected code can be composed of [NOPs](https://en.wikipedia.org/wiki/NOP_(code))
  only.


- **Poly-Metamorphism**: Combining both previous techniques, the malware will change **and** 
  encrypt its own code each time it is run.

In this project, `chameleon-rs`, we developed prototypes for each one of the techniques 
described above.

To make things more challenging, we used the Rust programming language to demonstrate the 
capacity of using modern tools with a lot of external support available.
That is, shaping malwares that could use modern third-party libraries (the Rust crates).

## Getting Started

To be able to test all the malwares on all platforms, we wrote a [Dockerfile](./Dockerfile) for 
a simple container which builds the binaries.

You can build and run it using the following commands:
```shell
docker build . -t chameleon-rs
docker run --rm -it chameleon-rs
```

Once in the container, execute the malware as many times as you wish. In the following example, 
we compute the sha256 hash for the binary (which must change between each execution):
```shell
# Initial binary hash.
sha256sum ./target/debug/polymorphic #549a821e28b6dd03e6d430852447d6f7b425f2e26da3eab49e044c86a53cf59b

# Execute the binary for the first time.
./target/debug/polymorphic

# New binary hash, malware signature has changed.
sha256sum ./target/debug/polymorphic #e0479c6b7d2af8b5f6ccf55561e4ba400653450dccd8871f8b558fcf29fa1cb3
```

### Polymorphic Malware

> The sources for this prototype are available in the [`polymorphic`](./polymorphic) folder.

Want to learn more about how it works ? Check out the dedicated [README](./polymorphic/README.md) file !

### Metamorphic Malware

> Coming soon !

### Poly-Metamorphic Malware

> Coming soon !

## Conclusion

We learned a lot during the development of those prototypes.
Go on and take a look at the code, try to understand what it does, how things could be improved, 
invent new ways to prevent those attacks.

## Authors

Made with 🔐 and ❤️ by the 🦎 at [Quartz](https://quartz.technology).