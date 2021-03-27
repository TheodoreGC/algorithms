# algorithms

<a href="https://www.buymeacoffee.com/theodoregc" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-red.png" alt="Buy Me A Beer" height="41" width="174"></a>

This project provides a playground where to implement the most common algorithms in Rust. This includes reading and responding to compiler messages!

## Getting Started

_Note: If you're on MacOS, make sure you've installed Xcode and its developer tools by typing `xcode-select --install`._

You will need to have Rust installed. You can get it by visiting https://rustup.rs. This'll also install Cargo, Rust's package/project manager.

## MacOS/Linux

Just run:

```bash
curl -L https://raw.githubusercontent.com/TheodoreGC/algorithms/master/install.sh | bash
# Or if you want it to be installed to a different path:
curl -L https://raw.githubusercontent.com/TheodoreGC/algorithms/master/install.sh | bash -s mypath/
```

This will install the project and give you access to the `algo` command. Run it to get started!

## Windows

In PowerShell, set `ExecutionPolicy` to `RemoteSigned`:

```ps
Set-ExecutionPolicy RemoteSigned
```

Then, you can run:

```ps
Start-BitsTransfer -Source https://raw.githubusercontent.com/algorithms/master/install.ps1 -Destination $env:TMP/install_algo.ps1; Unblock-File $env:TMP/install_algo.ps1; Invoke-Expression $env:TMP/install_algo.ps1
```

To install the project. Same as on MacOS/Linux, you will have access to the `algo` command after it.

## Manually

Basically: Clone the repository, checkout to the latest tag, run `cargo install`.

```bash
git clone https://github.com/TheodoreGC/algorithms
cd algorithms
git checkout tags/4.2.0 # or whatever the latest version is (find out at https://github.com/TheodoreGC/algorithms/releases/latest)
cargo install --force --path .
```

If there are installation errors, ensure that your toolchain is up to date. For the latest, run:
```bash
rustup update
```

Then, same as above, run `algo` to get started.
