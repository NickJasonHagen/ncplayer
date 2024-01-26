## Ncplayer 26-01-2024

So this lib plays soundfiles using threads the struct manages the threads and allows you to stop sounds.
the threads will stop themselves if you add the structs .runtimers() function from your struct.
this function checks all the timers for the active sounds and when they are finished it will signal the threads and shut them down.

so theres a few functions you can use, first off you spawn the struct
```rust
fn main() {
    let mut mysoundplayer = Ncplayer::new();
    mysoundplayer.playfile("./boom.mp3");
    thread::sleep(Duration::from_secs(1));
    mysoundplayer.playfile("./boom.mp3");
    thread::sleep(Duration::from_secs(1));
    let getfileid = mysoundplayer.playfile("./boom.mp3");
    thread::sleep(Duration::from_secs(1));

    loop {
        thread::sleep(Duration::from_secs(3));
        mysoundplayer.stop(getfileid);
        mysoundplayer.runtimers();
    }
}
```
the .playfile()  simply returns the soundid as a String, you can pass it to .stop(soundidstring)
