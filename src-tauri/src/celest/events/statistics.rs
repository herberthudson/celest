use super::elite_dangerous::TimeStamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Statistics {
    timestamp: TimeStamp,
    event: String,
    #[serde(rename = "Bank_Account")]
    bank_account: BankAccount,
    #[serde(rename = "Combat")]
    combat: Combat,
    #[serde(rename = "Crime")]
    crime: Crime,
    #[serde(rename = "Smuggling")]
    smuggling: Smuggling,
    #[serde(rename = "Trading")]
    tradding: Trading,
    #[serde(rename = "Mining")]
    mining: Mining,
    #[serde(rename = "Exploration")]
    exploration: Exploration,
    #[serde(rename = "Passenger")]
    passenders: Passenger,
    #[serde(rename = "Search_And_Rescue")]
    search_and_rescue: SearchAndRescue,
    #[serde(rename = "Crafting")]
    crafting: Crafting,
    #[serde(rename = "Crew")]
    crew: Crew,
    #[serde(rename = "Multicrew")]
    multicrew: Multicrew,
    #[serde(rename = "Material_Trader_Stats")]
    material_trader_stats: MaterialTraderStats,
    #[serde(rename = "Exobiology")]
    exobiology: Exobiology,
}

// TODO: valid fields with size of u64 is have the right type, or could be u16 or u8
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BankAccount {
    #[serde(rename = "Current_Wealth")]
    current_wealth: u64,
    #[serde(rename = "Spent_On_Ships")]
    spent_on_ships: u64,
    #[serde(rename = "Spent_On_Outfitting")]
    spent_on_outfitting: u64,
    #[serde(rename = "Spent_On_Repairs")]
    spent_on_repairs: u64,
    #[serde(rename = "Spent_On_Fuel")]
    spent_on_fuel: u64,
    #[serde(rename = "Spent_On_Ammo_Consumables")]
    spent_on_ammo_consumables: u64,
    #[serde(rename = "Insurance_Claims")]
    insurance_claims: u64,
    #[serde(rename = "Spent_On_Insurance")]
    spent_on_insurance: u64,
    #[serde(rename = "Owned_Ship_Count")]
    owned_ship_count: u16,
    #[serde(rename = "Spent_On_Suits")]
    spent_on_suits: u64,
    #[serde(rename = "Spent_On_Weapons")]
    spent_on_weapons: u64,
    #[serde(rename = "Spent_On_Suit_Consumables")]
    spent_on_suit_consumables: u64,
    #[serde(rename = "Suits_Owned")]
    suits_owned: u16,
    #[serde(rename = "Weapons_Owned")]
    weapons_owned: u16,
    #[serde(rename = "Spent_On_Premium_Stock")]
    spent_on_premium_stock: u64,
    #[serde(rename = "Premium_Stock_Bought")]
    premium_stock_bought: u16,
}

// TODO: check size of type integer is the correct size
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Combat {
    #[serde(rename = "Bounties_Claimed")]
    bounties_claimed: u16,
    #[serde(rename = "Bounty_Hunting_Profit")]
    bounty_hunting_profit: u64,
    #[serde(rename = "Combat_Bonds")]
    combat_bonds: u64,
    #[serde(rename = "Combat_Bond_Profits")]
    combat_bond_profits: u64,
    #[serde(rename = "Assassinations")]
    assassinations: u64,
    #[serde(rename = "Assassination_Profits")]
    assassination_profits: u64,
    #[serde(rename = "Highest_Single_Reward")]
    highest_single_reward: u64,
    #[serde(rename = "Skimmers_Killed")]
    skimmers_killed: u64,
    #[serde(rename = "OnFoot_Combat_Bonds")]
    onfoot_combat_bonds: u64,
    #[serde(rename = "OnFoot_Combat_Bonds_Profits")]
    onfoot_combat_bonds_profits: u64,
    #[serde(rename = "OnFoot_Vehicles_Destroyed")]
    onfoot_vehicles_destroyed: u64,
    #[serde(rename = "OnFoot_Ships_Destroyed")]
    onfoot_ships_destroyed: u64,
    #[serde(rename = "Dropships_Taken")]
    dropships_taken: u64,
    #[serde(rename = "Dropships_Booked")]
    dropships_booked: u64,
    #[serde(rename = "Dropships_Cancelled")]
    dropships_cancelled: u64,
    #[serde(rename = "ConflictZone_High")]
    conflict_zone_high: u64,
    #[serde(rename = "ConflictZone_Medium")]
    conflict_zone_medium: u64,
    #[serde(rename = "ConflictZone_Low")]
    conflict_zone_low: u64,
    #[serde(rename = "ConflictZone_Total")]
    conflict_zone_total: u64,
    #[serde(rename = "ConflictZone_High_Wins")]
    conflict_zone_high_wins: u64,
    #[serde(rename = "ConflictZone_Medium_Wins")]
    conflict_zone_medium_wins: u64,
    #[serde(rename = "ConflictZone_Low_Wins")]
    conflict_zone_low_wins: u64,
    #[serde(rename = "ConflictZone_Total_Wins")]
    conflict_zone_total_wins: u64,
    #[serde(rename = "Settlement_Defended")]
    settlement_defended: u64,
    #[serde(rename = "Settlement_Conquered")]
    settlement_conquered: u64,
    #[serde(rename = "OnFoot_Skimmers_Killed")]
    onfoot_skimmers_killed: u64,
    #[serde(rename = "OnFoot_Scavs_Killed")]
    onfoot_scavs_killed: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Crime {
    #[serde(rename = "Notoriety")]
    notoriety: u8,
    #[serde(rename = "Fines")]
    fines: u64,
    #[serde(rename = "Total_Fines")]
    total_fines: u64,
    #[serde(rename = "Bounties_Received")]
    bounties_received: u64,
    #[serde(rename = "Total_Bounties")]
    total_bounties: u64,
    #[serde(rename = "Highest_Bounty")]
    highest_bounty: u64,
    #[serde(rename = "Malware_Uploaded")]
    malware_uploaded: u64,
    #[serde(rename = "Settlements_State_Shutdown")]
    settlements_state_shutdown: u64,
    #[serde(rename = "Production_Sabotage")]
    production_sabotage: u64,
    #[serde(rename = "Production_Theft")]
    production_theft: u64,
    #[serde(rename = "Total_Murders")]
    total_murders: u64,
    #[serde(rename = "Citizens_Murdered")]
    citizens_murdered: u64,
    #[serde(rename = "Omnipol_Murdered")]
    omnipol_murdered: u64,
    #[serde(rename = "Guards_Murdered")]
    guards_murdered: u64,
    #[serde(rename = "Data_Stolen")]
    data_stolen: u64,
    #[serde(rename = "Goods_Stolen")]
    goods_stolen: u64,
    #[serde(rename = "Sample_Stolen")]
    sample_stolen: u64,
    #[serde(rename = "Total_Stolen")]
    total_stolen: u64,
    #[serde(rename = "Turrets_Destroyed")]
    turrets_destroyed: u64,
    #[serde(rename = "Turrets_Overloaded")]
    turrets_overloaded: u64,
    #[serde(rename = "Turrets_Total")]
    turrets_total: u64,
    #[serde(rename = "Value_Stolen_StateChange")]
    value_stolen_statechange: u64,
    #[serde(rename = "Profiles_Cloned")]
    profiles_cloned: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Smuggling {
    #[serde(rename = "Black_Markets_Traded_With")]
    black_markets_traded_with: u64,
    #[serde(rename = "Black_Markets_Profits")]
    black_markets_profits: u64,
    #[serde(rename = "Resources_Smuggled")]
    resources_smuggled: u64,
    #[serde(rename = "Average_Profit")]
    average_profit: u64,
    #[serde(rename = "Highest_Single_Transaction")]
    highest_single_transaction: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trading {
    #[serde(rename = "Markets_Traded_With")]
    markets_traded_with: u64,
    #[serde(rename = "Market_Profits")]
    market_profits: u64,
    #[serde(rename = "Resources_Traded")]
    resources_traded: u64,
    #[serde(rename = "Average_Profit")]
    average_profit: u64,
    #[serde(rename = "Highest_Single_Transaction")]
    highest_single_transaction: u64,
    #[serde(rename = "Data_Sold")]
    data_sold: u64,
    #[serde(rename = "Goods_Sold")]
    goods_sold: u64,
    #[serde(rename = "Assets_Sold")]
    assets_sold: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mining {
    #[serde(rename = "Mining_Profits")]
    mining_profits: u64,
    #[serde(rename = "Quantity_Mined")]
    quantity_mined: u64,
    #[serde(rename = "Materials_Collected")]
    materials_collected: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Exploration {
    #[serde(rename = "Systems_Visited")]
    systems_visited: u64,
    #[serde(rename = "Exploration_Profits")]
    exploration_profits: u64,
    #[serde(rename = "Planets_Scanned_To_Level_2")]
    planets_scanned_to_level_2: u64,
    #[serde(rename = "Planets_Scanned_To_Level_3")]
    planets_scanned_to_level_3: u64,
    #[serde(rename = "Efficient_Scans")]
    efficient_scans: u64,
    #[serde(rename = "Highest_Payout")]
    highest_payout: u64,
    #[serde(rename = "Total_Hyperspace_Distance")]
    total_hyperspace_distance: u64,
    #[serde(rename = "Total_Hyperspace_Jumps")]
    total_hyperspace_jumps: u64,
    #[serde(rename = "Greatest_Distance_From_Start")]
    greatest_distance_from_start: f32,
    #[serde(rename = "Time_Played")]
    time_played: u64,
    #[serde(rename = "OnFoot_Distance_Travelled")]
    onfoot_distance_travelled: u64,
    #[serde(rename = "Shuttle_Journeys")]
    shuttle_journeys: u64,
    #[serde(rename = "Shuttle_Distance_Travelled")]
    shuttle_distance_travelled: u64,
    #[serde(rename = "Spent_On_Shuttles")]
    spent_on_shuttles: u64,
    #[serde(rename = "First_Footfalls")]
    first_footfalls: u64,
    #[serde(rename = "Planet_Footfalls")]
    planet_footfalls: u64,
    #[serde(rename = "Settlements_Visited")]
    settlements_visited: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Passenger {
    #[serde(rename = "Passengers_Missions_Accepted")]
    passengers_missions_accepted: u64,
    #[serde(rename = "Passengers_Missions_Disgruntled")]
    passengers_missions_disgruntled: u64,
    #[serde(rename = "Passengers_Missions_Bulk")]
    passengers_missions_bulk: u64,
    #[serde(rename = "Passengers_Missions_VIP")]
    passengers_missions_vip: u64,
    #[serde(rename = "Passengers_Missions_Delivered")]
    passengers_missions_delivered: u64,
    #[serde(rename = "Passengers_Missions_Ejected")]
    passengers_missions_ejected: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "Search_And_Rescue")]
pub struct SearchAndRescue {
    #[serde(rename = "SearchRescue_Traded")]
    searchrescue_traded: u64,
    #[serde(rename = "SearchRescue_Profit")]
    searchrescue_profit: u64,
    #[serde(rename = "SearchRescue_Count")]
    searchrescue_count: u64,
    #[serde(rename = "Salvage_Legal_POI")]
    salvage_legal_poi: u64,
    #[serde(rename = "Salvage_Legal_Settlements")]
    salvage_legal_settlements: u64,
    #[serde(rename = "Salvage_Illegal_POI")]
    salvage_illegal_poi: u64,
    #[serde(rename = "Salvage_Illegal_Settlements")]
    salvage_illegal_settlements: u64,
    #[serde(rename = "Maglocks_Opened")]
    maglocks_opened: u64,
    #[serde(rename = "Panels_Opened")]
    panels_opened: u64,
    #[serde(rename = "Settlements_State_FireOut")]
    settlements_state_fireout: u64,
    #[serde(rename = "Settlements_State_Reboot")]
    settlements_state_reboot: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Crafting {
    #[serde(rename = "Count_Of_Crafted_Items")]
    count_of_crafted_items: u64,
    #[serde(rename = "Count_Of_Used_Engineers")]
    count_of_used_engineers: u64,
    #[serde(rename = "Recipes_Generated")]
    recipes_generated: u64,
    #[serde(rename = "Recipes_Generated_Rank_1")]
    recipes_generated_rank_1: u64,
    #[serde(rename = "Recipes_Generated_Rank_2")]
    recipes_generated_rank_2: u64,
    #[serde(rename = "Recipes_Generated_Rank_3")]
    recipes_generated_rank_3: u64,
    #[serde(rename = "Recipes_Generated_Rank_4")]
    recipes_generated_rank_4: u64,
    #[serde(rename = "Recipes_Generated_Rank_5")]
    recipes_generated_rank_5: u64,
    #[serde(rename = "Suit_Mods_Applied")]
    suit_mods_applied: u64,
    #[serde(rename = "Weapon_Mods_Applied")]
    weapon_mods_applied: u64,
    #[serde(rename = "Suits_Upgraded")]
    suits_upgraded: u64,
    #[serde(rename = "Weapons_Upgraded")]
    weapons_upgraded: u64,
    #[serde(rename = "Suits_Upgraded_Full")]
    suits_upgraded_full: u64,
    #[serde(rename = "Weapons_Upgraded_Full")]
    weapons_upgraded_full: u64,
    #[serde(rename = "Suit_Mods_Applied_Full")]
    suit_mods_applied_full: u64,
    #[serde(rename = "Weapon_Mods_Applied_Full")]
    weapon_mods_applied_full: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Crew {
    #[serde(rename = "NpcCrew_TotalWages")]
    npccrew_totalwages: u64,
    #[serde(rename = "NpcCrew_Hired")]
    npccrew_hired: u16,
    #[serde(rename = "NpcCrew_Fired")]
    npccrew_fired: u16,
    #[serde(rename = "NpcCrew_Died")]
    npccrew_died: u8,
}

// TODO: check size of type integer is the correct size
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Multicrew {
    #[serde(rename = "Multicrew_Time_Total")]
    multicrew_time_total: u64,
    #[serde(rename = "Multicrew_Gunner_Time_Total")]
    multicrew_gunner_time_total: u64,
    #[serde(rename = "Multicrew_Fighter_Time_Total")]
    multicrew_fighter_time_total: u64,
    #[serde(rename = "Multicrew_Credits_Total")]
    multicrew_credits_total: u64,
    #[serde(rename = "Multicrew_Fines_Total")]
    multicrew_fines_total: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "Material_Trader_Stats")]
pub struct MaterialTraderStats {
    #[serde(rename = "Trades_Completed")]
    trades_completed: u64,
    #[serde(rename = "Materials_Traded")]
    materials_traded: u64,
    #[serde(rename = "Encoded_Materials_Traded")]
    encoded_materials_traded: u64,
    #[serde(rename = "Raw_Materials_Traded")]
    raw_materials_traded: u64,
    #[serde(rename = "Grade_1_Materials_Traded")]
    grade_1_materials_traded: u64,
    #[serde(rename = "Grade_2_Materials_Traded")]
    grade_2_materials_traded: u64,
    #[serde(rename = "Grade_3_Materials_Traded")]
    grade_3_materials_traded: u64,
    #[serde(rename = "Grade_4_Materials_Traded")]
    grade_4_materials_traded: u64,
    #[serde(rename = "Grade_5_Materials_Traded")]
    grade_5_materials_traded: u64,
    #[serde(rename = "Assets_Traded_In")]
    assets_traded_in: u64,
    #[serde(rename = "Assets_Traded_Out")]
    assets_traded_out: u64,
}

// TODO: check size of type integer is the correct size
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Exobiology {
    #[serde(rename = "Organic_Genus_Encountered")]
    organic_genus_encountered: u64,
    #[serde(rename = "Organic_Species_Encountered")]
    organic_species_encountered: u64,
    #[serde(rename = "Organic_Variant_Encountered")]
    organic_variant_encountered: u64,
    #[serde(rename = "Organic_Data_Profits")]
    organic_data_profits: u64,
    #[serde(rename = "Organic_Data")]
    organic_data: u64,
    #[serde(rename = "First_Logged_Profits")]
    first_logged_profits: u64,
    #[serde(rename = "First_Logged")]
    first_logged: u64,
    #[serde(rename = "Organic_Systems")]
    organic_systems: u64,
    #[serde(rename = "Organic_Planets")]
    organic_planets: u64,
    #[serde(rename = "Organic_Genus")]
    organic_genus: u64,
    #[serde(rename = "Organic_Species")]
    organic_species: u64,
}
