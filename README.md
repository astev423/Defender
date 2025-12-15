Bevy notes:

-ECS design for data driven design, cache locality, etc

-fn move_players(mut q: Query<&mut Transform, With<Player>>), query all transforms, instead of querying all players with
transforms, that way you only get the exact transform component data you are looking for, rather than all player data

-Spawn bundles, which are 1-many components, each thing spawned is an entity

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

