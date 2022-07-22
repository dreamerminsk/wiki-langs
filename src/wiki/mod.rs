pub mod entities;



pub async fn get_wiki(inter_eiki: InterWiki) -> Result<Wiki, Box<dyn Error>> {
    let page = web::get(format!("{}{}{}", urls::HOST, urls::PLAYER, snooker_id)).await?;

  

    Ok(Wiki {
    
    })
}
