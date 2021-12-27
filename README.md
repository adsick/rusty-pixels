# Rusty Pixels

Idk what I'm doing, but maybe it will become a game

![image](https://user-images.githubusercontent.com/24528839/147421329-1b6efe9d-a725-4fcc-bb33-7c5359ec9b3a.png)


## About

This is powered by [pixels crate](https://github.com/parasyte/pixels) + winit and others, I'm planning integrating it with Bevy/Bevy ECS to see what's possible in terms of performance and efficiency.

![image](https://user-images.githubusercontent.com/24528839/147421333-8d7bc31b-458b-446e-bd63-24f83cd17a21.png)


## How it works
I allocate a lot of 'particles' which have properties like position, velocity, color, life etc. then simple brown-like motion comes into place and voila...
Everything is happening on cpu (now on one thread, in the future we will see how that could be improved) on decay effect I made use of rayon crate which speed up that part x3 times or even more.

![image](https://user-images.githubusercontent.com/24528839/147512605-e5d025c3-ce7c-4cd5-b32a-78bb65010943.png)


## 640 by 360
This is 1920x1080 divided by 3, when fullscreen it makes perfect 3 by 3 pixels

## Why the colors are so great
I don't know myself yet, somehow this algorithm yeilds perfect match autumn colors which wasn't what I expected tbh.
Interesting task will be trying to recreate all 4 seasons. 
