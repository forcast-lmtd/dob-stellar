# dob-stellar

A Stellar blockchain-based project management and evaluation system that enables decentralized project submission, approval, and scoring through smart contracts.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Smart Contract Documentation](#smart-contract-documentation)
- [Development](#development)
- [Testing](#testing)
- [Deployment](#deployment)
- [License](#license)
- [Contributing](#contributing)

## Overview

This project implements a decentralized project management system on the Stellar network. It allows users to submit projects for evaluation, enables whitelisted reviewers to approve or reject projects, and maintains a scoring system (TRUFA scores) for approved projects.

## Features

- **Project Submission**: Users can submit projects for evaluation
- **Decentralized Review Process**: Whitelisted addresses can approve/reject projects
- **Project Status Management**: Track projects through different states (Pending, Approved, Rejected)
- **TRUFA Scoring System**: Multi-dimensional scoring including:
  - Technical Feasibility
  - Regulatory Compliance
  - Financial Viability
  - Environment Impact
  - Overall TRUFA Score
- **Bulk Operations**: Query multiple project statuses efficiently
- **Admin Controls**: Administrative functions for whitelist management
- **Project Reset Functionality**: Ability to reset project statuses

## Project Structure

```
.
├── README.md                           # This file
├── LICENSE                            # Boost Software License
├── .gitignore                         # Git ignore rules
└── stellar-contract/                  # Stellar smart contracts
    ├── .gitignore
    ├── Cargo.lock
    ├── Cargo.toml                     # Workspace configuration
    ├── README.md                      # Soroban project documentation
    ├── .stellar/                      # Stellar CLI configuration
    │   └── contract-ids/
    │       └── projects.json
    └── contracts/
        └── projects/                  # Main projects contract
            ├── Cargo.toml
            ├── Makefile               # Build and test commands
            ├── src/
            │   ├── lib.rs
            │   ├── contract.rs        # Main contract implementation
            │   └── test.rs            # Contract tests
            └── test_snapshots/        # Test snapshot files
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools)
- [Soroban SDK](https://soroban.stellar.org/docs/getting-started/setup)

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd dob-stellar
   ```

2. Install dependencies:
   ```bash
   cd stellar-contract
   cargo check
   ```

3. Build the contracts:
   ```bash
   cd contracts/projects
   make build
   ```

## Smart Contract Documentation

### Main Contract: Projects

The projects contract is the core component that manages:

#### Key Functions

- `add_project(user: Address, project_hash: BytesN<32>)` - Submit a new project
- `set_project_approved(reviewer: Address, project_hash: BytesN<32>, scores: TrufaScoreValues)` - Approve a project with TRUFA scores
- `set_project_rejected(reviewer: Address, project_hash: BytesN<32>)` - Reject a project
- `get_project_status(project_hash: BytesN<32>) -> ProjectStatusEnum` - Get project status
- `get_all_projects_statuses() -> Vec<ProjectData>` - Get all projects and their statuses
- `reset_project(admin: Address, project_hash: BytesN<32>)` - Reset project status (admin only)

#### Project Status States

- `NotSet` - Project doesn't exist
- `Pending` - Project submitted, awaiting review
- `Approved` - Project approved with TRUFA scores
- `Rejected` - Project rejected by reviewer

#### TRUFA Score Components

- Technical Feasibility (0-100)
- Regulatory Compliance (0-100) 
- Financial Viability (0-100)
- Environment Impact (0-100)
- Overall TRUFA Score (0-100)

## Development

### Building

```bash
cd stellar-contract/contracts/projects
make build
```

### Running Tests

```bash
cd stellar-contract/contracts/projects
make test
```

### Code Formatting

```bash
cd stellar-contract/contracts/projects
make fmt
```

### Cleaning Build Artifacts

```bash
cd stellar-contract/contracts/projects
make clean
```

## Testing

The project includes comprehensive tests covering:

- Project submission and lifecycle management
- Approval and rejection workflows
- TRUFA scoring system
- Bulk query operations
- Administrative functions
- Error handling and edge cases

Test snapshots are maintained in [`stellar-contract/contracts/projects/test_snapshots/`](stellar-contract/contracts/projects/test_snapshots/) for regression testing.

## Deployment

[Add deployment instructions once available]

## License

This project is licensed under the [Boost Software License](LICENSE) - see the LICENSE file for details.

## Contributing

[Add contributing guidelines]

---

For more detailed information about Soroban development, see the [Stellar Documentation](https://developers.stellar.org/docs/smart-contracts).