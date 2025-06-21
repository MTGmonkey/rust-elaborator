use reqwest::header::USER_AGENT;
use rust_elaborator::*;
use std::io;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Welcome to rust_elaborator!");
    write_csv().await;
    Ok(())
}

async fn get_id_from_name(client: &reqwest::Client, name: String) -> Option<i32> {
    let request_url = format!("https://boardgamegeek.com/xmlapi/search?search={name}");
    println!("searching for name {}", name);
    let text = match make_request(client, &request_url).await {
        Some(text) => text,
        None => "".to_string(),
    };
    let xml: Keyword_search_results = match serde_xml_rs::from_str(&text) {
        Ok(xml) => xml,
        Err(_) => return None,
    };
    let mut games: Vec<Boardgame_overview> = xml.boardgames.clone();
    let (best_match, score) = find_best_boardgame(name, games);
    println!("found game {}", best_match.name);
    println!("score is {}", score);
    return Some(best_match.objectid);
}

async fn get_name_from_name(client: &reqwest::Client, name: String) -> Option<String> {
    let request_url = format!("https://boardgamegeek.com/xmlapi/search?search={name}");
    let text = match make_request(client, &request_url).await {
        Some(text) => text,
        None => "".to_string(),
    };
    let xml: Keyword_search_results = match serde_xml_rs::from_str(&text) {
        Ok(xml) => xml,
        Err(_) => return None,
    };
    let mut games: Vec<Boardgame_overview> = xml.boardgames.clone();
    let (best_match, score) = find_best_boardgame(name, games);
    return Some(best_match.name);
}

async fn get_boardgame_from_id(client: &reqwest::Client, id: i32) -> Option<Boardgame> {
    let request_url = format!("https://boardgamegeek.com/xmlapi/boardgame/{id}/");
    println!("searching for id {}", id);
    let text = match make_request(client, &request_url).await {
        Some(text) => text,
        None => "".to_string(),
    };
    let xml: Id_search_results = match serde_xml_rs::from_str(&text) {
        Ok(xml) => xml,
        Err(_) => return None,
    };
    return Some(xml.boardgame);
}

async fn get_boardgame_from_name(client: &reqwest::Client, name: String) -> Option<Boardgame> {
    let id = get_id_from_name(client, name).await;
    match id {
        Some(id) => get_boardgame_from_id(client, id).await,
        None => None,
    }
}

async fn make_request(client: &reqwest::Client, request_url: &str) -> Option<String> {
    let response = match client
        .get(request_url)
        .header(USER_AGENT, "andromeda-boardgame-info-finder")
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return None,
    };
    match response.text().await {
        Ok(text) => Some(text),
        Err(_) => None,
    }
}

fn find_best_boardgame(
    name: String,
    mut games: Vec<Boardgame_overview>,
) -> (Boardgame_overview, i32) {
    use fuzzy_matcher::FuzzyMatcher;
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
    //    println!("BEFOREHAND");
    //    println!("{:#?}", games);
    games.sort_by(|b, a| {
        matcher
            .fuzzy_match(
                &a.name.chars().collect::<Vec<char>>()
                    [..std::cmp::min(name.len(), a.name.chars().collect::<Vec<char>>().len())]
                    .iter()
                    .collect::<String>()
                    .to_lowercase(),
                &name.to_lowercase(),
            )
            .cmp(
                &matcher.fuzzy_match(
                    &b.name.chars().collect::<Vec<char>>()
                        [..std::cmp::min(name.len(), b.name.chars().collect::<Vec<char>>().len())]
                        .iter()
                        .collect::<String>()
                        .to_lowercase(),
                    &name.to_lowercase(),
                ),
            )
    });
    //    println!("AFTERHAND");
    //    println!("{:#?}", games);
    (
        games[0].clone(),
        match matcher.fuzzy_match(
            &games[0].name.chars().collect::<Vec<char>>()[..std::cmp::min(
                name.len(),
                games[0].name.chars().collect::<Vec<char>>().len(),
            )]
                .iter()
                .collect::<String>()
                .to_lowercase(),
            &name.to_lowercase(),
        ) {
            Some(val) => val,
            None => 0,
        },
    )
}

fn find_best_match(name: String, mut names: Vec<&String>) -> (String, i32) {
    use fuzzy_matcher::FuzzyMatcher;
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
    names.sort_unstable_by(|a, b| {
        matcher
            .fuzzy_match(a, &name)
            .cmp(&matcher.fuzzy_match(b, &name))
    });
    (
        names[0].to_string(),
        matcher.fuzzy_match(&names[0], &name).unwrap(),
    )
}

fn read_csv() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.records() {
        let record = result.unwrap();
        let game = &record[0];
        println!("Game: {}", game);
    }
    Ok(())
}

async fn write_csv() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut out = File::create("out.csv").unwrap();
    let mut writer = csv::Writer::from_writer(out);
    let mut reader = csv::Reader::from_reader(io::stdin());
    writer
        .write_record(&[
            "title",
            "foundtitle",
            "minplayers",
            "maxplayers",
            "playingtime",
            "minplaytime",
            "maxplaytime",
            "age",
        ])
        .unwrap();
    for result in reader.records() {
        let record = result.unwrap();
        let game = &record[0];
        let client = reqwest::Client::new();
        let boardgame = get_boardgame_from_name(&client, game.to_string()).await;
        match boardgame {
            Some(boardgame) => {
                let minplayers = if (boardgame.minplayers != 0) {
                    boardgame.minplayers.to_string()
                } else {
                    "".to_string()
                };
                let maxplayers = if (boardgame.maxplayers != 0) {
                    boardgame.maxplayers.to_string()
                } else {
                    "".to_string()
                };
                let playingtime = if (boardgame.playingtime != 0) {
                    boardgame.playingtime.to_string()
                } else {
                    "".to_string()
                };
                let minplaytime = if (boardgame.minplaytime != 0) {
                    boardgame.minplaytime.to_string()
                } else {
                    "".to_string()
                };
                let maxplaytime = if (boardgame.maxplaytime != 0) {
                    boardgame.maxplaytime.to_string()
                } else {
                    "".to_string()
                };
                let age = if (boardgame.age != 0) {
                    boardgame.age.to_string()
                } else {
                    "".to_string()
                };
                writer
                    .write_record(&[
                        game,
                        get_name_from_name(&client, game.to_string())
                            .await
                            .unwrap()
                            .as_str(),
                        minplayers.as_str(),
                        maxplayers.as_str(),
                        playingtime.as_str(),
                        minplaytime.as_str(),
                        maxplaytime.as_str(),
                        age.as_str(),
                    ])
                    .unwrap()
            }
            None => writer
                .write_record(&[game, "NOT_FOUND", "", "", "", "", "", ""])
                .unwrap(),
        };
        writer.flush();
    }
    Ok(())
}
