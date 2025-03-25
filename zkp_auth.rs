use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_relations::{
    lc,
    ns,
    r1cs::{ConstraintSynthesizer, ConstraintSystem, SynthesisError},
};
use ark_snark::SNARK;
use ark_std::rand::Rng;

#[derive(Clone)]
pub struct AuthCircuit {
    pub secret: Option<Fr>,  // Private input (e.g., hash of credentials)
    pub public: Fr,          // Public output (e.g., commitment)
}

impl ConstraintSynthesizer<Fr> for AuthCircuit {
    fn generate_constraints(
        self,
        cs: &mut ConstraintSystem<Fr>,
    ) -> Result<(), SynthesisError> {
        let secret_var = cs.new_witness_variable(|| self.secret.ok_or(SynthesisError::AssignmentMissing))?;
        let public_var = cs.new_input_variable(|| Ok(self.public))?;
        
        // Enforce: secret * 1 == public
        cs.enforce_constraint(lc!() + secret_var, lc!() + (Fr::one(), public_var), lc!())?;
        
        Ok(())
    }
}

pub struct ZKPAuth {
    pub proving_key: ProvingKey<Bls12_381>,
    pub verifying_key: VerifyingKey<Bls12_381>,
}

impl ZKPAuth {
    pub fn setup<R: Rng>(rng: &mut R) -> Self {
        let circuit = AuthCircuit {
            secret: None,
            public: Fr::from(0),
        };
        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, rng).unwrap();
        Self {
            proving_key: pk,
            verifying_key: vk,
        }
    }

    pub fn generate_proof(&self, secret: Fr, public: Fr) -> Proof<Bls12_381> {
        let circuit = AuthCircuit {
            secret: Some(secret),
            public,
        };
        Groth16::<Bls12_381>::prove(&self.proving_key, circuit, &mut rand::thread_rng()).unwrap()
    }

    pub fn verify_proof(&self, proof: &Proof<Bls12_381>, public: Fr) -> bool {
        let inputs = vec![public];
        Groth16::<Bls12_381>::verify(&self.verifying_key, &inputs, proof).unwrap()
    }
}
