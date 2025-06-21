use reqwest::header::USER_AGENT;
use rust_elaborator::*;
use std::io;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Welcome to rust_elaborator!");
    println!(
        "Someday, this program will allow a csv input and output a csv with more information."
    );
    println!("For now, however, it takes text input and outputs text as well.");
    println!("Enter the name of a boardgame, hit enter, and see information. Then repeat!");
    println!("Have fun with rust_elaborator!");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let client = reqwest::Client::new();
        match get_boardgame_from_name(&client, line.unwrap()).await {
            Some(boardgame) => println!("{:#?}", boardgame),
            None => println!("Game not found"),
        }
    }
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
                &a.name.chars().collect::<Vec<char>>()[..name.len()]
                    .iter()
                    .collect::<String>()
                    .to_lowercase(),
                &name.to_lowercase(),
            )
            .cmp(
                &matcher.fuzzy_match(
                    &b.name.chars().collect::<Vec<char>>()[..name.len()]
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
            &games[0].name.chars().collect::<Vec<char>>()[..name.len()]
                .iter()
                .collect::<String>()
                .to_lowercase(),
            &name.to_lowercase(),
        ) {
            Some(val) => val,
            Nothing => 0,
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
