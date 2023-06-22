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
    println!("Ты ответил, на интересующий мне вопрос...");
    wait(4);
    println!("Можем приступать...");
    wait(2);
    crd();
    sdl2::mixer::Music::fade_out(2_000).unwrap();
    println!("???\n{name}! {name}!!!");
    crd();
    wait(2);
    println!("Я проснулся, первое что увидели мои глаза - маму с недовольной физиономией.");
    wait(4);
    crd();
    println!("Мама\nТы опять до 4-х играл?!");
    crd();
    wait(3);
    println!("Что мне ей ответить?");
    wait(3);
    crd();
    println!("Таинственный голос\nПохоже,");
    wait(1);
    println!("что настало время...");
    sdl2::mixer::Music::halt();
    wait(2);
    pressingPursuit.play(-1).unwrap();
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
            sdl2::mixer::Music::fade_out(2_000).unwrap();
            wait(2);
            crd();
            println!("Мама\nСколько раз мне говорить, чтобы ты ложился спать вовремя?!");
            crd();
            wait(3);
            println!("{name}\nДа понял я, понял...");
            sdl2::mixer::Music::halt();
            wait(1);
            crd();
            println!("Мама\nНЕТ! НЕ ПОНЯЛ!");
            wait(1);
            println!("ЕСЛИ ТЫ СЕГОДНЯ ТАКЖЕ НЕ БУДЕШЬ ЛОЖИТЬСЯ СПАТЬ ВОВРЕМЯ, МНЕ ПРИДЁТСЯ ОТОБРАТЬ У ТЕБЯ КОМПЬЮТЕР НА ВСЁ ЛЕТО!");
            crd();
            wait(3);
            println!("Я знал что она так не поступит, максимум только на месяц...");
            wait(2);
            println!("Поэтому я не боялся.");
            crd();
            println!("{name}\nХорошо, мам... Я лягу сегодня по раньше... Обещаю!");
            crd();
            wait(3);
            println!("Моя мама фыркнула и ушла из комнаты захлопнув дверь.");
            wait(2);
            break;
        } else if FirstAnswer == "B" {
            crd();
            println!("{name}\nПОШЛА ТЫ! СЕГОДНЯ ВЫХОДНОЙ, ТАК ЧТО ЕСЛИ ТЫ ХОЧЕШЬ ЧТОБЫ Я СПАЛ, Я ПОСПЛЮ ТОЛЬКО ЕСЛИ НЕ БУДУ СЛУШАТЬ ТВОИ ЛЕКЦИИ, ПРОВАЛИВАЙ!");
            crd();
            sdl2::mixer::Music::fade_out(2_000).unwrap();
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
            wait(2);
            sdl2::mixer::Music::halt();
            break;
        } else {
            println!("Это не ответ.");
        }
    }

    println!("Я встал с кровати и подошёл к компьютеру.");
    wait(3);
    println!("Я заметил, что я забыл выключить компьютер перед тем как лечь спать, видимо, устал.");
    wait(3);
    println!("Подвинув мышкой, экран за светился.");
    wait(2);
    println!("Вчера я играл в Манирафт - обычная песочница про кубики.");
    wait(3);
    println!("Прокрутив камерой, я заметил кое что странное...");
    wait(2);
    println!("Постройку в виде какого-то символа... Я не помню чтобы строил её.");
    wait(3);
    print!("Ну и ладно.");
    wait(1);
    println!(" Всеровно это украшает мой мир.");
    wait(2);
    print!("Я выключил компьютер,");
    wait(2);
    println!(" лучше подышу свежим воздухом.");


}

#[inline]
fn wait(t:usize) { sleep(Duration::from_secs(t)); }

#[inline]
fn crd(){ println!("--------------------"); }

#[inline]
fn msc(){ println!("##############################################"); }
