Features :

- .dot state séparé du solve (map, market)
- offload node in file (serialization, indirection via global data strucuture (hybrid in memory / in file)
- prune "similar" node same location, same time, same cargo, same wallet ? relax la condition sur le time ? sur le wallet ? sur le cargo 


évolution strucuture:

Tas, ordonné sur le score (max ? min ?)
partagé en r/w avec tout les worket

Metastore -> hashmap uuid:Node
partagé en r/w avec tout les worker

State -> to define pour le market
payload -> immutable also, on remplace in placepar une new instance qui contient les bonnes valeurs


StateNode

    - Location -> enum
    - Wallet -> int
    - Time -> int
    - Dynamic Market -> struct
    - Cargo -> struct
    
    - Stateless Ship (for now) -> SCU, engine, thruster (QT & non QT, source from files)
    - Stateless Market (list pt de vente, list produit, quantité max par endroit
    - Stateless Map
  

function global, methode de struct ?

Struct Action
    fn Display
    
fn global
    dot_state // print stateless data to file
    dot_tree -> a refactor total avec le drop de parent & des noeud intermédiare
    dot_tree_rec -> a refactor total avec drop parent & intermediaire node
    
    main // main, classico
    core_process // worker 

struct state

struct payload
    fn add
    fn remove
    fn space
    fn empty
    fn display
    
TOUT ce qui est dans postprocess -> a re ecrire !
