flowchart LR
  subgraph UserSide[User Wallet / Device]
    Cred[ZKP Credential private]
    Prover[ZK Prover]
  end

  Issuer[Issuer]
  Verifier[Verifier App]

  subgraph Casper[Casper Network]
    VCReg[Credential Registry]
    VRF[Verification Contract]
    Rev[Revocation List]
  end

  Issuer -->|attests| Cred
  Cred --> Prover
  Prover -->|ZK proof age >= N| Verifier
  Verifier -->|verifyProof| VRF
  VRF -->|valid or invalid| Verifier
  Issuer -->|publish revocations| Rev
  Verifier -->|read registry| VCReg
