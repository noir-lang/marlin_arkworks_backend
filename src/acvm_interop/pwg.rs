use acvm::{
    PartialWitnessGenerator,
};
use acvm::acir::{self, circuit::gate::GadgetCall, native_types::Witness};
use acvm::FieldElement;
use std::collections::BTreeMap;

mod gadget_call;

use self::gadget_call::GadgetCaller;
use super::Marlin;

impl PartialWitnessGenerator for Marlin {
    fn solve_gadget_call(
        initial_witness: &mut BTreeMap<Witness, FieldElement>, 
        gc: &GadgetCall
    ) -> Result<(), acir::OPCODE> {
        GadgetCaller::solve_gadget_call(initial_witness, gc)
    }
    //XXX: marlin is using r1cs? if so truncate should be done with split&join as there is probably no rangecheck
    // a method should be added to acvm PWG that can be overwritten here for the truncate directive
}
