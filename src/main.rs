use console_utils::{input::select, input::input};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::time;//::duration
use std::thread;

//#[derive(Parser)]
//struct Pomo{
//    focus:i32,
//    rest:i32,
//}

fn init_bar(bar_color:&str)->ProgressBar{
    let bar = ProgressBar::new(600);
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
    println!("{}", time);
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


fn set_bar(time_focus:i32, bar_color:&str){
    //logic for visual bar
    let bar = init_bar(bar_color);
    let sleepy = time::Duration::from_millis(999);
    //println!("{}",timeFocus);
    let period = if time_focus*60 < 600 {600/(time_focus*60)} else {(time_focus*60)/600};
    //start loop for incr time
    for i in 1..time_focus * 60{ //TODO: fix calc to be consistent maybe floats that check how much
        //to add to each line
        if i%period == 0{
            incr_bar(&bar, 1);
        }else{
            incr_bar(&bar, 0);
        }
        thread::sleep(sleepy);
    }
    bar.finish();

}

fn main() {

    //get focus times
    let  time_focus = get_time([">25 Minutes", ">30 Minutes", ">40 Minutes", ">Custom"], true);
    //get break times
    let time_break = get_time([">5 Minutes", ">10 Minutes", ">15 Minutes", ">Custom"], false);
    loop{
        set_bar(time_focus,"cyan");
        set_bar(time_break, "210");
        let options = [">Continue", ">Quit"];
        let selected_index = select("<------>", &options);
        if selected_index != 0{break;}
    }


    println!("Hello, world!");

}
