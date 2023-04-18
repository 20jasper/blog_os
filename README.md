# blog_os

This is my repo for following along with the [Writing an OS in Rust series](https://os.phil-opp.com/) by [@phil-opp](https://github.com/phil-opp)

- [blog\_os](#blog_os)
	- [Usage](#usage)
		- [Build project](#build-project)
		- [Create bootable image](#create-bootable-image)
		- [Run in QEMU](#run-in-qemu)


## Usage
### Build project

To build the project, run `cargo build`

### Create bootable image
To create a bootable image, install bootimage
```bash
cargo install bootimage
```
And then run `cargo bootimage`

### Run in QEMU

```bash
cargo run
```
