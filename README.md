# Savra
**StarCitizen commodities trading tool**

## General

As many martingale provider flourish, like Versemate, Gallog, uexcorp, SC trade tools, here yet another "Best trade route" finders. With somes differences, no website, no fancy GUI, all source open, all data open. Up to you to test it on your computer ! (But do not ask me how to run it on MS windows :shrug:). I do not claim this tool is perfect, bug free or accurate ! Always double check what you plan. Still feel free to contribute to this tool if you want :hugs:.

## Current feature

- Able to load Universe data model
- Able to find best behavior from a starting location, initial invest and a time frame
- Output Universe data model as .dot file
- Output possiblities tree as .dot file (not recommended)

## Planned feature roadmap / improvement
(Because every SC project need a roadmap :grimacing:)


### Ingress data model

- [ ] Simpler syntax for bi-directionnal location link
- [ ] Complete data model with more destinations, accurate travel time
- [ ] Ability to print logical map of the Universe
- [ ] Add ship profile
- [ ] Miscellanous info on Location (armistice, hidden, pads, ground vehicle spawn)
- [ ] Miscellanous info on Product (gas / metal / illegal)

### Post process & visualisation

- [ ] Complete refacto for .dot printing, chronological x location node position
- [ ] Node formating, float width
- [ ] Node formating, name width
- [ ] Node formating, cargo display
- [ ] Node formating, State vs Action node, shape, arrow
- [ ] .dot toolchain to render graph ?

### Engine

- [ ] Add ship profile (cargo, fuel, speed/travel time, landing ability)
- [ ] Update heuristic, location attractiveness based on product max profit
- [ ] Cargo update memory footprint, switch to RwLock
- [ ] Syntax standardization to access hashmap map[key] vs map.get(key).unwrap()
- [ ] Const array for location

### Flow refactor & retroaction

- [ ] Implement flow logic for Buy/Sell actions
- [ ] Add flow information on data model
- [ ] Introduce price variability from current agent

### Resolution logic

- [ ] Introduce action wait in children generation
- [ ] Introduce fine graine amount choice in children generation
- [ ] comparison of A* pruning method over Monte Carlo variant

### Adressing Time vs Memory

State of node are not stored in memory bu recursively computed on demand, this reduce memory foot print but increase execution time, an hybrid approach need to be found to transform hollow node by counting call and recusrion lenght. This problem need to be addressed to increase perf (?)

### Futur work

- [ ] Add danger factor (pirate, escort, 30k)
- [ ] Support celestial movement for accurate distance & time & ephemeris
- [ ] Support same-travel delivery quest ?
- [ ] Fancy GUI ?

## Install

1. Clone the repo
2. (Optional) Adjust data model
3. Run !

## Options

Currently there is 4 value to set :
```
-c, --cargo <cargo>          Cargo capacity
-l, --location <location>    Starting location
-m, --money <money>          Starting money
-t, --time <time>            Time limit for the run
```

Additionaly, depending on your hardware you might want to set number of thread
```
-n, --thread <thread>        Number of thread for parallel computing
```


## Data model

Universe, Product, Location are written in `data_model/*.yml` files. Each file is a list of location:

```yaml
- location: location1_name
  destination:
    - location: location2_name
      distance: 271828
    - ...
  buy:
    - product: product_name
      price: 3.14
    - ...
  sell:
    - product: product_name
      price: 1.618
    - ...
- location: location2_name
  destination:
    - location: location1_name
      distance: 83462
    - ...
```

`buy` and `sell` element are optional, a location can only allow buying things or only selling things or is only a travel nexus.
Every location in destination must have their entry somewhere (not necessarily on same file).
For now only distance stand for location to location link, further refinement is planned. Link can be asymetric (place A to place B but no A from B) and so distance (think lift off vs landing, not same duration)


## Resolution principle

Trader behavior are described with 4 base actions:
- Move
- Buy
- Sell
- Wait

State node are composed of a location, a wallet, a cargo, a time delta from run beginning.
Path exploration algorithm used is __A*__ with a custom heuristic.

## Thanks & Inspiration

Node strucutre is heavily inspired by [this post](https://developerlife.com/2022/02/24/rust-non-binary-tree/) with source code available [here](https://gist.github.com/rust-play/b194d56e5dcd538d88dc4e490c39862b)



