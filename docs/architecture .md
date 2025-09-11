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
  Verifier -->|read registry| VCReg
