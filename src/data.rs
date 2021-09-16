use std::collections::HashMap;

// Auto-generated Location
#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug)]
pub enum Location {
  ArcCorpMiningArea157,
  BensonMiningOutpost,
  Cellin,
  Crusader,
  Daymar,
  DeakinsResearchOutpost,
  GalleteFamilyFarms,
  GrimHex,
  HickesResearchOutpost,
  TerraMillsHydrofarm,
  TramMyersMining,
  Yela
}

// Auto-generated Product
#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug)]
pub enum Product {
  Beryl,
  Aluminium,
  Stims,
  Corundum,
  Widow,
  Fluorine,
  Gold,
  Neon,
  Iodine,
  Titanium,
  Hydrogen,
  Laranite,
  Chlorine,
  Diamond,
  Astatine,
  DistilledSpirits,
  MedicalSupplies,
  ProcessedFood,
  RevenantTreePollen,
  Agricium,
  Waste,
  AgriculturalSupplies,
  Scrap,
  Altruciatoxin,
  Etam,
  Quartz,
  Tungsten
}

impl Product {
pub fn min(&self) -> f64 {
match self {
Product::Beryl => 0 as f64,
Product::Aluminium => 1.11 as f64,
Product::Stims => 2.82 as f64,
Product::Corundum => 2.3 as f64,
Product::Widow => 0 as f64,
Product::Fluorine => 0 as f64,
Product::Gold => 0 as f64,
Product::Neon => 0 as f64,
Product::Iodine => 0 as f64,
Product::Titanium => 7.98 as f64,
Product::Hydrogen => 0 as f64,
Product::Laranite => 27.54 as f64,
Product::Chlorine => 0 as f64,
Product::Diamond => 5.85 as f64,
Product::Astatine => 0 as f64,
Product::DistilledSpirits => 0 as f64,
Product::MedicalSupplies => 15.84 as f64,
Product::ProcessedFood => 0 as f64,
Product::RevenantTreePollen => 1.32 as f64,
Product::Agricium => 0 as f64,
Product::Waste => 0.01 as f64,
Product::AgriculturalSupplies => 1 as f64,
Product::Scrap => 1.49 as f64,
Product::Altruciatoxin => 0 as f64,
Product::Etam => 0 as f64,
Product::Quartz => 0 as f64,
Product::Tungsten => 0 as f64,
}}
pub fn max(&self) -> f64 {
match self {
Product::Beryl => 4.35 as f64,
Product::Aluminium => 1.3 as f64,
Product::Stims => 3.8 as f64,
Product::Corundum => 2.71 as f64,
Product::Widow => 126.03 as f64,
Product::Fluorine => 2.95 as f64,
Product::Gold => 6.41 as f64,
Product::Neon => 89.34 as f64,
Product::Iodine => 0.45 as f64,
Product::Titanium => 8.8 as f64,
Product::Hydrogen => 1.11 as f64,
Product::Laranite => 30.57 as f64,
Product::Chlorine => 1.71 as f64,
Product::Diamond => 6.98 as f64,
Product::Astatine => 9 as f64,
Product::DistilledSpirits => 5.56 as f64,
Product::MedicalSupplies => 18.11 as f64,
Product::ProcessedFood => 1.5 as f64,
Product::RevenantTreePollen => 0 as f64,
Product::Agricium => 27.5 as f64,
Product::Waste => 0 as f64,
Product::AgriculturalSupplies => 1.2 as f64,
Product::Scrap => 0 as f64,
Product::Altruciatoxin => 52.43 as f64,
Product::Etam => 101.36 as f64,
Product::Quartz => 1.55 as f64,
Product::Tungsten => 4.03 as f64,
}}
pub fn all() -> [Product;27] {
[Product::Beryl,
Product::Aluminium,
Product::Stims,
Product::Corundum,
Product::Widow,
Product::Fluorine,
Product::Gold,
Product::Neon,
Product::Iodine,
Product::Titanium,
Product::Hydrogen,
Product::Laranite,
Product::Chlorine,
Product::Diamond,
Product::Astatine,
Product::DistilledSpirits,
Product::MedicalSupplies,
Product::ProcessedFood,
Product::RevenantTreePollen,
Product::Agricium,
Product::Waste,
Product::AgriculturalSupplies,
Product::Scrap,
Product::Altruciatoxin,
Product::Etam,
Product::Quartz,
Product::Tungsten
]}
}


// Auto-generated get_map()
pub fn get_map() -> HashMap<Location,Vec<(Location,f64)>> {[
(Location::Cellin,[(Location::Crusader,50 as f64),(Location::GalleteFamilyFarms,50 as f64),(Location::TerraMillsHydrofarm,50 as f64),(Location::TramMyersMining,50 as f64),(Location::HickesResearchOutpost,50 as f64),].iter().cloned().collect()),
(Location::HickesResearchOutpost,[(Location::Cellin,50 as f64),].iter().cloned().collect()),
(Location::TerraMillsHydrofarm,[(Location::Cellin,50 as f64),].iter().cloned().collect()),
(Location::TramMyersMining,[(Location::Cellin,50 as f64),].iter().cloned().collect()),
(Location::GalleteFamilyFarms,[(Location::Cellin,50 as f64),].iter().cloned().collect()),
(Location::Crusader,[(Location::Yela,50 as f64),(Location::Cellin,50 as f64),(Location::Daymar,100 as f64),].iter().cloned().collect()),
(Location::Daymar,[(Location::Yela,100 as f64),(Location::Cellin,100 as f64),(Location::Crusader,100 as f64),].iter().cloned().collect()),
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
(Location::GrimHex,[(Product::Agricium,27.5 as f64),(Product::AgriculturalSupplies,1.15 as f64),(Product::Altruciatoxin,52.43 as f64),(Product::Aluminium,1.3 as f64),(Product::Astatine,9 as f64),(Product::Beryl,4.35 as f64),(Product::Chlorine,1.71 as f64),(Product::Corundum,2.71 as f64),(Product::Diamond,6.98 as f64),(Product::DistilledSpirits,5.46 as f64),(Product::Etam,101.36 as f64),(Product::Fluorine,2.94 as f64),(Product::Gold,6.41 as f64),(Product::Hydrogen,1.11 as f64),(Product::Iodine,0.45 as f64),(Product::Laranite,30.57 as f64),(Product::Neon,89.34 as f64),(Product::ProcessedFood,1.5 as f64),(Product::Quartz,1.55 as f64),(Product::Stims,3.8 as f64),(Product::Titanium,8.8 as f64),(Product::Tungsten,4.03 as f64),(Product::Widow,126.03 as f64),].iter().cloned().collect()),
(Location::BensonMiningOutpost,[(Product::DistilledSpirits,5.55 as f64),(Product::MedicalSupplies,17.55 as f64),(Product::ProcessedFood,1.5 as f64),(Product::Stims,3.8 as f64),].iter().cloned().collect()),
(Location::ArcCorpMiningArea157,[(Product::DistilledSpirits,5.45 as f64),(Product::MedicalSupplies,17.51 as f64),(Product::ProcessedFood,1.5 as f64),(Product::Stims,3.77 as f64),].iter().cloned().collect()),
(Location::DeakinsResearchOutpost,[(Product::Astatine,8.23 as f64),(Product::Chlorine,1.7 as f64),(Product::DistilledSpirits,5.56 as f64),(Product::Fluorine,2.84 as f64),(Product::Iodine,0.45 as f64),(Product::ProcessedFood,1.5 as f64),].iter().cloned().collect()),
].iter().cloned().collect()}

