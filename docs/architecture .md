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

    subgraph Casper[Casper Network]
        VCReg["Credential Registry"]
        VRF["Verification Contract"]
        Rev["Revocation List"]
    end

    Issuer --> Cred
    Cred --> Prover
    Prover --> Verifier
    Verifier --> VRF
    VRF --> Verifier
    Issuer --> Rev
    Verifier --> VCReg
sequenceDiagram
    participant U as User (Wallet + Prover)
    participant I as Issuer
    participant V as Verifier App
    participant C as Casper Contracts

    rect rgb(245,245,245)
    Note over U,I: Enrollment / Credential Issuance
    U ->> I: Present identity & DOB (one-time)
    I ->> C: (optional) Register credential metadata
    I ->> U: Issue signed credential (kept private)
    end

    rect rgb(245,245,245)
    Note over U,V: Age Proof without revealing DOB
    V ->> U: Request proof (age ≥ threshold)
    U ->> U: Generate ZK proof locally
    U ->> V: Send proof (no DOB/PII)
    V ->> C: Verify proof
    C ->> V: Return result (valid/invalid)
    V ->> U: Access granted/denied
    end

    rect rgb(245,245,245)
    Note over I,C: Revocation
    I ->> C: Revoke or expire credential
    V ->> C: Check revocation
    end
