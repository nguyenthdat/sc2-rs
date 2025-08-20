//! Information about units, ablities, upgrades, buffs and effects provided by API stored here.
#![allow(missing_docs)]

use crate::{
	ids::{AbilityId, BuffId, EffectId, UnitTypeId, UpgradeId},
	player::Race,
	FromProto, TryFromProto,
};
use num_traits::FromPrimitive;
use rustc_hash::FxHashMap;
use sc2_proto::{
	data::{
		ability_data::Target as AbilityData_Target, weapon::TargetType as Weapon_TargetType,
		AbilityData as ProtoAbilityData, Attribute as ProtoAttribute, BuffData as ProtoBuffData,
		EffectData as ProtoEffectData, UnitTypeData as ProtoUnitTypeData, UpgradeData as ProtoUpgradeData,
		Weapon as ProtoWeapon,
	},
	sc2api::ResponseData,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// All the data about different ids stored here.
/// Can be accessed through [`game_data`](crate::bot::Bot::game_data) field.
#[derive(Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameData {
	/// Information about abilities mapped to `AbilityId`s.
	pub abilities: FxHashMap<AbilityId, AbilityData>,
	/// Information about units mapped to `UnitTypeId`s.
	pub units: FxHashMap<UnitTypeId, UnitTypeData>,
	/// Information about upgrades mapped to `UpgradeId`s.
	pub upgrades: FxHashMap<UpgradeId, UpgradeData>,
	/// Information about buffs mapped to `BuffId`s.
	pub buffs: FxHashMap<BuffId, BuffData>,
	/// Information about effects mapped to `EffectId`s.
	pub effects: FxHashMap<EffectId, EffectData>,
}
impl FromProto<ResponseData> for GameData {
	fn from_proto(data: ResponseData) -> Self {
		Self {
			abilities: data
				.abilities
				.iter()
				.filter_map(|a| AbilityData::try_from_proto(a).map(|data| (data.id, data)))
				.collect(),
			units: data
				.units
				.iter()
				.filter_map(|u| UnitTypeData::try_from_proto(u).map(|data| (data.id, data)))
				.collect(),
			upgrades: data
				.upgrades
				.iter()
				.filter_map(|u| UpgradeData::try_from_proto(u).map(|data| (data.id, data)))
				.collect(),
			buffs: data
				.buffs
				.iter()
				.filter_map(|b| BuffData::try_from_proto(b).map(|data| (data.id, data)))
				.collect(),
			effects: data
				.effects
				.iter()
				.filter_map(|e| EffectData::try_from_proto(e).map(|data| (data.id, data)))
				.collect(),
		}
	}
}

/// Cost of an item (`UnitTypeId` or `UpgradeId`) in resources, supply and time.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cost {
	pub minerals: u32,
	pub vespene: u32,
	pub supply: f32,
	pub time: f32,
}

/// Possible target of ability, needed when giving commands to units.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AbilityTarget {
	None,
	Point,
	Unit,
	PointOrUnit,
	PointOrNone,
}
impl FromProto<AbilityData_Target> for AbilityTarget {
	fn from_proto(target: AbilityData_Target) -> Self {
		match target {
			AbilityData_Target::None => AbilityTarget::None,
			AbilityData_Target::Point => AbilityTarget::Point,
			AbilityData_Target::Unit => AbilityTarget::Unit,
			AbilityData_Target::PointOrUnit => AbilityTarget::PointOrUnit,
			AbilityData_Target::PointOrNone => AbilityTarget::PointOrNone,
		}
	}
}

/// Differents attributes of units.
#[variant_checkers]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Attribute {
	Light,
	Armored,
	Biological,
	Mechanical,
	Robotic,
	Psionic,
	Massive,
	Structure,
	Hover,
	Heroic,
	Summoned,
}
impl FromProto<ProtoAttribute> for Attribute {
	fn from_proto(attribute: ProtoAttribute) -> Self {
		match attribute {
			ProtoAttribute::Light => Attribute::Light,
			ProtoAttribute::Armored => Attribute::Armored,
			ProtoAttribute::Biological => Attribute::Biological,
			ProtoAttribute::Mechanical => Attribute::Mechanical,
			ProtoAttribute::Robotic => Attribute::Robotic,
			ProtoAttribute::Psionic => Attribute::Psionic,
			ProtoAttribute::Massive => Attribute::Massive,
			ProtoAttribute::Structure => Attribute::Structure,
			ProtoAttribute::Hover => Attribute::Hover,
			ProtoAttribute::Heroic => Attribute::Heroic,
			ProtoAttribute::Summoned => Attribute::Summoned,
		}
	}
}

/// Possible target of unit's weapon.
#[variant_checkers]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TargetType {
	Ground,
	Air,
	Any,
}
impl FromProto<Weapon_TargetType> for TargetType {
	fn from_proto(target_type: Weapon_TargetType) -> Self {
		match target_type {
			Weapon_TargetType::Ground => TargetType::Ground,
			Weapon_TargetType::Air => TargetType::Air,
			Weapon_TargetType::Any => TargetType::Any,
		}
	}
}

/// Weapon's characteristic.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Weapon {
	/// Possible targets.
	pub target: TargetType,
	/// Usual damage.
	pub damage: u32,
	/// Addidional damage vs units with specific attribute.
	pub damage_bonus: Vec<(Attribute, u32)>,
	/// Number of attacks per use.
	pub attacks: u32,
	/// Maximum range.
	pub range: f32,
	/// Cooldown (in seconds * game speed).
	pub speed: f32,
}
impl FromProto<&ProtoWeapon> for Weapon {
	fn from_proto(weapon: &ProtoWeapon) -> Self {
		Self {
			target: TargetType::from_proto(weapon.type_()),
			damage: weapon.damage() as u32,
			damage_bonus: weapon
				.damage_bonus
				.iter()
				.map(|db| (Attribute::from_proto(db.attribute()), db.bonus() as u32))
				.collect(),
			attacks: weapon.attacks(),
			range: weapon.range(),
			speed: weapon.speed(),
		}
	}
}

/// Information about specific ability.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbilityData {
	pub id: AbilityId,
	pub link_name: String,
	pub link_index: u32,
	pub button_name: Option<String>,
	pub friendly_name: Option<String>,
	pub hotkey: Option<String>,
	pub remaps_to_ability_id: Option<AbilityId>,
	/// Ability is available in current game version.
	pub available: bool,
	/// Possible target of ability, needed when giving commands to units.
	pub target: AbilityTarget,
	/// Ability can be used on minimap.
	pub allow_minimap: bool,
	/// Ability can be autocasted.
	pub allow_autocast: bool,
	/// Ability is used to construct a building.
	pub is_building: bool,
	/// Half of the building size.
	pub footprint_radius: Option<f32>,
	pub is_instant_placement: bool,
	/// Maximum range to target of the ability.
	pub cast_range: Option<f32>,
}
impl TryFromProto<&ProtoAbilityData> for AbilityData {
	fn try_from_proto(a: &ProtoAbilityData) -> Option<Self> {
		Some(Self {
			id: AbilityId::from_u32(a.ability_id())?,
			link_name: a.link_name().to_string(),
			link_index: a.link_index(),
			button_name: a.button_name.as_ref().cloned(),
			friendly_name: a.friendly_name.as_ref().cloned(),
			hotkey: a.hotkey.as_ref().cloned(),
			remaps_to_ability_id: a.remaps_to_ability_id.and_then(AbilityId::from_u32),
			available: a.available(),
			target: AbilityTarget::from_proto(a.target()),
			allow_minimap: a.allow_minimap(),
			allow_autocast: a.allow_autocast(),
			is_building: a.is_building(),
			footprint_radius: a.footprint_radius,
			is_instant_placement: a.is_instant_placement(),
			cast_range: a.cast_range,
		})
	}
}

/// Information about specific unit type.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnitTypeData {
	pub id: UnitTypeId,
	pub name: String,
	/// Unit is available in current game version.
	pub available: bool,
	/// Space usage in transports and bunkers.
	pub cargo_size: u32,
	pub mineral_cost: u32,
	pub vespene_cost: u32,
	pub food_required: f32,
	pub food_provided: f32,
	/// Ability used to produce unit or `None` if unit can't be produced.
	pub ability: Option<AbilityId>,
	/// Race of unit.
	pub race: Race,
	pub build_time: f32,
	/// Unit contains vespene (i.e. is vespene geyser).
	pub has_vespene: bool,
	/// Unit contains minerals (i.e. is mineral field).
	pub has_minerals: bool,
	pub sight_range: f32,
	pub tech_alias: Vec<UnitTypeId>,
	pub unit_alias: Option<UnitTypeId>,
	pub tech_requirement: Option<UnitTypeId>,
	pub require_attached: bool,
	pub attributes: Vec<Attribute>,
	pub movement_speed: f32,
	pub armor: i32,
	pub weapons: Vec<Weapon>,
}
impl UnitTypeData {
	pub fn cost(&self) -> Cost {
		Cost {
			minerals: self.mineral_cost,
			vespene: self.vespene_cost,
			supply: self.food_required,
			time: self.build_time,
		}
	}
}
impl TryFromProto<&ProtoUnitTypeData> for UnitTypeData {
	fn try_from_proto(u: &ProtoUnitTypeData) -> Option<Self> {
		Some(Self {
			id: UnitTypeId::from_u32(u.unit_id())?,
			name: u.name().to_string(),
			available: u.available(),
			cargo_size: u.cargo_size(),
			mineral_cost: u.mineral_cost(),
			vespene_cost: u.vespene_cost(),
			food_required: u.food_required(),
			food_provided: u.food_provided(),
			ability: u.ability_id.and_then(AbilityId::from_u32),
			race: Race::from_proto(u.race()),
			build_time: u.build_time(),
			has_vespene: u.has_vespene(),
			has_minerals: u.has_minerals(),
			sight_range: u.sight_range(),
			tech_alias: u
				.tech_alias
				.iter()
				.filter_map(|a| UnitTypeId::from_u32(*a))
				.collect(),
			unit_alias: u.unit_alias.and_then(UnitTypeId::from_u32),
			tech_requirement: u.tech_requirement.and_then(UnitTypeId::from_u32),
			require_attached: u.require_attached(),
			attributes: u
				.attributes
				.iter()
				.map(|&a| Attribute::from_proto(a.enum_value_or_default()))
				.collect(),
			movement_speed: u.movement_speed(),
			armor: u.armor() as i32,
			weapons: u.weapons.iter().map(Weapon::from_proto).collect(),
		})
	}
}

/// Information about specific upgrade.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpgradeData {
	pub id: UpgradeId,
	/// Ability used to research the upgrade.
	pub ability: AbilityId,
	pub name: String,
	pub mineral_cost: u32,
	pub vespene_cost: u32,
	pub research_time: f32,
}
impl UpgradeData {
	pub fn cost(&self) -> Cost {
		Cost {
			minerals: self.mineral_cost,
			vespene: self.vespene_cost,
			supply: 0.0,
			time: self.research_time,
		}
	}
}
impl TryFromProto<&ProtoUpgradeData> for UpgradeData {
	fn try_from_proto(u: &ProtoUpgradeData) -> Option<Self> {
		Some(Self {
			id: UpgradeId::from_u32(u.upgrade_id())?,
			ability: AbilityId::from_u32(u.ability_id())?,
			name: u.name().to_string(),
			mineral_cost: u.mineral_cost(),
			vespene_cost: u.vespene_cost(),
			research_time: u.research_time(),
		})
	}
}

/// Information about specific buff.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffData {
	pub id: BuffId,
	pub name: String,
}
impl TryFromProto<&ProtoBuffData> for BuffData {
	fn try_from_proto(b: &ProtoBuffData) -> Option<Self> {
		Some(Self {
			id: BuffId::from_u32(b.buff_id())?,
			name: b.name().to_string(),
		})
	}
}

/// Information about specific effect.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EffectData {
	pub id: EffectId,
	pub name: String,
	pub friendly_name: String,
	pub radius: f32,
	/// Targets affected by this effect.
	pub target: TargetType,
	/// `true` if effect affects allied units.
	pub friendly_fire: bool,
}
impl TryFromProto<&ProtoEffectData> for EffectData {
	fn try_from_proto(e: &ProtoEffectData) -> Option<Self> {
		EffectId::from_u32(e.effect_id()).map(|id| Self {
			id,
			name: e.name().to_string(),
			friendly_name: e.friendly_name().to_string(),
			radius: e.radius(),
			target: match id {
				EffectId::Null
				| EffectId::PsiStormPersistent
				| EffectId::ScannerSweep
				| EffectId::NukePersistent
				| EffectId::RavagerCorrosiveBileCP => TargetType::Any,
				_ => TargetType::Ground,
			},
			friendly_fire: matches!(
				id,
				EffectId::PsiStormPersistent | EffectId::NukePersistent | EffectId::RavagerCorrosiveBileCP
			),
		})
	}
}
