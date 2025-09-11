# Architecture

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
    Prover -->|ZK proof: age ≥ N| Verifier
    Verifier -->|verifyProof| VRF
    VRF -->|valid/invalid| Verifier
    Issuer -->|publish revocations| Rev
    Verifier -->|read registry| VCReg

    %% ----- Styles -----
    classDef user fill:#0ea5e9,stroke:#075985,stroke-width:1px,color:#0b1020;
    classDef app  fill:#f59e0b,stroke:#92400e,stroke-width:1px,color:#0b1020;
    classDef chain fill:#10b981,stroke:#065f46,stroke-width:1px,color:#0b1020;
    classDef neutral fill:#cbd5e1,stroke:#475569,stroke-width:1px,color:#0b1020;

    class Cred,Prover user
    class Issuer,Verifier app
    class VCReg,VRF,Rev chain
    class UserSide,Casper neutral
sequenceDiagram
    %% Participants
    participant U as "User Wallet and Prover"
    participant I as "Issuer"
    participant V as "Verifier App"
    participant C as "Casper Contracts"

    %% Enrollment / Issuance
    rect rgb(224,242,254)
    Note over U,I: Enrollment / Credential Issuance
    U ->> I: Present identity & DOB
    I ->> C: (optional) Register credential hash
    I ->> U: Issue signed credential
    end

    %% Age Proof
    rect rgb(254,249,195)
    Note over U,V: Age Proof without revealing DOB
    V ->> U: Request proof (age ≥ N)
    U ->> U: Generate ZK proof locally
    U ->> V: Send proof (no DOB/PII)
    V ->> C: Verify proof
    C ->> V: Result valid/invalid
    V ->> U: Access granted/denied
    end

    %% Revocation
    rect rgb(209,250,229)
    Note over I,C: Revocation
    I ->> C: Revoke or expire credential
    V ->> C: Check revocation at verification
    end
