use std::collections::HashMap;
use strum::EnumString;

// Auto-generated Location
#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug, EnumString)]
pub enum Location {
  ArcCorpMiningArea141,
  ArcCorpMiningArea157,
  BensonMiningOutpost,
  BountifulHarvestHydroponics,
  Cellin,
  Crusader,
  Daymar,
  DeakinsResearchOutpost,
  GalleteFamilyFarms,
  GrimHex,
  HickesResearchOutpost,
  KudreOre,
  ShubinMiningFacilityScd1,
  TerraMillsHydrofarm,
  TramMyersMining,
  Yela
}

// Auto-generated Product
#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug)]
pub enum Product {
  Chlorine,
  Aluminium,
  Stims,
  Gold,
  Agricium,
  MedicalSupplies,
  Hydrogen,
  Corundum,
  Tungsten,
  Scrap,
  Etam,
  Titanium,
  Altruciatoxin,
  Fluorine,
  Astatine,
  Laranite,
  Waste,
  DistilledSpirits,
  Iodine,
  RevenantTreePollen,
  Diamond,
  Beryl,
  ProcessedFood,
  Quartz,
  Neon,
  AgriculturalSupplies,
  Widow
}

impl Product {
pub fn min(&self) -> f64 {
match self {
Product::Chlorine => 0 as f64,
Product::Aluminium => 1.11 as f64,
Product::Stims => 2.82 as f64,
Product::Gold => 5.67 as f64,
Product::Agricium => 24.44 as f64,
Product::MedicalSupplies => 15.84 as f64,
Product::Hydrogen => 0 as f64,
Product::Corundum => 2.3 as f64,
Product::Tungsten => 3.55 as f64,
Product::Scrap => 1.49 as f64,
Product::Etam => 0 as f64,
Product::Titanium => 7.98 as f64,
Product::Altruciatoxin => 0 as f64,
Product::Fluorine => 0 as f64,
Product::Astatine => 0 as f64,
Product::Laranite => 27.54 as f64,
Product::Waste => 0.01 as f64,
Product::DistilledSpirits => 0 as f64,
Product::Iodine => 0 as f64,
Product::RevenantTreePollen => 1.32 as f64,
Product::Diamond => 5.85 as f64,
Product::Beryl => 4.05 as f64,
Product::ProcessedFood => 0 as f64,
Product::Quartz => 1.25 as f64,
Product::Neon => 0 as f64,
Product::AgriculturalSupplies => 1 as f64,
Product::Widow => 0 as f64,
}}
pub fn max(&self) -> f64 {
match self {
Product::Chlorine => 1.71 as f64,
Product::Aluminium => 1.3 as f64,
Product::Stims => 3.8 as f64,
Product::Gold => 6.41 as f64,
Product::Agricium => 27.5 as f64,
Product::MedicalSupplies => 18.11 as f64,
Product::Hydrogen => 1.11 as f64,
Product::Corundum => 2.71 as f64,
Product::Tungsten => 4.03 as f64,
Product::Scrap => 0 as f64,
Product::Etam => 101.36 as f64,
Product::Titanium => 8.8 as f64,
Product::Altruciatoxin => 52.43 as f64,
Product::Fluorine => 2.95 as f64,
Product::Astatine => 9 as f64,
Product::Laranite => 30.57 as f64,
Product::Waste => 0 as f64,
Product::DistilledSpirits => 5.56 as f64,
Product::Iodine => 0.45 as f64,
Product::RevenantTreePollen => 0 as f64,
Product::Diamond => 6.98 as f64,
Product::Beryl => 4.35 as f64,
Product::ProcessedFood => 1.5 as f64,
Product::Quartz => 1.55 as f64,
Product::Neon => 89.34 as f64,
Product::AgriculturalSupplies => 1.2 as f64,
Product::Widow => 126.03 as f64,
}}
pub fn all() -> [Product;27] {
[Product::Chlorine,
Product::Aluminium,
Product::Stims,
Product::Gold,
Product::Agricium,
Product::MedicalSupplies,
Product::Hydrogen,
Product::Corundum,
Product::Tungsten,
Product::Scrap,
Product::Etam,
Product::Titanium,
Product::Altruciatoxin,
Product::Fluorine,
Product::Astatine,
Product::Laranite,
Product::Waste,
Product::DistilledSpirits,
Product::Iodine,
Product::RevenantTreePollen,
Product::Diamond,
Product::Beryl,
Product::ProcessedFood,
Product::Quartz,
Product::Neon,
Product::AgriculturalSupplies,
Product::Widow
]}
}


// Auto-generated get_map()
pub fn get_map() -> HashMap<Location,Vec<(Location,f64)>> {[
(Location::Cellin,[(Location::Crusader,50 as f64),(Location::GalleteFamilyFarms,50 as f64),(Location::TerraMillsHydrofarm,50 as f64),(Location::TramMyersMining,50 as f64),(Location::HickesResearchOutpost,50 as f64),].iter().cloned().collect()),
(Location::HickesResearchOutpost,[(Location::Cellin,50 as f64),].iter().cloned().collect()),
(Location::TerraMillsHydrofarm,[(Location::Cellin,50 as f64),].iter().cloned().collect()),
(Location::TramMyersMining,[(Location::Cellin,50 as f64),].iter().cloned().collect()),
(Location::GalleteFamilyFarms,[(Location::Cellin,50 as f64),].iter().cloned().collect()),
(Location::Crusader,[(Location::Yela,50 as f64),(Location::Cellin,50 as f64),(Location::Daymar,50 as f64),].iter().cloned().collect()),
(Location::Daymar,[(Location::Crusader,50 as f64),(Location::ArcCorpMiningArea141,50 as f64),(Location::BountifulHarvestHydroponics,50 as f64),(Location::KudreOre,50 as f64),(Location::ShubinMiningFacilityScd1,50 as f64),].iter().cloned().collect()),
(Location::ArcCorpMiningArea141,[(Location::Daymar,50 as f64),].iter().cloned().collect()),
(Location::BountifulHarvestHydroponics,[(Location::Daymar,50 as f64),].iter().cloned().collect()),
(Location::KudreOre,[(Location::Daymar,50 as f64),].iter().cloned().collect()),
(Location::ShubinMiningFacilityScd1,[(Location::Daymar,50 as f64),].iter().cloned().collect()),
(Location::Yela,[(Location::Crusader,50 as f64),(Location::GrimHex,50 as f64),(Location::BensonMiningOutpost,50 as f64),(Location::DeakinsResearchOutpost,50 as f64),(Location::ArcCorpMiningArea157,50 as f64),].iter().cloned().collect()),
(Location::GrimHex,[(Location::Yela,50 as f64),].iter().cloned().collect()),
(Location::BensonMiningOutpost,[(Location::Yela,50 as f64),].iter().cloned().collect()),
(Location::ArcCorpMiningArea157,[(Location::Yela,50 as f64),].iter().cloned().collect()),
(Location::DeakinsResearchOutpost,[(Location::Yela,50 as f64),].iter().cloned().collect()),
].iter().cloned().collect()}

// Auto-generated get_buy()
pub fn get_buy() -> HashMap<Location,HashMap<Product,f64>> {[
(Location::HickesResearchOutpost,[(Product::AgriculturalSupplies,1 as f64),(Product::MedicalSupplies,17.05 as f64),(Product::Stims,2.82 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::TerraMillsHydrofarm,[(Product::DistilledSpirits,4.08 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::TramMyersMining,[(Product::Aluminium,1.11 as f64),(Product::Corundum,2.3 as f64),(Product::Diamond,5.85 as f64),(Product::Laranite,27.54 as f64),(Product::Titanium,7.98 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::GalleteFamilyFarms,[(Product::ProcessedFood,1.2 as f64),(Product::RevenantTreePollen,1.32 as f64),(Product::DistilledSpirits,4.58 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::ArcCorpMiningArea141,[(Product::Agricium,24.94 as f64),(Product::Beryl,4.09 as f64),(Product::Corundum,2.3 as f64),(Product::Quartz,1.26 as f64),(Product::Tungsten,3.56 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::BountifulHarvestHydroponics,[(Product::ProcessedFood,1.2 as f64),(Product::RevenantTreePollen,1.32 as f64),(Product::DistilledSpirits,4.65 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::KudreOre,[(Product::Agricium,24.44 as f64),(Product::Beryl,4.05 as f64),(Product::Gold,5.67 as f64),(Product::Quartz,1.25 as f64),(Product::Tungsten,3.55 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::ShubinMiningFacilityScd1,[(Product::Beryl,4.05 as f64),(Product::Corundum,2.3 as f64),(Product::Gold,5.86 as f64),(Product::Quartz,1.25 as f64),(Product::Tungsten,3.8 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::GrimHex,[(Product::Scrap,1.49 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::BensonMiningOutpost,[(Product::Astatine,7.87 as f64),(Product::Chlorine,1.3 as f64),(Product::Fluorine,2.35 as f64),(Product::Hydrogen,0.9 as f64),(Product::Iodine,0.35 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::ArcCorpMiningArea157,[(Product::Astatine,7 as f64),(Product::Chlorine,1.3 as f64),(Product::Fluorine,2.35 as f64),(Product::Hydrogen,0.9 as f64),(Product::Iodine,0.35 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
(Location::DeakinsResearchOutpost,[(Product::AgriculturalSupplies,1 as f64),(Product::MedicalSupplies,15.84 as f64),(Product::Stims,2.92 as f64),(Product::Waste,0.01 as f64),].iter().cloned().collect()),
].iter().cloned().collect()}

// Auto-generated get_sell()
pub fn get_sell() -> HashMap<Location,HashMap<Product,f64>> {[
(Location::HickesResearchOutpost,[(Product::Astatine,8.51 as f64),(Product::Chlorine,1.7 as f64),(Product::Fluorine,2.95 as f64),(Product::DistilledSpirits,5.55 as f64),(Product::Iodine,0.45 as f64),(Product::ProcessedFood,1.5 as f64),].iter().cloned().collect()),
(Location::TerraMillsHydrofarm,[(Product::AgriculturalSupplies,1.2 as f64),(Product::MedicalSupplies,18.11 as f64),(Product::ProcessedFood,1.5 as f64),].iter().cloned().collect()),
(Location::TramMyersMining,[(Product::MedicalSupplies,11.56 as f64),].iter().cloned().collect()),
(Location::GalleteFamilyFarms,[(Product::AgriculturalSupplies,1.11 as f64),(Product::MedicalSupplies,17.81 as f64),].iter().cloned().collect()),
(Location::ArcCorpMiningArea141,[(Product::DistilledSpirits,5.47 as f64),(Product::MedicalSupplies,17.53 as f64),(Product::ProcessedFood,1.5 as f64),(Product::Stims,3.8 as f64),].iter().cloned().collect()),
(Location::BountifulHarvestHydroponics,[(Product::AgriculturalSupplies,1.14 as f64),(Product::MedicalSupplies,17.58 as f64),(Product::Stims,3.58 as f64),].iter().cloned().collect()),
(Location::KudreOre,[(Product::DistilledSpirits,5.55 as f64),(Product::MedicalSupplies,17.67 as f64),(Product::ProcessedFood,1.5 as f64),(Product::Stims,3.8 as f64),].iter().cloned().collect()),
(Location::ShubinMiningFacilityScd1,[(Product::DistilledSpirits,5.55 as f64),(Product::MedicalSupplies,17.79 as f64),(Product::ProcessedFood,1.5 as f64),(Product::Stims,3.8 as f64),].iter().cloned().collect()),
(Location::GrimHex,[(Product::Agricium,27.5 as f64),(Product::AgriculturalSupplies,1.15 as f64),(Product::Altruciatoxin,52.43 as f64),(Product::Aluminium,1.3 as f64),(Product::Astatine,9 as f64),(Product::Beryl,4.35 as f64),(Product::Chlorine,1.71 as f64),(Product::Corundum,2.71 as f64),(Product::Diamond,6.98 as f64),(Product::DistilledSpirits,5.46 as f64),(Product::Etam,101.36 as f64),(Product::Fluorine,2.94 as f64),(Product::Gold,6.41 as f64),(Product::Hydrogen,1.11 as f64),(Product::Iodine,0.45 as f64),(Product::Laranite,30.57 as f64),(Product::Neon,89.34 as f64),(Product::ProcessedFood,1.5 as f64),(Product::Quartz,1.55 as f64),(Product::Stims,3.8 as f64),(Product::Titanium,8.8 as f64),(Product::Tungsten,4.03 as f64),(Product::Widow,126.03 as f64),].iter().cloned().collect()),
(Location::BensonMiningOutpost,[(Product::DistilledSpirits,5.55 as f64),(Product::MedicalSupplies,17.55 as f64),(Product::ProcessedFood,1.5 as f64),(Product::Stims,3.8 as f64),].iter().cloned().collect()),
(Location::ArcCorpMiningArea157,[(Product::DistilledSpirits,5.45 as f64),(Product::MedicalSupplies,17.51 as f64),(Product::ProcessedFood,1.5 as f64),(Product::Stims,3.77 as f64),].iter().cloned().collect()),
(Location::DeakinsResearchOutpost,[(Product::Astatine,8.23 as f64),(Product::Chlorine,1.7 as f64),(Product::DistilledSpirits,5.56 as f64),(Product::Fluorine,2.84 as f64),(Product::Iodine,0.45 as f64),(Product::ProcessedFood,1.5 as f64),].iter().cloned().collect()),
].iter().cloned().collect()}

