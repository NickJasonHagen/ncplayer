use std::io::BufReader;
use std::time::Duration;
//use rodio::Source;
use std::sync::mpsc;
use std::collections::HashMap;
//use media::Media;
//extern crate id3;
use std::thread;
use mp3_duration;
use std::path::Path;
//use cpal::traits::{DeviceTrait, StreamTrait};
use std::time::Instant;

struct nscriptsound{
    threadssenders: HashMap<String,mpsc::Sender<String>>,
    durations: HashMap<String,Duration>,
    //timers: HashMap<String,Duration>,
    instants: HashMap<String,Instant>,
    allsoundids: String,
    soundint:usize,
}

impl nscriptsound{
    pub fn new() -> nscriptsound {
        nscriptsound {
            threadssenders: HashMap::new(),
            durations: HashMap::new(),
            //timers: HashMap::new(),
            instants: HashMap::new(),
            allsoundids:  String::new(),
            soundint: 0
        }
    }

    pub fn playfile(&mut self,filepath: &str){
        let path = Path::new(&filepath);
        let duration = mp3_duration::from_path(&path).unwrap();
        let thisid = self.play(filepath.to_string());
        self.durations.insert(thisid.clone().to_string(),duration);

        self.allsoundids = "".to_owned() + &self.allsoundids + &thisid + "|";
        println!("adding:{}",thisid);
        //println!("arr:{}",self.allsoundids);
        let start_time = Instant::now();
        self.instants.insert(thisid.clone(),start_time);
        let elapsed_time = start_time.elapsed();
        //self.timers.insert(thisid.clone().to_string(),elapsed_time);
        //
        // loop {
        //     let elapsed_time = start_time.elapsed();
        //     //println!("elapsed {:?}",elapsed_time);
        //     if elapsed_time >= duration {
        //         //println!("Timer has elapsed after {:?} seconds!", elapsed_time.as_secs());
        //         self.stop("stop");
        //         break;
        //     }
        // }
    }
    pub fn runtimers(&mut self){
        if self.allsoundids == "" {
            return
        }
        let mut tostop = String::new();
        let array: Vec<&str>= self.allsoundids.split("|").collect();
//println!("arr:{}",self.allsoundids);

        for musicid in  array{
            //println!("entre:{}",&musicid);

            //let thisduration : &Duration;
            if let Some(lastelapsed_time) = self.instants.get(musicid) {
                if let Some(duration) = self.durations.get(musicid) {
                    //thisduration = lastelapsed_time;
                    if lastelapsed_time.elapsed() >= duration.to_owned() {

                        //self.timers.insert(musicid.clone().to_string(),thisduration.to_owned());
                        //println!("Timer has elapsed after {:?} seconds!", elapsed_time.as_secs());
                        tostop = tostop + musicid +"|";
                    }
                }
            }

        }
        if tostop != ""{
            let len = tostop.len();
            if nscriptsound::fromright(&tostop,1) == "|"{
                tostop = String::from(&tostop[0..len-1]);
            }
            let array: Vec<&str>= tostop.split("|").collect();
            for stopid in array{
                println!("removed soundthread {}",&stopid);
                self.stop(&stopid);

            }
        }

    }
     fn fromright(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len {
            return String::from(&s[len - f..len]);
        } else {
            return String::new();
        }
    }
    fn create_soundid(&mut self)->String{
        self.soundint = self.soundint + 1;
            if self.soundint > 999{
                self.soundint = 0;
            }
        // create a unique id to search and replace
        let newid = "[".to_owned()+ &self.soundint.to_string() + "]";

        newid
    }

    fn create_thread(&mut self) -> (mpsc::Sender<String>,mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel();
        // Do something with tx if needed
        (tx, rx)
    }
    fn play(&mut self,soundfile:String) -> String{
        let soundfilecl = soundfile.clone();
        let (tx, rx)  = self.create_thread();
        thread::spawn(move || {
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&handle).unwrap();
            //println!("file:{}",&soundfilecl);
            let file = std::fs::File::open(soundfilecl.clone()).unwrap();
            let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(decoder);
            //let playing = true;
            loop {

                if sink.empty() {
                    break;
                }
                let received_message = rx.recv().unwrap();
                match received_message.as_str(){
                    "stop" => {
                        //println!("stop signal received!");
                        //println!("leng:{}",sink.len());
                        break;
                    }
                    _ =>{}
                }
                thread::sleep(Duration::from_millis(10));
            }
        });


        tx.send("st".to_string()).unwrap();
       let thisid = self.create_soundid();

        self.threadssenders.insert(thisid.clone().to_string(),tx);
        thisid
    }
    fn stop(&mut self,id:&str){
        if let Some(sender) = self.threadssenders.get(id) {
            sender.send("stop".to_string());
                let torep = "".to_owned() + &id +"|";
                self.allsoundids = self.allsoundids.replace(&torep,"");
            }
    }
}

fn main() {
    let mut play = nscriptsound::new();
    play.playfile("./boom.mp3");
thread::sleep(Duration::from_secs(1));
     play.playfile("./boom.mp3");
thread::sleep(Duration::from_secs(1));
   play.playfile("./boom.mp3");
thread::sleep(Duration::from_secs(1));

    loop {
        //thread::sleep(Duration::from_secs(3));
        //play.stop("stop");
        //println!("oi");
        //break;
    play.runtimers();
    }
}



