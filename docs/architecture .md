## Component Diagram

```mermaid
flowchart LR
  subgraph UserSide["User Wallet / Device"]
    Cred["ZKP Credential (private)"]
    Prover["ZK Prover"]
  end

  Issuer["Issuer (e.g., Registrar/KYC)"]
  Verifier["Verifier App"]

  subgraph Casper["Casper Network"]
    VCReg["Credential Registry"]
    VRF["Verification Contract"]
    Rev["Revocation List"]
  end

  Issuer -->|attests| Cred
  Cred --> Prover
  Prover -->|ZK proof: age >= N| Verifier
  Verifier -->|verify proof| VRF
  VRF -->|valid / invalid| Verifier
  Issuer -->|publish revocations| Rev
  Verifier -->|read registry| VCReg```


## Sequence Diagram

```mermaid
sequenceDiagram
    participant U as "User Wallet and Prover"
    participant I as "Issuer"
    participant V as "Verifier App"
    participant C as "Casper Contracts"

    Note over U,I: Enrollment / Credential Issuance
    U ->> I: Present identity & DOB
    I ->> C: (optional) Register credential hash
    I ->> U: Issue signed credential

    Note over U,V: Age Proof without revealing DOB
    V ->> U: Request proof (age ≥ N)
    U ->> U: Generate ZK proof locally
    U ->> V: Send proof (no DOB/PII)
    V ->> C: Verify proof against registry
    C ->> V: Result valid/invalid
    V ->> U: Access granted/denied

    Note over I,C: Revocation
    I ->> C: Revoke or expire credential
    V ->> C: Check revocation at verification
