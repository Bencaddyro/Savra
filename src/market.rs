// ------------------------------------------------------------------------------------------------
/*
/   This file will define market structure
*/
// ------------------------------------------------------------------------------------------------

use std::collections::HashMap;
use crate::Product;
use crate::Location;


// Market: [Location][Product](price,stock)
pub type Market = HashMap<Location, HashMap<Product, (f64, usize)>>;

