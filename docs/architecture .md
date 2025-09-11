```mermaid
flowchart LR
    subgraph UserSide[User Wallet / Device]
        Cred[ZKP Credential (private)]P
        Prover[ZK Prover]
    end

    Issuer[Issuer (e.g., Registrar/KYC Provider)]
    Verifier[Verifier Platform (App/Web needing age check)]
    
    subgraph Casper[Casper Network]
        VCReg[Credential Registry (Contract)]
        VRF[Verification Contract (Age ≥ Threshold)]
        Rev[Revocation List (Contract)]
    end

    Issuer -->|issues signed attestation| Cred
    Cred --> Prover
    Prover -->|ZK proof: age ≥ N| Verifier
    Verifier -->|verifyProof(proof)| VRF
    VRF -->|valid/invalid| Verifier
    Issuer -->|publish revocations| Rev
    Verifier -->|optionally read| VCReg```
