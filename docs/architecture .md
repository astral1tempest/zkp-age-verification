## Component Diagram

```mermaid
flowchart LR
  subgraph UserSide[User Wallet / Device]
    Cred[ZKP Credential (private)]
    Prover[ZK Prover]
  end

  Issuer[Issuer (e.g., Registrar/KYC)]
  Verifier[Verifier App]

  subgraph Casper[Casper Network]
    VCReg[Credential Registry]
    VRF[Verification Contract]
    Rev[Revocation List]
  end

  Issuer -->|attests| Cred
  Cred --> Prover
  Prover -->|ZK proof: age >= N| Verifier
  Verifier -->|verify proof| VRF
  VRF -->|valid / invalid| Verifier
  Issuer -->|publish revocations| Rev
  Verifier -->|read registry| VCReg```
sequenceDiagram
  actor U as User Wallet + Prover
  participant I as Issuer
  participant V as Verifier App
  participant C as Casper Contracts

  %% Enrollment / Issuance
  U->>I: Present identity and DOB (one-time)
  I->>C: (optional) Register credential hash
  I-->>U: Issue signed credential (kept private)

  %% Age proof
  V->>U: Request proof (age >= threshold)
  U->>U: Generate ZK proof locally
  U-->>V: Send proof (no DOB/PII)
  V->>C: verifyProof(proof, threshold)
  C-->>V: Result valid / invalid
  V-->>U: Access granted / denied

  %% Revocation
  I->>C: Revoke or expire credential
  V->>C: Check revocation at verify time
