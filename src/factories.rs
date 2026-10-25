use crate::gridutils::*;
use crate::types::*;
use macroquad::prelude::*;
use std::collections::*;

pub fn convertItemType(
    mapOfResources: &mut HashMap<IVec2, Traveller>,
    inputType: TravellerItem,
    outputType: TravellerItem,
    factoryPos: IVec2,
) {
    if mapOfResources.get(&factoryPos).unwrap().itemType == Some(inputType) {
        println!("input match, can convert");
        mapOfResources.get_mut(&factoryPos).unwrap().itemType = Some(outputType);
        println!("converted to {:?}", outputType);
    }
}
