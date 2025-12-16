Bevy notes:

-ECS design for data driven design, cache locality, etc. Components like columns in DB, each row is an
entity. Entities can have all, none, or some of the columns. None = marker entity

-Query (transform, player) to get both transform and player if you need them both, but if you just
need transform for player, then use With<Player> to make query more efficient and have unused variable.
Kind of like querying in SQL


-Spawn bundles, which are 1-many components, each thing spawned is an entity

-Put components together in a #[derive(bundle)] to make like a class, or put very tightly connected things together
in a component like Transform struct

-Resources are for global shared things

-Components are where data for entity is stored like position, size

-Query for one component or multiple components (tuple), you can filter components with With<>

-Plugins can connect multiple systems together for encapsulation

-Window is for information on window like mouse position, resizability, resolution, etc

-Change transform to move things and set spawn location

-Use delta to keep movement and other things consistent

Wave based tower defense game. Protect the object in the middle. Monsters will try to destroy it
and your defenses. Use different buildings and walls to protect the center. You only have a limited
amount of money, so build carefully. Certain towers work better against monsters than other towers
