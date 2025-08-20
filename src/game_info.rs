//! Constant information about map, populated on first step.

use crate::{
	FromProto,
	bot::Rs,
	geometry::{Point2, Rect, Size},
	pixel_map::{ByteMap, PixelMap},
	player::{AIBuild, Difficulty, PlayerType, Race},
};
use rustc_hash::FxHashMap;
use sc2_proto::sc2api::ResponseGameInfo;
use std::path::Path;

/// Structure where all map information stored.
#[derive(Default, Clone)]
pub struct GameInfo {
	/// Map name bot playing on, which depends on sc2 localization language.
	pub map_name: String,
	/// Map name bot playing on, which depends on file name.
	pub map_name_path: String,
	/// Mods used on that map.
	pub mod_names: Vec<String>,
	/// Path to the map on current computer.
	pub local_map_path: String,
	/// Players on this map, mapped by their ids.
	pub players: FxHashMap<u32, PlayerInfo>,
	/// Full size of the map.
	pub map_size: Size,
	/// Grid with information about pathable tiles on that map.
	pub pathing_grid: PixelMap,
	/// Grid with information about terrain height on that map.
	pub terrain_height: Rs<ByteMap>,
	/// Grid with information about buildable tiles on that map.
	pub placement_grid: PixelMap,
	/// Usually maps have some unplayable area around it, where units can't exist.
	/// This rectangle is only playble area on that map.
	pub playable_area: Rect,
	/// All starting locations of opponents.
	pub start_locations: Vec<Point2>,
	/// Center of the map.
	pub map_center: Point2,
}
impl FromProto<ResponseGameInfo> for GameInfo {
	fn from_proto(game_info: ResponseGameInfo) -> Self {
		let start_raw = game_info.start_raw.as_ref().unwrap_or_default();
		let map_size = start_raw.map_size.as_ref().unwrap_or_default();
		let area = start_raw.playable_area.as_ref().unwrap_or_default();
		let area_p0 = area.p0.as_ref().unwrap_or_default();
		let area_p1 = area.p1.as_ref().unwrap_or_default();
		let area_p0_x = area_p0.x();
		let area_p0_y = area_p0.y();
		let area_p1_x = area_p1.x();
		let area_p1_y = area_p1.y();
		let local_map_path = game_info.local_map_path().to_string();
		Self {
			map_name: game_info.map_name().to_string(),
			mod_names: game_info.mod_names.to_vec(),
			map_name_path: Path::new(&local_map_path)
				.file_stem()
				.unwrap_or_default()
				.to_str()
				.unwrap()
				.to_string(),
			local_map_path,
			players: game_info
				.player_info
				.iter()
				.map(|i| {
					let id = i.player_id();
					(
						id,
						PlayerInfo {
							id,
							player_type: PlayerType::from_proto(i.type_()),
							race_requested: Race::from_proto(i.race_requested()),
							race_actual: i.race_actual.map(|r| Race::from_proto(r.enum_value_or_default())),
							difficulty: i
								.difficulty
								.map(|d| Difficulty::from_proto(d.enum_value_or_default())),
							ai_build: i.ai_build.map(|b| AIBuild::from_proto(b.enum_value_or_default())),
							player_name: i.player_name.as_ref().cloned(),
						},
					)
				})
				.collect(),
			map_size: Size::new(map_size.x() as usize, map_size.y() as usize),
			pathing_grid: PixelMap::from_proto(start_raw.pathing_grid.as_ref().unwrap_or_default()),
			terrain_height: Rs::new(ByteMap::from_proto(
				start_raw.terrain_height.as_ref().unwrap_or_default(),
			)),
			placement_grid: PixelMap::from_proto(start_raw.placement_grid.as_ref().unwrap_or_default()),
			playable_area: Rect::new(
				area_p0_x as usize,
				area_p0_y as usize,
				area_p1_x as usize,
				area_p1_y as usize,
			),
			start_locations: start_raw.start_locations.iter().map(Point2::from_proto).collect(),
			map_center: Point2::new(
				(area_p0_x + (area_p1_x - area_p0_x) / 2) as f32,
				(area_p0_y + (area_p1_y - area_p0_y) / 2) as f32,
			),
		}
	}
}

/// Information about player.
#[derive(Clone)]
pub struct PlayerInfo {
	/// Player id.
	pub id: u32,
	/// Player type, can be `Participant`, `Computer` or `Observer`.
	pub player_type: PlayerType,
	/// Requested race, can be random.
	pub race_requested: Race,
	/// Actual race, it's never random, `None` for opponents.
	pub race_actual: Option<Race>,
	/// Difficulty, populated only for computer opponents.
	pub difficulty: Option<Difficulty>,
	/// AI Build, populated only for computer opponents.
	pub ai_build: Option<AIBuild>,
	/// In-game name of player.
	pub player_name: Option<String>,
}
