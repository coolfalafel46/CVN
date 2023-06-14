#![allow(non_snake_case)]

use soloud::*;

use std::{time::Duration, thread::sleep};

use std::io;

use std::thread;

#[inline]
fn wait(t:u64){
    sleep(Duration::from_secs(t));
}

#[inline]
fn crd(){
    println!("--------------------");
}

#[inline]
fn msc(){
    println!("##############################################");
}

fn main() {
    let mut name = String::new();


    thread::spawn(|| {
        let sl = Soloud::default().unwrap();
        let mut wav = audio::Wav::default();
        wav.load_mem(include_bytes!("ANOTHER HIM.mp3")).unwrap();
        sl.play(&wav);
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    msc();
    println!("Deltarune OST: 1 - ANOTHER HIM");
    msc();
    crd();
    println!("Таинственный голос\nПривет.");
    wait(2);
    println!("Перед тем как мы начнём...");
    wait(2);
    println!("Я бы хотел задать тебе один вопрос.");
    wait(3);
    println!("Какого твоё имя?");
    crd();

    io::stdin()
        .read_line(&mut name)
        .expect("ЙОУ! ЕСЛИ ТЫ ЭТО ВИДИШЬ - ТЫ ГОВНОКОДЕР ЕБАНЫЙ!");
    let name = name.trim_end();

    crd();
    println!("Таинственный голос\n{}? Интересно...", name);
    wait(2);
    println!("Очень...");
    wait(1);
    println!("Интересно......");
    wait(4);
    println!("Хм... {name}...");
    wait(3);
    println!("{name}.....");
    wait(6);
    println!(".........{name}........");
    wait(10);
    println!("{name}...{name}...{name}...{name}...");
    wait(15);
    println!("Что-ж, {name}.");
    wait(3);
    println!("Ты овтетил, на интересующий мне вопрос...");
    wait(4);
    println!("Можем приступать...");
    wait(2);
    crd();
    println!("???\n{name}! {name}!!!");
    crd();
    wait(2);
    println!("Я проснулся, первое что увидели мои глаза - маму с недовольной физиономией.");
    wait(4);
    crd();
    println!("Мама\n{name} опять до 4-х играл?!");
    crd();
    wait(3);
    println!("Что мне ей ответить?");
    wait(3);
    crd();
    println!("Таинственный голос\nПохоже,");
    wait(1);
    println!("что настало время...");
    thread::spawn(|| {
        let sl = Soloud::default().unwrap();
        let mut wav = audio::Wav::default();
        wav.load_mem(include_bytes!("Pressing Pursuit.mp3")).unwrap();
        sl.play(&wav);
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    wait(2);
    println!("ВЫБОРА!");
    wait(1);
    msc();
    println!("Ace Attorney OST - Pressing Pursuit ~ Cornered - Variation");
    msc();



}