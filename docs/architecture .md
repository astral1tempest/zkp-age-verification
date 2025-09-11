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
sequenceDiagram
    participant U as User_Wallet_Prover
    participant I as Issuer
    participant V as Verifier_App
    participant C as Casper_Contracts

    Note over U,I: Enrollment / Credential Issuance
    U ->> I: Present identity & DOB
    I ->> C: (optional) Register credential metadata
    I ->> U: Issue signed credential (kept private)

    Note over U,V: Age Proof without revealing DOB
    V ->> U: Request proof (age >= threshold)
    U ->> U: Generate ZK proof locally
    U ->> V: Send proof (no DOB/PII)
    V ->> C: Verify proof against threshold
    C ->> V: Result valid/invalid
    V ->> U: Access granted or denied

    Note over I,C: Revocation (if needed)
    I ->> C: Revoke or expire credential
    V ->> C: Check revocation at verify time
