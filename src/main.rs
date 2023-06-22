#![allow(non_snake_case)]

use std::{time::Duration, thread::sleep};
use std::io;

use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

fn main() {
    let sdl = sdl2::init().unwrap();
    let _audio = sdl.audio().unwrap();

    let freq = 48000;
    let chunk_size = 1_024;

    sdl2::mixer::open_audio(freq, AUDIO_S16LSB, DEFAULT_CHANNELS, chunk_size).unwrap();
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3);

    sdl2::mixer::allocate_channels(4);
    sdl2::mixer::Music::set_volume(50);

    let projector = sdl2::mixer::Music::from_file("src/Projector.mp3").unwrap();

    wait(3);
    projector.play(0).unwrap();
    println!("   ______                       __   ");
    println!("  / ____/___  ____  _________  / /__ ");
    println!(r" / /   / __ \/ __ \/ ___/ __ \/ / _ \");
    println!("/ /___/ /_/ / / / (__  ) /_/ / /  __/");
    println!(r"\____/\____/_/ /_/____/\____/_/\___/ ");
    println!("| |  / (_)______  ______ _/ /        ");
    println!("| | / / / ___/ / / / __ `/ /         ");
    println!("| |/ / (__  ) /_/ / /_/ / /          ");
    println!(r"|___/_/____/\__,_/\__,_/_/__         ");
    println!("   / | / /___ _   _____  / /         ");
    println!(r"  /  |/ / __ \ | / / _ \/ /          ");
    println!(" / /|  / /_/ / |/ /  __/ /           ");
    println!(r"/_/ |_/\____/|___/\___/_/            ");
    println!("");
    wait(4);
    sdl2::mixer::Music::halt();
 
    let anotherHim = sdl2::mixer::Music::from_file("src/ANOTHER HIM.mp3").unwrap();
    let pressingPursuit = sdl2::mixer::Music::from_file("src/Pressing Pursuit.mp3").unwrap();


    anotherHim.play(-1).unwrap();
    let mut name = String::new();

    msc();
    println!("Deltarune OST: 1 - ANOTHER HIM");
    msc();
    crd();
    paw!(2, "Таинственный голос\nПривет.");
    paw!(2, "Перед тем как мы начнём...");
    paw!(3, "Я бы хотел задать тебе один вопрос.");
    println!("Какого твоё имя?");
    crd();

    io::stdin()
        .read_line(&mut name)
        .expect("ОШИБКА : Невозможно прочесть ответ...");
    let name = name.trim_end();

    crd();
    paw!(2, "Таинственный голос\n{}? Интересно...", name);
    paw!(1, "Очень...");
    paw!(4, "Интересно......");
    paw!(3, "Что-ж, {name}.");
    paw!(4, "Ты ответил, на интересующий мне вопрос...");
    paw!(2, "Можем приступать...");
    crd();
    sdl2::mixer::Music::fade_out(2_000).unwrap();
    println!("???\n{name}! {name}!!!");
    crd();
    wait(2);
    paw!(4, "Я проснулся, первое что увидели мои глаза - маму с недовольной физиономией.");
    crd();
    println!("Мама\nТы опять до 4-х играл?!");
    crd();
    wait(3);
    paw!(3, "Что мне ей ответить?");
    crd();
    paw!(1, "Таинственный голос\nПохоже,");
    sdl2::mixer::Music::halt();
    paw!(2, "что настало время...");
    pressingPursuit.play(-1).unwrap();
    paw!(1, "ВЫБОРА!");
    msc();
    println!("Ace Attorney OST - Pressing Pursuit ~ Cornered - Variation");
    msc();
    wait(2);
    crd();
    paw!(1, "Ваша кровь ускорилась, вы напряглись.");
    paw!(2, "Раздумывая над ответами, вы решаете...");
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
            sdl2::mixer::Music::fade_out(2_000).unwrap();
            paw!(2, "Да мам, извини...");
            crd();
            println!("Мама\nСколько раз мне говорить, чтобы ты ложился спать вовремя?!");
            crd();
            wait(3);
            sdl2::mixer::Music::halt();
            paw!(1, "{name}\nДа понял я, понял...");
            crd();
            paw!(1, "Мама\nНЕТ! НЕ ПОНЯЛ!");
            println!("ЕСЛИ ТЫ СЕГОДНЯ ТАКЖЕ НЕ БУДЕШЬ ЛОЖИТЬСЯ СПАТЬ ВОВРЕМЯ, МНЕ ПРИДЁТСЯ ОТОБРАТЬ У ТЕБЯ КОМПЬЮТЕР НА ВСЁ ЛЕТО!");
            crd();
            wait(3);
            paw!(2, "Я знал что она так не поступит, максимум только на месяц...");
            println!("Поэтому я не боялся.");
            crd();
            println!("{name}\nХорошо, мам... Я лягу сегодня по раньше... Обещаю!");
            crd();
            wait(3);
            paw!(2, "Моя мама фыркнула и ушла из комнаты захлопнув дверь.");
            break;
        } else if FirstAnswer == "B" {
            crd();
            println!("{name}\nПОШЛА ТЫ! СЕГОДНЯ ВЫХОДНОЙ, ТАК ЧТО ЕСЛИ ТЫ ХОЧЕШЬ ЧТОБЫ Я СПАЛ, Я ПОСПЛЮ ТОЛЬКО ЕСЛИ НЕ БУДУ СЛУШАТЬ ТВОИ ЛЕКЦИИ, ПРОВАЛИВАЙ!");
            crd();
            sdl2::mixer::Music::fade_out(2_000).unwrap();
            wait(2);
            paw!(3, "После того как я осознал что я сказал... Было поздно.");
            paw!(2, "Моя мама стояла передо мной с ошарашенным лицом.");
            paw!(3, "Я забоялся, что она психанёт, отберёт у меня компьютер на месяц, накажет...");
            paw!(2, "Но она тихо и медленно ушла, закрыв мою дверь.");
            paw!(2, "Я успокоился...");
            sdl2::mixer::Music::halt();
            break;
        } else {
            println!("Это не ответ.");
        }
    }

    paw!(3, "Я встал с кровати и подошёл к компьютеру.");
    paw!(3, "Я заметил, что я забыл выключить компьютер перед тем как лечь спать, видимо, устал.");
    paw!(2, "Подвинув мышкой, экран за светился.");
    paw!(3, "Вчера я играл в Манирафт - обычная песочница про кубики.");
    paw!(2, "Прокрутив камерой, я заметил кое что странное...");
    paw!(3, "Постройку в виде какого-то символа... Я не помню чтобы строил её.");
    pnaw!(1, "Ну и ладно.");
    paw!(2, " Всеровно это украшает мой мир.");
    paw!(2, "Я выключил компьютер,");
    println!(" лучше подышу свежим воздухом.");
}

#[macro_export]
macro_rules! paw {
    ($t:expr, $($v:expr),*) => {
        println!($($v),*);
        wait($t);
    };
}

#[macro_export]
macro_rules! pnaw {
    ($t:expr, $($v:expr),*) => {
        print!($($v),*);
        wait($t);
    };
}

#[inline]
fn wait(t:u64) { sleep(Duration::from_secs(t)); }

#[inline]
fn crd(){ println!("--------------------"); }

#[inline]
fn msc(){ println!("##############################################"); }
