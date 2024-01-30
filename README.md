## Ncplayer 26-01-2024

So this lib plays soundfiles using threads the struct manages the threads and allows you to stop sounds.
the threads will stop themselves if you add the structs .runtimers() function from your struct.
this function checks all the timers for the active sounds and when they are finished it will signal the threads and shut them down.

so theres a few functions you can use, first off you spawn the struct

special notes: alot of soundfiles give errors from the used libs, to avoid these make sure the mp3 filenmames are stripped of special characters and spaces underscores do work.
however im not entirely sure what this issue is about but, "music_song_blabla.mp3" like this.

```rust
fn main() {
    // first off spawn the player struct.
    let mut mysoundplayer = Ncplayer::new();

    // play some soundfiles at the same time with a slight starting delay
    mysoundplayer.playfile("./boom.mp3");
    thread::sleep(Duration::from_secs(1));
    mysoundplayer.playfile("./boom.mp3");
    thread::sleep(Duration::from_secs(1));

    // getfileid is a String which contains the soundstructs info. use to control your sound.
    let getfileid = mysoundplayer.playfile("./boom.mp3");
    thread::sleep(Duration::from_secs(1));

    //set soundvolume of a thread goes 1.0 = 100% can be increased or decreased
    mysoundplayer.setvolume(getfileid,0.8);

    //mute
    mysoundplayer.mute(getfileid);

    //unmute
    mysoundplayer.unmute(getfileid);


    loop {
        thread::sleep(Duration::from_secs(3));

        // to stop a sound before it finishes
        mysoundplayer.stop(getfileid);

        // this is required to shutdown threads when their sounds are played. it removes the threads properly
        mysoundplayer.runtimers();
    }
}
```
the .playfile()  simply returns the soundid as a String, you can pass it to .stop(soundidstring)
