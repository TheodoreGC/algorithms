# CS Fundamentals in Rust

<a href="https://www.buymeacoffee.com/theodoregc" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-red.png" alt="Buy Me A Beer" height="41" width="174"></a>

*This project is inspired by [rustlings](https://github.com/rust-lang/rustlings).*
*I love the idea to provide a "framework" for learning Rust by doing and wanted to provide a similar experience for learning CS fundamentals in Rust. As the goal of [rustlings](https://github.com/rust-lang/rustlings) is mostly focusing on syntax and features Rust provides, I decided to create this project oriented exclusively on algorithms, CS concepts and data structures.*

*I strongly recommend going trough the [rustlings](https://github.com/rust-lang/rustlings) before getting started with this project.*

This project provides a playground where to implement algorithms, CS concepts and data structures in Rust. This includes reading and responding to compiler messages!

## Getting Started

*Note: If you're on MacOS, make sure you've installed Xcode and its developer tools by typing `xcode-select --install`.*

You will need to have Rust installed. You can get it by visiting <https://rustup.rs>. This will also install Cargo, Rust's package/project manager.

## MacOS/Linux

Just run:

```bash
curl -L https://raw.githubusercontent.com/TheodoreGC/rust-cs-fundamentals/main/install.sh | bash
# Or if you want it to be installed to a different path:
curl -L https://raw.githubusercontent.com/TheodoreGC/rust-cs-fundamentals/main/install.sh | bash -s mypath/
```

This will install the project and give you access to the `algo` command. Run it to get started!

## Windows

In PowerShell, set `ExecutionPolicy` to `RemoteSigned`:

```ps
Set-ExecutionPolicy RemoteSigned
```

Then, you can run:

```ps
Start-BitsTransfer -Source https://raw.githubusercontent.com/rust-cs-fundamentals/main/install.ps1 -Destination $env:TMP/install_algo.ps1; Unblock-File $env:TMP/install_algo.ps1; Invoke-Expression $env:TMP/install_algo.ps1
```

To install the project. Same as on MacOS/Linux, you will have access to the `algo` command after it.

## Manually

Basically: Clone the repository, checkout to the latest tag, run `cargo install`.

```bash
git clone https://github.com/TheodoreGC/rust-cs-fundamentals
cd rust-cs-fundamentals
git checkout tags/4.2.0 # or whatever the latest version is (find out at https://github.com/TheodoreGC/rust-cs-fundamentals/releases/latest)
cargo install --force --path .
```

If there are installation errors, ensure that your toolchain is up to date. For the latest, run:

```bash
rustup update
```

Then, same as above, run `algo` to get started.

## Writing the exercises

The exercises are sorted by topic and can be found in the subdirectory `rust-cs-fundamentals/fundamentals/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. It is highly recommend that you have a look at them before you start.

The task is simple. The exercises are empty and all yours to implement. Some exercises are also run as tests, but exercises handles them all the same. To run the exercises in the recommended order, execute:

```bash
algo watch
```

This will try to verify the completion of every exercise in a predetermined order. It will also rerun automatically every time you change a file in the `rust-cs-fundamentals/` directory. If you want to only run it once, you can use:

```bash
algo verify
```

This will do the same as watch, but it'll quit after running.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
algo run Algo1
```

In case you get stuck, you can run the following command to get a hint for your
exercise:

``` bash
algo hint Algo1
```
