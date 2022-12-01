// fn main() {
//     // Copy a tracklist for a CD from Discogs,
//     // transform it to a form Exact Audio Copy understands
//     // artist/trackname
//     let text = "2-1	–David Et Jonathan	Bella Vita	4:29
//     2-2	–Cookie Dingler	Femme Libérée	3:40
//     2-3	–Didier Barbelivien	Elle	3:10
//     2-4	–C.Jerôme	Et Tu Danses Avec Lui	3:31
//     2-5	–Claude Barzotti	Elle Me Tue	3:51
//     2-6	–Michelle Torr	I Remember You	4:03
//     2-7	–Felix Gray	La Gitane	4:20
//     2-8	–Jean-Pierre François	Je Te Survivrai	4:19
//     2-9	–David Et Jonathan	Est-Ce Que Tu Viens Pour Les Vacances	4:24
//     2-10	–Didier Barbelivien	Elsa	3:14
//     2-11	–Mike Brant	Love Is A Feeling	4:13
//     2-12	–Claude Barzotti	Le Rital	3:27
//     2-13	–Felix Gray	Te Revoir A Madrid	3:42
//     2-14	–Eric Charden	Les Monde Est Gris, Le Monde Est Bleu	3:35
// ";
//     let lines: Vec<Vec<_>> = text.lines().map(|line| line.split("	").collect()).collect();
//     for line in lines {
//         let artist: String = line[1].chars().filter(|&c| c != '–').collect();
//         let track = line[2];
//         println!("{}/{}", artist, track);
//     }
//     // for mut line in text.lines().map(|line| line.split("	")) {
//     //     line.next();
//     //     let artist: String = line.next().unwrap().chars().filter(|&c| c != '–').collect();
//     //     let track = line.next().unwrap();
//     //     println!("{}/{}", artist, track);
//     // }
// }

#[derive(Debug, serde::Deserialize)]
struct Record {
    something: String,
    artist: String,
    track: String,
    length: String,
}

// Afscheid vol2 cd1
// 1-1	Marco Borsato–	Het Water	5:28
//     1-2	Natalia (3), Gabriel Rios–	Hallelujah	4:18
//     1-3	Gilbert Bécaud–	Et Maintenant	3:39
//     1-4	Il Divo–	The Lord's Prayer	2:51
//     1-5	Andrei Lugovski–	Ave Maria	4:53
//     1-6	Teatro–	Memory	3:56
//     1-7	Liesbeth List–	Laat Me Niet Alleen	3:13
//     1-8	Paul de Leeuw–	De Steen	2:42
//     1-9	Jan De Wilde–	De Eerste Sneeuw	4:42
//     1-10	Jo Lemaire–	Je Suis Venue Te Dire Que Je M' En Vais	3:39
//     1-11	Arno (2)–	Les Yeux De Ma Mère	3:39
//     1-12	Reinhard Mey–	Als De Dag Van Toen	4:34
//     1-13	Rob de Nijs–	Alleen Is Maar Alleen	2:46
//     1-14	André Hazes–	De Vlieger	3:52
//     1-15	Sweet People–	Et Les Oiseaux Chantaient	2:55
//     1-16	Simon & Garfunkel–	Bridge Over Troubled Water	4:52
//     1-17	Randy Vanwarmer–	Just When I Needed You Most	4:03
//     1-18	Michael Jackson–	One Day In Your Life	4:07
//     1-19	Céline Dion–	The Power Of Love	4:47
//     1-20	Whitney Houston–	I Will Always Love You	4:29

// Afscheid vol2 cd2
// 2-1	Bette Midler–	The Rose	3:32
// 2-2	Elvis Presley–	Love Me Tender	2:43
// 2-3	The Righteous Brothers–	Unchained Melody	3:36
// 2-4	Seal–	Stand By Me	4:02
// 2-5	Jim Reeves–	He'll Have To Go	2:19
// 2-6	Clouseau–	Ik Denk Aan Jou	3:57
// 2-7	Frank Boeijen, Stef Bos–	Twee Mannen Zo Stil	4:34
// 2-8	Volumia!–	Afscheid	4:35
// 2-9	Will Tura–	Hoop Doet Leven	4:05
// 2-10	Paul Young–	Everytime You Go Away	4:23
// 2-11	Roxette–	Spending My Time	4:35
// 2-12	Sinéad O'Connor–	Nothing Compares 2 U	5:01
// 2-13	Nilsson*–	Without You	3:17
// 2-14	Johnny Logan–	What's Another Year	2:57
// 2-15	The Pretenders–	I'll Stand By You	3:57
// 2-16	Editors–	No Sound But The Wind	3:49
// 2-17	dEUS–	Nothing Really Ends	4:58
// 2-18	Herman van Veen–	Weet Je Nog	3:07
// 2-19	Urbanus–	Als Ik Doodga	3:46
// 2-20	Boudewijn De Groot–	Avond	4:34

// Donna Amour 9 cd1
// 1-01	Kate Winslet–	What If	4:07
// 1-02	Robbie Williams–	Supreme	4:15
// 1-03	Geri Halliwell–	Calling	4:25
// 1-04	Blue (5)–	Too Close	3:45
// 1-05	Britney Spears–	Born To Make You Happy	3:35
// 1-06	Westlife–	Uptown Girl	3:06
// 1-07	Valeria Rossi–	Tre Parole	3:44
// 1-08	Dido–	Here With Me	4:05
// 1-09	Hooverphonic–	Mad About You	3:43
// 1-10	Jan Leyers–	Only Your Love Will Do	3:30
// 1-11	Bosson–	One In A Million	3:34
// 1-12	Roxette–	Milk And Toast And Honey	4:05
// 1-13	Kim Wilde–	Loved	3:33
// 1-14	Kosheen–	Catch	3:19
// 1-15	Dee Dee–	Forever	3:55
// 1-16	Iio–	Rapture	3:13
// 1-17	Bellefire–	Perfect Bliss	3:38
// 1-18	Riva Feat. Dannii Minogue–	Who Do You Love Now? (Stringer)	3:26
// 1-19	Ebon-e–	On My Way (Hey What 2 Do)	3:29

// Donna Amour 9 cd2
// 2-01	Krezip–	I Would Stay	3:50
// 2-02	Clouseau–	Ik Geef Me Over	4:26
// 2-03	Atomic Kitten–	Eternal Flame	3:15
// 2-04	Milk & Sugar vs John Paul*–	Love Is In The Air	3:48
// 2-05	Backstreet Boys–	Shape Of My Heart	3:47
// 2-06	Emma Bunton–	Take My Breath Away	3:38
// 2-07	Esther*, Kate*, Linda*, Maaike* & Pascale*–	Oh Baby I	3:58
// 2-08	Twarres–	She Couldn't Laugh	3:47
// 2-09	Paul Michiels–	Let Me Be Turned To Stone	3:40
// 2-10	Melanie C–	Never Be The Same Again	4:12
// 2-11	Vanda Vanda–	Love Of My Life	3:27
// 2-12	Boyzone–	Every Day I Love You	3:34
// 2-13	Belle Perez–	Honey Bee	3:16
// 2-14	Jean Jacques Smoothie–	2 People	3:26
// 2-15	Liquid (3) Feat. Silvy*–	Turn The Tide	4:03
// 2-16	Orion Too Feat. Caitlin–	You And Me	3:08
// 2-17	Lasgo–	Something	3:41
// 2-18	Rui Da Silva Feat. Cassandra*–	Touch Me	3:28


// 1-01	Kate Winslet–	What If	4:07
// 1-02	Robbie Williams–	Supreme	4:15
// 1-03	Geri Halliwell–	Calling	4:25
// 1-04	Blue (5)–	Too Close	3:45
// 1-05	Britney Spears–	Born To Make You Happy	3:35
// 1-06	Westlife–	Uptown Girl	3:06
// 1-07	Valeria Rossi–	Tre Parole	3:44
// 1-08	Dido–	Here With Me	4:05
// 1-09	Hooverphonic–	Mad About You	3:43
// 1-10	Jan Leyers–	Only Your Love Will Do	3:30
// 1-11	Bosson–	One In A Million	3:34
// 1-12	Roxette–	Milk And Toast And Honey	4:05
// 1-13	Kim Wilde–	Loved	3:33
// 1-14	Kosheen–	Catch	3:19
// 1-15	Dee Dee–	Forever	3:55
// 1-16	Iio–	Rapture	3:13
// 1-17	Bellefire–	Perfect Bliss	3:38
// 1-18	Riva Feat. Dannii Minogue–	Who Do You Love Now? (Stringer)	3:26
// 1-19	Ebon-e–	On My Way (Hey What 2 Do)	3:29";
fn main() -> Result<(), Box<dyn std::error::Error>> {
let text = "
1-19	Westlife	Swear It Again (Radio Edit)	4:04
1-19	Westlife	Forever	5:05";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(text.as_bytes());

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{}/{}", record.artist.trim_end_matches("–"), record.track);
    }
    Ok(())
}

// #[derive(Debug, serde::Deserialize)]
// struct CD {
//     num: String,
//     artist: String,
//     title: String,
// }
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let text = "
// 2-20	Vera Lynn–	We'll Meet Again
// 1-20	Edith Piaf–	Non, Je Ne Regrette Rien
// 2-1	James Blunt–	Goodbye My Lover
// 2-2	Nelly Furtado–	All Good Things (Come To An End)
// 2-3	Coldplay–	The Scientist
// 2-5	Simon & Garfunkel–	The Sound Of Silence
// 2-6	Johan Verminnen–	Laat Me Nu Toch Niet Alleen
// 2-7	Rob De Nijs–	Open Einde
// 2-8	Bram Vermeulen–	De Steen
// 2-9	Clouseau–	Afscheid Van Een Vriend
// 2-10	Randy Crawford–	One Day I'll Fly Away
// 2-11	Lionel Richie–	Hello
// 2-12	Vicki Brown–	Stay With Me Til Morning
// 2-13	Eric Carmen–	All By Myself
// 2-14	Don McLean–	Vincent
// 2-15	Diana Ross, Marvin Gaye–	You Are Everything
// 2-16	Louis Armstrong–	What A Wonderful World
// 2-17	Axelle Red–	Parce Que C'Est Toi
// 2-18	Michael Jackson–	I'll Be There
// 2-19	Will Tura–	My Way
// ";

//     let mut rdr = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .delimiter(b'\t')
//         .from_reader(text.as_bytes());

//     for result in rdr.deserialize() {
//         let record: CD = result?;
//         println!("{}/{}", record.artist.trim_end_matches("–"), record.title);
//     }
//     Ok(())
// }

// Jimmy Frey/Zo mooi zo blond
// Weird Al Yankovic/Ugly Girl
// Weird Al Yankovic/Fuck the Macarena
// Weird Al Yankovic/I think I'm a Clone now
// Weird Al Yankovic/Which Backstreet Boy is Gay
// Weird Al Yankovic/Smoke a Bowl
// Weird Al Yankovic/Pretty Fly for a Rabbi
// Weird Al Yankovic/Girls Just Wanna Have Lunch
// Weird Al Yankovic/What if God Smoked Cannabis
// Weird Al Yankovic/Let's Bomb Iraq
// Weird Al Yankovic/Just Six Words Long
// De Boswachters/Het Bananenlied
// Kabouter Plop/Ploplied
// Kabouter Plop/Kabouterdans
// Kabouter Plop/lalala
// Unknown/Sesame Street on Marijuana
// Theo Maassen/De Schimmel in de Pap
// Theo Maassen/Mirakelman
// Theo Maassen/Vanbinnen
// Theo Maassen/Eigen Mening

// Ik leef ver carnaval/Den Bremt
// Nen ajoin zen baol/Onzjier zen preferes
// Den Beijoard/De foef
// Monkeirk er een vois/Den Beinaver
// Gemem op de GSM/Kris en de foef
// Da bresjten/Gary
// Bleif van men aar af/Ondersteboven
// Montegnor/neig geambuleerd
// Gedetaljeerd/kris 2
// Oiljst Carnaval/eiry
// De poeirkes gon al oepen/Tony
// Saas an a lippen/den brenais
// go van men boan/gary en de loszemanen
// kakakakarnaval/jp de boseleir
// k'em gezopen/kris en de foef
// allajoof/kris, kristof, pascal en jak
// dag carnaval/de bremt
