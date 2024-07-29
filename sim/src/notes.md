
# Sim Notes

## Manipulating the world
Theres two ways you could do it, either use operations that manipulate the world itself via commands (https://bevy-cheatbook.github.io/programming/world.html) or you can get full direct access to the world which gives you more control (https://bevy-cheatbook.github.io/programming/ec.html). See the second docs for more details. I don't know if it is better to go that route or try using commands.

More discussion on world access vs doing it through commands
- https://github.com/bevyengine/bevy/discussions/3332#discussioncomment-7418611

## The whole issue of having a link that has two nodes but nodes can be shared between multiple links
- https://gamedev.stackexchange.com/questions/204007/in-bevy-ecs-what-is-a-good-way-to-have-entities-reference-each-other


## Misc
Hmm this spawn empty is also interesting
https://docs.rs/bevy_ecs/latest/bevy_ecs/world/struct.World.html#method.spawn_empty



# Extension cloth sim
it would be great to define the x y z and direction of how many nodes you want. 
that way, should be really easy to directly make a cloth simulation as well? 
You will need to implement gravity and also the anchoring of certain nodes as well.