#![allow(non_snake_case)]

mod sound;

use std::{time::Duration, thread::sleep};
use std::io;

use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

fn sdl_init() {
    let sdl = sdl2::init().unwrap();
    let _audio = sdl.audio().unwrap();

    let freq = 44_100;
    let chunk_size = 1_024;

    sdl2::mixer::open_audio(freq, AUDIO_S16LSB, DEFAULT_CHANNELS, chunk_size).unwrap();
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3);

    sdl2::mixer::allocate_channels(4);
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let _audio = sdl.audio().unwrap();

    let freq = 44_100;
    let chunk_size = 1_024;

    sdl2::mixer::open_audio(freq, AUDIO_S16LSB, DEFAULT_CHANNELS, chunk_size).unwrap();
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3);

    sdl2::mixer::allocate_channels(4);

    let music = sdl2::mixer::Music::from_file("src/ANOTHER HIM.mp3").unwrap();
    sdl2::mixer::Music::set_volume(64);

    music.play(-1).unwrap();
    let mut name = String::new();
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
        .expect("ОШИБКА : Невозможно прочесть ответ...");
    let name = name.trim_end();

    crd();
    println!("Таинственный голос\n{}? Интересно...", name);
    wait(2);
    println!("Очень...");
    wait(1);
    println!("Интересно......");
    wait(4);
    println!("Что-ж, {name}.");
    wait(3);
    println!("Ты овтетил, на интересующий мне вопрос...");
    wait(4);
    println!("Можем приступать...");
    wait(2);
    crd();
    sdl2::mixer::Music::pause();
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
    wait(2);
    println!("ВЫБОРА!");
    wait(1);
    msc();
    println!("Ace Attorney OST - Pressing Pursuit ~ Cornered - Variation");
    msc();
    wait(2);
    crd();
    println!("Ваша кровь ускорилась, вы напряглись.");
    wait(1);
    println!("Раздумывая над ответами, вы решаете...");
    wait(2);
    crd();
    println!("Напишите Английскую A = Сказать 'Да' и извиниться.");
    println!("Напишите Английскую B = Прогнать из комнаты.");


    loop{

        let mut FirstAnswer:String = String::new();
        io::stdin()
            .read_line(&mut FirstAnswer)
            .expect("ОШИБКА : Невозможно прочесть ответ...");
        let FirstAnswer = FirstAnswer.trim_end();


        if FirstAnswer == "A" {
            println!("Да мам, извини...");

        } else if FirstAnswer == "B" {
            println!("ПОШЛА ОТСЮДА ВОН! РАЗ У НАС СЕГОДНЯ ВЫХОДНОЙ... Я МОГУ ПОСПАТЬ ЕЩЁ ЧАС, И МНЕ ОБСАЛЮТНО НАСРАТЬ НА ТВОИ ЛЕКЦИИ, ПОНЯЛА?!");
            wait(2);
            println!("После того как я осознал что я сказал... Было поздно.");
            wait(3);
            println!("Моя мама стояла передо мной с ошарашенным лицом.");
            wait(2);
            println!("Я забоялся, что она психанёт, отберёт у меня компьютер на месяц, накажет...");
            wait(3);
            println!("Но она тихо и медленно ушла, закрыв мою дверь.");
            wait(2);
            println!("Я успокоился...");
            break;
        } else {
            println!("Это не ответ.");
        }
    }
}

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
