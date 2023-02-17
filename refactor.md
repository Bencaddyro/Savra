Features :

- Pool de gagnant (limité par une valeur X)
- .dot state séparé du solve (map, market)
- offload node in file (serialization, indirection via global data strucuture (hybrid in memory / in file)
- prune "similar" node same location, same time, same cargo, same wallet ? relax la condition sur le time ? sur le wallet ? sur le cargo 
- post process reconstruction from action list, drop parent field (and persistent storage from children)



évolution strucuture:

Tas, ordonné sur le score (max ? min ?)
partagé en r/w avec tout les worket

Metastore -> hashmap uuid:Node
partagé en r/w avec tout les worker

Node -> immutable -> rajouter des uuid de children (pas de lien mémmoire direct of course) = recréer un node et l'insérer en place

State -> to define pour le market
payload -> immutable also, on remplace in placepar une new instance qui contient les bonnes valeurs

AStartNode (vertice)
Action (edge)

StateNode

    - Location -> enum
    - Wallet -> int
    - Time -> int
    - Dynamic Market -> struct
    - Cargo -> struct
    
    - Stateless Ship (for now) -> SCU, engine, thruster (QT & non QT, source from files)
    - Stateless Market (list pt de vente, list produit, quantité max par endroit
    - Stateless Map
  


Reflexion architecturation de code, qui est methode de quoi ?
function global, methode de struct ?

need un inventaire

Struct Action
    fn Display
    
fn global
    dot_state // print stateless data to file
    dot_tree -> a refactor total avec le drop de parent & des noeud intermédiare
    dot_tree_rec -> a refactor total avec drop parent & intermediaire node
    
    main // main, classico
    core_process // worker 

struct node
struct nodeata

struct state

struct payload
    fn add
    fn remove
    fn space
    fn empty
    fn display
    
TOUT ce qui est dans postprocess -> a re ecrire !
