// Player struct to retrieve player data from dataset and send to frontend
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Player {
    pub nick: String,
    pub country: String,
    pub stats_link: String,
    pub teams: String,
    pub maps_played: u64,
    pub rounds_played: u64,
    pub kd_difference: u64,
    pub kd_ratio: f64,
    pub rating: f64,
    pub total_kills: u64,
    pub headshot_percentage: f64,
    pub total_deaths: u64,
    pub grenade_damage_per_round: f64,
    pub kills_per_round: f64,
    pub assists_per_round: f64,
    pub deaths_per_round: f64,
    pub teammate_saved_per_round: f64,
    pub saved_by_teammate_per_round: f64,
    pub kast: f64,
    pub impact: f64,
}
