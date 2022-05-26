pub const HOST: &str = "http://www.snooker.org";

pub const RESULTS: &str = "/res/index.asp?template=22&season={}";
pub const UPCOMING_MATCHES: &str = "/res/index.asp?template=24";

pub const RANKINGS: &str = "/res/index.asp?template=31&season={}";
pub const SEEDINGS: &str = "/res/index.asp?template=32&season={}";
pub const POINTS: &str = "/res/index.asp?template=33&season={}";

pub const PLAYER: &str = "/res/index.asp?player={}";
pub const EVENT: &str = "/res/index.asp?event={}";






pub struct PlayerLink{
id: u32,
name: String,
}



pub struct  EventLink{
id: u32,
title:String,
}
