# A terminal-based snake game written in Rust

A little project I thought would be fun to work on. No online snake tutorials were used, so the code design choices are authentically mine!

![image](https://github.com/jack3898/rust-text-snake/assets/28375223/a76472d7-e8e0-4be8-bf35-498688c84b92)

# How to play

Use the arrow keys to change the trajectory of the snake. Eat apples to increase the length of the snake, whilst making sure to not collide with the map wall or yourself!

# Planned features

Powerups, to spice the game up a bit. Some powerup ideas I have:
_most powerups are temporary and expire_

- Map wrap (temporary); if you hit a wall, you automatically wrap around to the other side of the map.
- Slowdown (temporary); you move much slower, but can fast-forward by pressing the arrow key that corresponds to your direction. This will give you more control over the snake.
- Supersnake (temporary); the snake goes blue, which means you can travel through yourself without eating yourself.
- Map grow (permanent); The map will increase its size by 1 giving you more space to move around. There will be a maximum size for the map, but I haven't decided how that will work.
