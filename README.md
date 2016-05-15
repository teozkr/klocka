# Klocka

## Motivation

Well, I recently got [an awful (cheap) doorbell](http://www.clasohlson.com/se/Tr%C3%A5dl%C3%B6s-d%C3%B6rrklocka/36-6016) from Clas Ohlson. As it turns out, my AA batteries wouldn't even fit inside the receiver, and the speaker sounds awful. But, what if I could hook up the receiver to a RPi instead? Hmm...

## Usage

When compiled, this should create a single self-contained binary, which listens
on port 8080 for the web interface and binds to the GPIO pin 22. To register
your browser, go to http://localhost:8080/ and approve the permission request.
If you want to be able to register a remote device, you'll need to put a SSL
terminating reverse proxy in front of this, since browsers for some reason don't
support push notifications from unencrypted websites.

## Browser support

Currently only Google Chrome is supported. Firefox has support for Push, but requires a
custom encryption scheme that I haven't got around to, yet. No other browsers
currently implement the Push API required for this to function.

## Customizing

If you don't want to make it bind any GPIO pins, set the environment variable
`KLOCKA_TRIGGER` to `FD`, which will make it trigger on newlines in stdin instead. If you want to change the pin it uses (22 by default),
change `GPIO_PIN` in `server/src/trigger/mod.rs`. The port for the web UI is set as `WEB_PORT` in `server/src/main.rs`.

## Building (client)

In `web-client/manifest.json`, change `gcm_sender_id` to your Google project ID.

## Building (server)

Create a file called `server/gcm_key.txt` containing **ONLY** your Google Cloud
Messaging key. Afterwards you should be able to `cargo run` (from the `server`
directory).

### Cross-compiling for the RPi 2/3

You'll need:

* A `rustc` target (install with `rustup target add armv7-unknown-linux-gnueabihf`)
* A GCC for linking to the target platform (on Arch, this means `armv7-unknown-linux-gnueabihf` from the AUR)
* OpenSSL for the target platform (the easiest way is to copy `openssl-build-arm.sh` to the OpenSSL source directory, adapt the paths inside it, and run it)

Afterwards, you'll need to set up your `~/.cargo/config` so it knows about your
GCC install, and tell `server/crossbuild-arm.sh` about your OpenSSL install
directory. `~/.cargo/config` should contain at least the following:

```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

Finally, run `server/crossbuild-arm.sh` to compile!
