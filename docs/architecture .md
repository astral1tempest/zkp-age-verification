# Architecture

## Component Diagram
```mermaid
flowchart LR
    subgraph UserSide[User Wallet / Device]
        Cred["ZKP Credential (private)"]
        Prover["ZK Prover"]
    end

    Issuer["Issuer (e.g., Registrar/KYC Provider)"]
    Verifier["Verifier App"]

    subgraph Casper["Casper Network"]
        VCReg["Credential Registry"]
        VRF["Verification Contract"]
        Rev["Revocation List"]
    end

    Issuer -->|attests| Cred
    Cred --> Prover
    Prover -->|ZK proof: age ≥ N| Verifier
    Verifier -->|verifyProof(proof)| VRF
    VRF -->|valid/invalid| Verifier
    Issuer -->|publish revocations| Rev
    Verifier -->|read registry| VCReg
sequenceDiagram
    participant U as "User Wallet + Prover"
    participant I as Issuer
    participant V as Verifier App
    participant C as Casper Contracts

    rect rgb(245,245,245)
    Note over U,I: Enrollment / Credential Issuance
    U ->> I: Present identity & DOB
    I ->> C: (optional) Register credential hash
    I ->> U: Issue signed credential
    end

    rect rgb(245,245,245)
    Note over U,V: Age Proof
    V ->> U: Request proof (age ≥ threshold)
    U ->> U: Generate ZK proof locally
    U ->> V: Send proof (no DOB/PII)
    V ->> C: Verify proof against contract
    C ->> V: Result valid/invalid
    V ->> U: Access granted/denied
    end

    rect rgb(245,245,245)
    Note over I,C: Revocation
    I ->> C: Revoke or expire credential
    V ->> C: Check revocation at verification
    end
