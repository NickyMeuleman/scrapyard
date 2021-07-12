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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "2-1	–David Et Jonathan	Bella Vita	4:29
2-2	–Cookie Dingler	Femme Libérée	3:40
2-3	–Didier Barbelivien	Elle	3:10
2-4	–C.Jerôme	Et Tu Danses Avec Lui	3:31
2-5	–Claude Barzotti	Elle Me Tue	3:51
2-6	–Michelle Torr	I Remember You	4:03
2-7	–Felix Gray	La Gitane	4:20
2-8	–Jean-Pierre François	Je Te Survivrai	4:19
2-9	–David Et Jonathan	Est-Ce Que Tu Viens Pour Les Vacances	4:24
2-10	–Didier Barbelivien	Elsa	3:14
2-11	–Mike Brant	Love Is A Feeling	4:13
2-12	–Claude Barzotti	Le Rital	3:27
2-13	–Felix Gray	Te Revoir A Madrid	3:42
2-14	–Eric Charden	Les Monde Est Gris, Le Monde Est Bleu	3:35
";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(text.as_bytes());

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{}/{}", record.artist.trim_start_matches("–"), record.track);
    }
    Ok(())
}
