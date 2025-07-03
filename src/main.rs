#![feature(thread_sleep_until)]
use console_utils::{input::select, input::input};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::process::exit;
use std::time;//::duration
use std::thread;
use std::time::Instant;
use k_board::termio::{restore, setup_raw_mode, termios};
use std::io::{Read, Write};


pub fn get_key() -> std::io::Result<u32>  {
    let termios_enviroment: termios = setup_raw_mode()?;
    std::io::stdout().flush().unwrap();
    let mut buffer: [u8; 3] = [0; 3];
    #[allow(clippy::unused_io_amount)]
    std::io::stdin().read(&mut buffer)?;
    let mut ret:u32 = 0;
    if buffer[0] != 0x00 {
        /*println!(
            "[0x{:x?}, 0x{:x?}, 0x{:x?}]",
            buffer[0], buffer[1], buffer[2]
        );*/
        if buffer[0] == 0x71{ret = 1;} //checks for q
        if buffer[0] == 0x70{ret = 2;} //checks for p 
        if buffer[0] == 0x73{ret = 3;} //checks for s
    }
    std::io::stdout().flush().unwrap();
    restore(&termios_enviroment)?; 
    Ok(ret)
}

fn init_bar(bar_color:&str)->ProgressBar{
    let bar = ProgressBar::new(600);
    //add percent for debug [{percent:>.cyan}]
    let format = "{bar:>80.".to_owned() + bar_color +"/61} [{elapsed:>.cyan}] {msg:>.12}";
    bar.set_style(ProgressStyle::with_template(format.as_str())//alt color 210
    .unwrap());
    //.progress_chars("##-"));
    bar.set_message("\n>q to quit\n>p to pause\n>s to skip");
    bar
}

fn incr_bar(bar:&ProgressBar, forward:u64){
    bar.inc(forward);
    
}

fn get_time(options:[&str;4], switcher: bool)->i32{
    //let options = [">25 Minutes", ">30 Minutes", ">40 Minutes", ">Custom"];
    let selected_index = select(">Select A Focus Time<", &options);
    let time: i32;
    if selected_index > 2{
         let mut temp: String = input("Enter  your time: ");
         //println!("{}",temp.len());
         temp.truncate(temp.len() - 1);
         time = temp.parse().unwrap();
    } else {
         time = if switcher {check_focus(selected_index)} else {check_break(selected_index)};    
    }
    time
}

fn check_focus(input: usize)->i32{
    match input{
        0=> {25}
        1=> {30}
        2=> {40}
        _=> {30}
    }
}
fn check_break(input: usize)->i32{
    match input{
        0=> {5}
        1=> {10}
        2=> {15}
        _=> {20}
    }
}

fn event_handler(event: u32)->bool{
    if event == 1{
        //quit = 1;
        exit(0);
    } else if event == 2{
        let sleepy = time::Duration::from_millis(200);
        loop{
            let p = get_key().unwrap();
            if p==2{
                break;
            }else if p ==1{
                //quit = 1;
                exit(0);
                //break;
            }else if p == 3{
                return true;
            }
            thread::sleep(sleepy);
        }
    }else if event==3{
        return true;
    }

    false
}

fn set_bar(time_focus:i32, bar_color:&str){
    //logic for visual bar
    //println!("{}",time_focus as f64*120.0/600.0);
    let bar = init_bar(bar_color);
    let sleepy = time::Duration::from_millis(500);
    let period:f64 = if time_focus*120 < 600 {600.0/(time_focus as f64*120.0)} else {(time_focus as f64*120.0)/600.0};
    //start loop for incr time
    let mut progress = period; 
    let mut next_start = Instant::now();
    for i in 1..time_focus * 120{ //TODO: fix calc to be consistent maybe floats that check how much
        next_start+=sleepy;
        thread::sleep_until(next_start);
        //to add to each line
        if i as f64>=progress{
            incr_bar(&bar, 1);
            progress += period;
        }else{
            incr_bar(&bar, 0);
        }
        //println!("{}", progress);
        //keyboard inturupt handling
        let event = get_key().unwrap();
        if event != 0{
            if event_handler(event){
                break;
            }
            next_start = Instant::now();
        }        
    
    }
    bar.finish();

}

fn main() {
    //kept all in main due to small program
    //get focus times
    let  time_focus = get_time([">25 Minutes", ">30 Minutes", ">40 Minutes", ">Custom"], true);
    //get break times
    let time_break = get_time([">5 Minutes", ">10 Minutes", ">15 Minutes", ">Custom"], false);
    //start main loop for program
    loop{
       
        set_bar(time_focus,"cyan");
        //if quit!=0{exit(0);} 
        set_bar(time_break, "210");
        //if quit!=0{exit(0)}
        let options = [">Continue", ">Quit"];
        let selected_index = select("<------>", &options);
        if selected_index != 0{break;}
    }


    //println!("Hello, world!");

}
