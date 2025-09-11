# Architecture

## Component Diagram

```mermaid
flowchart LR
    subgraph UserSide["User Wallet / Device"]
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
    Prover -->|ZK proof: age >= N| Verifier
    Verifier -->|verify proof| VRF
    VRF -->|valid / invalid| Verifier
    Issuer -->|publish revocations| Rev
    Verifier -->|read registry| VCReg
sequenceDiagram
    participant U as User_Wallet_Prover
    participant I as Issuer
    participant V as Verifier_App
    participant C as Casper_Contracts

    rect rgb(245,245,245)
    Note over U,I: Enrollment / Credential Issuance
    U ->> I: Present identity & DOB (one-time)
    I ->> C: (optional) Register credential hash
    I ->> U: Issue signed credential
    end

    rect rgb(245,245,245)
    Note over U,V: Age Proof without revealing DOB
    V ->> U: Request proof (age >= threshold)
    U ->> U: Generate ZK proof locally
    U ->> V: Send proof (no DOB/PII)
    V ->> C: Verify proof
    C ->> V: Result: valid/invalid
    V ->> U: Access granted/denied
    end

    rect rgb(245,245,245)
    Note over I,C: Revocation (if needed)
    I ->> C: Revoke / expire credential
    V ->> C: Check revocation at verification
    end
