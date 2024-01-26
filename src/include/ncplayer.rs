
use crate::*;

struct Ncplayer{
    threadssenders: HashMap<String,mpsc::Sender<String>>,
    durations: HashMap<String,Duration>,
    instants: HashMap<String,Instant>,
    allsoundids: String,
    soundint:usize,
}

impl Ncplayer{
    pub fn new() -> Ncplayer {
        Ncplayer {
            threadssenders: HashMap::new(),
            durations: HashMap::new(),
            instants: HashMap::new(),
            allsoundids:  String::new(),
            soundint: 0
        }
    }

    pub fn playfile(&mut self,filepath: &str)->String{
        // begins the whole things,
        let path = Path::new(&filepath);
        // get and set duration
        let duration = mp3_duration::from_path(&path).unwrap();
        let thisid = self.play(filepath.to_string());
        self.durations.insert(thisid.clone().to_string(),duration);
        //Add to activesoundbuffer and set timer
        self.allsoundids = "".to_owned() + &self.allsoundids + &thisid + "|";
        let start_time = Instant::now();
        self.instants.insert(thisid.clone(),start_time);
        thisid
    }
    pub fn duration(&mut self,id:&str)->Duration{
        if let Some(ret) = self.durations.get(id){
            return ret.to_owned();
        }
        Duration::from_secs(0)
    }
    pub fn elapsed(&mut self,id: &str)->Duration{
        if let Some(elapsed_time) = self.instants.get(id) {
            if let Some(duration) = self.durations.get(id) {
                Duration::from(elapsed_time.elapsed());
            }
        }
       Duration::from_secs(0)

    }
    pub fn runtimers(&mut self){
        // use this inside your apps loops, this will handle the sounds by killing the threads when
        if self.allsoundids == "" {
            return
        }
        //stop() String buffer
        let mut tostop = String::new();
        // get array of all active sounds and check their timers
        let array: Vec<&str>= self.allsoundids.split("|").collect();
          for musicid in  array{
            if let Some(lastelapsed_time) = self.instants.get(musicid) {
                if let Some(duration) = self.durations.get(musicid) {
                    if lastelapsed_time.elapsed() >= duration.to_owned() {
                        // Stringbuffer with the music ids which will be stopped.
                        tostop = tostop + musicid +"|";
                    }
                }
            }

        }
        // handle stop() buffer.
        if tostop != ""{
            let len = tostop.len();
            if Ncplayer::fromright(&tostop,1) == "|"{
                tostop = String::from(&tostop[0..len-1]);
            }
            let array: Vec<&str>= tostop.split("|").collect();
            for stopid in array{
                self.stop(&stopid);

            }
        }

    }

     fn fromright(s: &str, f: usize) -> String {
        // strips a number of chrs from a string
        let len = s.len();
        if f < len {
            return String::from(&s[len - f..len]);
        } else {
            return String::new();
        }
    }

    fn create_soundid(&mut self)->String{
        //1k sounds max, gonna get crazy resets itself so play 1000 at the same time max
        self.soundint = self.soundint + 1;
            if self.soundint > 999{
                self.soundint = 0;
            }
        // create a unique id to search and replace
        // since we counting we add a prefix and suffix cause we do replacements on strings.
        let newid = "[".to_owned()+ &self.soundint.to_string() + "]";
        newid
    }

    fn create_thread(&mut self) -> (mpsc::Sender<String>,mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel();
        (tx, rx)
    }

    fn play(&mut self,soundfile:String) -> String{
        //interally used to spawn threads to play the audio
        let soundfilecl = soundfile.clone();
        let (tx, rx)  = self.create_thread();
        thread::spawn(move || {
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&handle).unwrap();
            let file = std::fs::File::open(soundfilecl.clone()).unwrap();
            let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(decoder);
            loop {

                if sink.empty() {
                    break;
                }
                let received_message = rx.recv().unwrap();
                match received_message.as_str(){
                    "stop" => {
                      break;
                    }
                    _ =>{}
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        let thisid = self.create_soundid();
        self.threadssenders.insert(thisid.clone().to_string(),tx);
        thisid
    }

    fn stop(&mut self,id:&str){
        // used to stop active sounds,
        if let Some(sender) = self.threadssenders.get(id) {
            sender.send("stop".to_string());
            let torep = "".to_owned() + &id +"|";
            self.allsoundids = self.allsoundids.replace(&torep,"");
        }
    }
}
//
// fn main() {
//     let mut play = nscriptsound::new();
//     play.playfile("./boom.mp3");
// thread::sleep(Duration::from_secs(1));
//      play.playfile("./boom.mp3");
// thread::sleep(Duration::from_secs(1));
//    play.playfile("./boom.mp3");
// thread::sleep(Duration::from_secs(1));
//
//     loop {
//         //thread::sleep(Duration::from_secs(3));
//         //play.stop("stop");
//         //println!("oi");
//         //break;
//     play.runtimers();
//     }
// }
//


