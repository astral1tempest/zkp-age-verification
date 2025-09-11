# Architecture

## Component Diagram
```mermaid
flowchart LR
    subgraph UserSide[User Wallet / Device]
        Cred["ZKP Credential (private)"]
        Prover[ZK Prover]
    end

    Issuer["Issuer (e.g., Registrar/KYC Provider)"]
    Verifier["Verifier Platform (App/Web needing age check)"]

    subgraph Casper[Casper Network]
        VCReg["Credential Registry (Contract)"]
        VRF["Verification Contract (Age ≥ Threshold)"]
        Rev["Revocation List (Contract)"]
    end

    Issuer -->|"issues signed attestation"| Cred
    Cred --> Prover
    Prover -->|"ZK proof: age ≥ N"| Verifier
    Verifier -->|"verifyProof(proof)"| VRF
    VRF -->|"valid/invalid"| Verifier
    Issuer -->|"publish revocations"| Rev
    Verifier -->|"optionally read"| VCReg
sequenceDiagram
    participant U as "User (Wallet + Prover)"
participant I as "Issuer"
participant V as "Verifier App"
participant C as "Casper Contracts"

    rect rgb(245,245,245)
    Note over U,I: Enrollment / Credential Issuance
    U->>I: Present identity & DOB (one-time)
    I->>C: (optional) Register credential hash/metadata
    I->>U: Issue signed credential (kept private)
    end

    rect rgb(245,245,245)
    Note over U,V: Prove age without revealing DOB
    V->>U: Request proof: "age ≥ threshold"
    U->>U: Generate ZK proof locally
    U->>V: Send proof (no DOB/PII)
    V->>C: verifyProof(proof, threshold)
    C->>V: Result: valid/invalid
    V->>U: Access granted/denied
    end

    rect rgb(245,245,245)
    Note over I,C: Revocation (if needed)
    I->>C: Revoke/expire credential
    V->>C: Check revocation at verify time
    end
