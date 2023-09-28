//! functionality relating to Minecraft biomes.

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Deserializer};

// Values from https://minecraft.wiki/w/Java_Edition_data_value#Biomes
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i32)] // i32 as in corresponding NBT.
pub enum Biome {
    Ocean = 0,
    Forest = 4,
    River = 7,
    FrozenOcean = 10,
    FrozenRiver = 11,
    Beach = 16,
    DeepOcean = 24,
    StoneShore = 25,
    SnowyBeach = 26,
    WarmOcean = 44,
    LukewarmOcean = 45,
    ColdOcean = 46,
    DeepWarmOcean = 47,
    DeepLukewarmOcean = 48,
    DeepColdOcean = 49,
    DeepFrozenOcean = 50,
    WoodedHills = 18,
    FlowerForest = 132,
    BirchForest = 27,
    BirchForestHills = 28,
    TallBirchForest = 155,
    TallBirchHills = 156,
    DarkForest = 29,
    DarkForestHills = 157,
    Jungle = 21,
    JungleHills = 22,
    ModifiedJungle = 149,
    JungleEdge = 23,
    ModifiedJungleEdge = 151,
    BambooJungle = 168,
    BambooJungleHills = 169,
    Taiga = 5,
    TaigaHills = 19,
    TaigaMountains = 133,
    SnowyTaiga = 30,
    SnowyTaigaHills = 31,
    SnowyTaigaMountains = 158,
    GiantTreeTaiga = 32,
    GiantTreeTaigaHills = 33,
    GiantSpruceTaiga = 160,
    GiantSpruceTaigaHills = 161,
    MushroomFields = 14,
    MushroomFieldShore = 15,
    Swamp = 6,
    SwampHills = 134,
    Savanna = 35,
    SavannaPlateau = 36,
    ShatteredSavanna = 163,
    ShatteredSavannaPlateau = 164,
    Plains = 1,
    SunflowerPlains = 129,
    Desert = 2,
    DesertHills = 17,
    DesertLakes = 130,
    SnowyTundra = 12,
    SnowyMountains = 13,
    IceSpikes = 140,
    Mountains = 3,
    WoodedMountains = 34,
    GravellyMountains = 131,
    ModifiedGravellyMountains = 162,
    MountainEdge = 20,
    Badlands = 37,
    BadlandsPlateau = 39,
    ModifiedBadlandsPlateau = 167,
    WoodedBadlandsPlateau = 38,
    ModifiedWoodedBadlandsPlateau = 166,
    ErodedBadlands = 165,
    Nether = 8,
    TheEnd = 9,
    SmallEndIslands = 40,
    EndMidlands = 41,
    EndHighlands = 42,
    EndBarrens = 43,
    SoulSandValley = 170,
    CrimsonForest = 171,
    WarpedForest = 172,
    TheVoid = 127,
    BasaltDeltas = 173,

    // Biomes after 1.18, where world data moved away from raw numbers. We just
    // let these take whatever value in the Rust enum.
    DripstoneCaves,
    FrozenPeaks,
    Grove,
    JaggedPeaks,
    LushCaves,
    Meadow,
    NetherWastes,
    OldGrowthBirchForest,
    OldGrowthPineTaiga,
    OldGrowthSpruceTaiga,
    SnowyPlains,
    SnowySlopes,
    SparseJungle,
    StonyPeaks,
    StonyShore,
    WindsweptForest,
    WindsweptGravellyHills,
    WindsweptHills,
    WindsweptSavanna,
    WoodedBadlands,
    MangroveSwamp,
    DeepDark,
    Unknown,
}

impl<'de> Deserialize<'de> for Biome {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        use Biome::*;

        match s.strip_prefix("minecraft:") {
            Some(name) => Ok(match name {
                "badlands" => Badlands,
                "bamboo_jungle" => BambooJungle,
                "basalt_deltas" => BasaltDeltas,
                "beach" => Beach,
                "birch_forest" => BirchForest,
                "cold_ocean" => ColdOcean,
                "crimson_forest" => CrimsonForest,
                "dark_forest" => DarkForest,
                "deep_cold_ocean" => DeepColdOcean,
                "deep_frozen_ocean" => DeepFrozenOcean,
                "deep_lukewarm_ocean" => DeepLukewarmOcean,
                "deep_ocean" => DeepOcean,
                "desert" => Desert,
                "dripstone_caves" => DripstoneCaves,
                "end_barrens" => EndBarrens,
                "end_highlands" => EndHighlands,
                "end_midlands" => EndMidlands,
                "eroded_badlands" => ErodedBadlands,
                "flower_forest" => FlowerForest,
                "forest" => Forest,
                "frozen_ocean" => FrozenOcean,
                "frozen_peaks" => FrozenPeaks,
                "frozen_river" => FrozenRiver,
                "grove" => Grove,
                "ice_spikes" => IceSpikes,
                "jagged_peaks" => JaggedPeaks,
                "jungle" => Jungle,
                "lukewarm_ocean" => LukewarmOcean,
                "lush_caves" => LushCaves,
                "meadow" => Meadow,
                "mushroom_fields" => MushroomFields,
                "nether_wastes" => NetherWastes,
                "ocean" => Ocean,
                "old_growth_birch_forest" => OldGrowthBirchForest,
                "old_growth_pine_taiga" => OldGrowthPineTaiga,
                "old_growth_spruce_taiga" => OldGrowthSpruceTaiga,
                "plains" => Plains,
                "river" => River,
                "savanna" => Savanna,
                "savanna_plateau" => SavannaPlateau,
                "small_end_islands" => SmallEndIslands,
                "snowy_beach" => SnowyBeach,
                "snowy_plains" => SnowyPlains,
                "snowy_slopes" => SnowySlopes,
                "snowy_taiga" => SnowyTaiga,
                "soul_sand_valley" => SoulSandValley,
                "sparse_jungle" => SparseJungle,
                "stony_peaks" => StonyPeaks,
                "stony_shore" => StonyShore,
                "sunflower_plains" => SunflowerPlains,
                "swamp" => Swamp,
                "taiga" => Taiga,
                "the_end" => TheEnd,
                "the_void" => TheVoid,
                "warm_ocean" => WarmOcean,
                "warped_forest" => WarpedForest,
                "windswept_forest" => WindsweptForest,
                "windswept_gravelly_hills" => WindsweptGravellyHills,
                "windswept_hills" => WindsweptHills,
                "windswept_savanna" => WindsweptSavanna,
                "wooded_badlands" => WoodedBadlands,
                "mangrove_swamp" => MangroveSwamp,
                "deep_dark" => DeepDark,
                _ => Unknown,
            }),
            None => Ok(Unknown),
        }
    }
}

pub struct Climate {
    pub temperature: f64,
    pub rainfall: f64,
}

impl Biome {
    // Values from https://github.com/erich666/Mineways/blob/master/Win/biomes.cpp
    pub fn climate(self) -> Climate {
        let climate = |t, r| Climate {
            temperature: t,
            rainfall: r,
        };

        use Biome::*;

        match self {
            Ocean => climate(0.5, 0.5),
            Plains => climate(0.8, 0.4),
            Desert => climate(2.0, 0.0),
            Mountains => climate(0.2, 0.3),
            Forest => climate(0.7, 0.8),
            Taiga => climate(0.25, 0.8),
            Swamp => climate(0.8, 0.9),
            River => climate(0.5, 0.5),
            Nether => climate(2.0, 0.0),
            TheEnd => climate(0.5, 0.5),
            FrozenOcean => climate(0.0, 0.5),
            FrozenRiver => climate(0.0, 0.5),
            SnowyTundra => climate(0.0, 0.5),
            SnowyMountains => climate(0.0, 0.5),
            MushroomFields => climate(0.9, 1.0),
            MushroomFieldShore => climate(0.9, 1.0),
            Beach => climate(0.8, 0.4),
            DesertHills => climate(2.0, 0.0),
            WoodedHills => climate(0.7, 0.8),
            TaigaHills => climate(0.25, 0.8),
            MountainEdge => climate(0.2, 0.3),
            Jungle => climate(0.95, 0.9),
            JungleHills => climate(0.95, 0.9),
            JungleEdge => climate(0.95, 0.8),
            DeepOcean => climate(0.5, 0.5),
            StoneShore => climate(0.2, 0.3),
            SnowyBeach => climate(0.05, 0.3),
            BirchForest => climate(0.6, 0.6),
            BirchForestHills => climate(0.6, 0.6),
            DarkForest => climate(0.7, 0.8),
            SnowyTaiga => climate(-0.5, 0.4),
            SnowyTaigaHills => climate(-0.5, 0.4),
            GiantTreeTaiga => climate(0.3, 0.8),
            GiantTreeTaigaHills => climate(0.3, 0.8),
            WoodedMountains => climate(0.2, 0.3),
            Savanna => climate(1.2, 0.0),
            SavannaPlateau => climate(1.0, 0.0),
            Badlands => climate(2.0, 0.0),
            WoodedBadlandsPlateau => climate(2.0, 0.0),
            BadlandsPlateau => climate(2.0, 0.0),
            SmallEndIslands => climate(0.5, 0.5),
            EndMidlands => climate(0.5, 0.5),
            EndHighlands => climate(0.5, 0.5),
            EndBarrens => climate(0.5, 0.5),
            WarmOcean => climate(0.8, 0.5),
            LukewarmOcean => climate(0.8, 0.5),
            ColdOcean => climate(0.8, 0.5),
            DeepWarmOcean => climate(0.8, 0.5),
            DeepLukewarmOcean => climate(0.8, 0.5),
            DeepColdOcean => climate(0.8, 0.5),
            DeepFrozenOcean => climate(0.8, 0.5),
            TheVoid => climate(0.5, 0.5),
            SunflowerPlains => climate(0.8, 0.4),
            DesertLakes => climate(2.0, 0.0),
            GravellyMountains => climate(0.2, 0.3),
            FlowerForest => climate(0.7, 0.8),
            TaigaMountains => climate(0.25, 0.8),
            SwampHills => climate(0.8, 0.9),
            IceSpikes => climate(0.0, 0.5),
            ModifiedJungle => climate(0.95, 0.9),
            ModifiedJungleEdge => climate(0.95, 0.8),
            TallBirchForest => climate(0.6, 0.6),
            TallBirchHills => climate(0.6, 0.6),
            DarkForestHills => climate(0.7, 0.8),
            SnowyTaigaMountains => climate(-0.5, 0.4),
            GiantSpruceTaiga => climate(0.25, 0.8),
            GiantSpruceTaigaHills => climate(0.25, 0.8),
            ModifiedGravellyMountains => climate(0.2, 0.3),
            ShatteredSavanna => climate(1.1, 0.0),
            ShatteredSavannaPlateau => climate(1.0, 0.0),
            ErodedBadlands => climate(2.0, 0.0),
            ModifiedWoodedBadlandsPlateau => climate(2.0, 0.0),
            ModifiedBadlandsPlateau => climate(2.0, 0.0),
            BambooJungle => climate(0.95, 0.9),
            BambooJungleHills => climate(0.95, 0.9),
            SoulSandValley => climate(2.0, 0.0),
            CrimsonForest => climate(2.0, 0.0),
            WarpedForest => climate(2.0, 0.0),
            BasaltDeltas => climate(2.0, 0.0),
            DripstoneCaves => climate(0.8, 0.4),
            FrozenPeaks => climate(-0.7, 0.9),
            JaggedPeaks => climate(-0.7, 0.9),
            Grove => climate(-0.2, 0.8),
            LushCaves => climate(0.5, 0.5),
            Meadow => climate(0.5, 0.8),
            NetherWastes => climate(2.0, 0.0),
            OldGrowthBirchForest => climate(0.6, 0.6),
            OldGrowthPineTaiga => climate(0.3, 0.8),
            OldGrowthSpruceTaiga => climate(0.25, 0.8),
            SnowyPlains => climate(0.0, 0.5),
            SnowySlopes => climate(-0.3, 0.9),
            SparseJungle => climate(0.95, 0.8),
            StonyPeaks => climate(1.0, 0.3),
            StonyShore => climate(0.2, 0.3),
            WindsweptForest => climate(0.2, 0.3),
            WindsweptGravellyHills => climate(0.2, 0.3),
            WindsweptHills => climate(0.2, 0.3),
            WindsweptSavanna => climate(1.1, 0.0),
            WoodedBadlands => climate(2.0, 0.0),
            MangroveSwamp => climate(0.8, 0.9),
            DeepDark => climate(0.8, 0.5),
            Unknown => climate(0.0, 0.0),
        }
    }
}
