# Savra
**StarCitizen commodities trading tool**

## General

As many martingale provider flourish, like Versemate, Gallog, uexcorp, SC trade tools, here yet another "Best trade route" finders. With somes differences, no website, no fancy GUI, all source open, all data open. Up to you to test it on your computer ! (But do not ask me how to run it on MS windows :shrug:). I do not claim this tool is perfect, bug free or accurate ! Always double check what you plan. Still feel free to contribute to this tool if you want :hugs:.

### Current feature

- Able to load Universe data model
- Able to find best behavior from a starting location, initial invest and a time frame
- Output Universe data model as .dot file
- Output possiblities tree as .dot file (not recommended)

### Planned feature roadmap / improvement
(Because every SC project need a roadmap :grimacing:)

#### Adressing Time vs Memory 

Due to last update, the project need to reduce memory footprint for calculation, we will switch from in-node storage to action-based recalculation to get information for leaf node, keeping only action (the core of information modification). A time execution overhead will rise but hopefully it will be minor comparing to memory footprint reduction.

-[x] Done

Same paradigm will be employed for incoming "product flow" feature

#### Minor tech fix

- prep: simpler syntax for bi-directionnal location link
- prep: review all to clean String usage & memory 
- post: visu cargo ?
- post: width float & name & unit
- post: update arrow shape (action vs state)
- post: use time for state node position (use of subgraph & rank=same ? -> time slice)
- post: use location for state node position (geographical view, subgraph if same location)
- cleaner get_location with CONSTANT ?
- correct float overflow with manual tweak ?
- update heuristic: max profit per product -> impact locality attractiveness ?
- eval fn: clean cargo with map / sum
- multi: better insert child in queue (not 1 by 1 but by batch) ?
- node access hasmap: map[key] vs map.get(key).unwrap() syntax uniformaisation
- replace hashmap for location & product to static array

#### Futur work

- [ ] add product info (gas / metal / illegal)
- [ ] add location info (armistice / blue pad / hidden)
- [x] fine grained cargo for < UEC (aka < 100 unit)
- [ ] refactoring codebase with Ship profile (include cargo, fuel consumption, speed, travel time, ability to land)
- [ ] handle max flow for buy/sell
- [ ] introduce action wait in children generation
- [ ] handle fuel cost
- [ ] handle price variability from current agent
- [ ] introduce fine graine amount choice in children generation
- [ ] comparison of A* pruning method over Monte Carlo variant

#### Futur futur work

- [ ] add danger factor (pirate, escort, 30k)
- [ ] support celestial movement for accurate distance & time & ephemeris
- [ ] support same-travel delivery quest ?
- [ ] Fancy GUI ?
- [ ] clean .dot output & auto build with dot librabry ?

## Howto

### Install

1. Clone the repo
2. (Optional) Adjust data model
3. Run !

### Data model

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

### Options

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

### Resolution principle

Trader behavior are described with 4 base actions:
- Move
- Buy
- Sell
- Wait

State node are composed of a location, a wallet, a cargo, a time delta from run beginning.
Path exploration algorithm used is __A*__ with a custom heuristic.


