# Simple UI for cool cats to register their Ethereum addresses and vote on proposed Milk Funds


## Flow:

```mermaid
sequenceDiagram
    participant Client
    participant RustAPI
    participant Database
    participant CatCommunity
    participant MilkFund
    
    Note over Client,MilkFund: Address Binding Flow
    Client->>Client: connectWallet()
    Client->>RustAPI: POST /api/bind-address {address}
    RustAPI->>RustAPI: generateNonce()
    RustAPI-->>Client: message_to_sign
    Client->>Client: signMessage(message_to_sign)
    Client->>RustAPI: POST /api/bind-address/verify {address, signature}
    RustAPI->>RustAPI: verifySignature()
    RustAPI->>Database: updateUserAddress()
    RustAPI->>CatCommunity: registerMember(address)
    CatCommunity-->>RustAPI: success
    RustAPI-->>Client: success

    Note over Client,MilkFund: MilkFund Setup Flow
    Client->>RustAPI: POST /api/milk-fund/create
    RustAPI->>Database: createMilkFund()
    RustAPI->>MilkFund: createFund(
    Note right of RustAPI: Array of:
    Note right of RustAPI: - recipient addresses
    Note right of RustAPI: - percentages
    Note right of RustAPI: - service descriptions
    MilkFund-->>RustAPI: fundId
    RustAPI->>Database: storeFundDetails()
    RustAPI-->>Client: success

    Note over Client,MilkFund: Fund Details
    Client->>RustAPI: GET /api/milk-fund/current
    RustAPI->>MilkFund: getFundDetails()
    MilkFund-->>RustAPI: {recipients[], percentages[]}
    RustAPI->>Database: getServiceDescriptions()
    RustAPI-->>Client: {recipients[], percentages[], descriptions[]}
```mermaid