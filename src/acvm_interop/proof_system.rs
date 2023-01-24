use std::collections::BTreeMap;

use acvm::acir::{circuit::Circuit, native_types::Witness, BlackBoxFunc};
use acvm::{FieldElement, Language, ProofSystemCompiler};

use super::Marlin;

impl ProofSystemCompiler for Marlin {
    fn get_exact_circuit_size(&self, _: Circuit) -> u32 {
        todo!()
    }

    fn blackbox_function_supported(&self, _: &BlackBoxFunc) -> bool {
        false
    }

    fn prove_with_meta(
        &self,
        circuit: Circuit,
        witness_values: BTreeMap<Witness, FieldElement>,
    ) -> Vec<u8> {
        // XXX: modify arkworks serialiser to accept the BTreeMap
        let values: Vec<_> = witness_values.values().collect();
        arkworks_backend::prove(circuit, values)
    }

    fn verify_from_cs(
        &self,
        proof: &[u8],
        public_inputs: Vec<FieldElement>,
        circuit: Circuit,
    ) -> bool {
        arkworks_backend::verify(circuit, proof, public_inputs)
    }

    fn np_language(&self) -> Language {
        Language::R1CS
    }
}
