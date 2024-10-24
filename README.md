# Namada Mainnet Governance Proposals

Namada mainnet launch is divided into [5 phases](https://namada.net/mainnet-launch). Progressing from one phase to the next requires a governance proposal to active some features. This repository contains the WASM associated with each governance proposal.

> 🔧 NOTE: The current parameter values are temporary and still to be determined. 🔧

## Phase 1 -> 2 (Block party -> Staking party)

Staking rewards are enabled for delegators and validators staking NAM. Public Goods Funding is enabled to support public goods in the Namada ecosystem and beyond.

The reference code can be found in [phase2 folder](./phase2/).

## Phase 2 -> 3 (Staking party -> Shielding party)

Transparent and shielded transfers of governance-enabled IBC assets are enabled. Users can begin shielding assets in the unified shielded set. NAM transfers remain locked until phase 5 (NAM party).

The reference code can be found in [phase3 folder](./phase3/).

## Phase 3 -> 4 (Shielding party -> Shielding Rewards party)

Shielding rewards for governance-enabled IBC assets are enabled. Users begin collecting rewards for shielding assets, which protects their data and helps strengthen Namada’s unified shielded set.

The reference code can be found in [phase4 folder](./phase4/).

## Phase 4 -> 5 (Shielding Reward party -> NAM Party)

When the Namada community is confident that the network is stable, NAM transfers are enabled. All key protocol functionality is now live. From here on, new features and support for new assets can continue to be added by the community via on-chain governance.

The reference code can be found in [phase5 folder](./phase5/).

# How to build 
To generate the wasm artifacts that can be attached to the governance proposal, run:
```
earthly +build
```

This will create a folder called `artifacts` that contains the WASMs.

You can install `earthly` following the official guide [here](https://earthly.dev/get-earthly).

# How to generate a proposal
Use the python script `build_proposal.py` inside the build directory to build any of the four proposals that progress to the next phase:
```
python3 builder/build_proposal.py -d $PARAMETERS_PATH -o $OUTPUT_PATH
```
The `$PARAMETERS_PATH` is a json file that contains some parameters and values needed to properly construct a proposal. For example, to build the phase1 to phase2 proposal:
```
python3 builder/build_proposal.py -d builder/parameters/phase2.json -o proposal.json
```