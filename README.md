# Unify 🔄
> Does what you want, not what you say.

Unify is a universal package and project management system for any language,
any build system, and any framework. Here's how:

```
$ unify new myProject
    [i] Creating new project 'myProject'...

$ unify add myPack
    [!] Package 'myPack' has multiple versions!
    [q] Choose a version:
        1: myPack 6.3.2 (latest)
        2: myPack 5.4.0 
        3: myPack 1.0.0 
    ==> 1
    [i] Installing packager 'myPack' (6.3.2)...

$ unify archive myProject
    [i] Archive file 'myProject.ark' created!

$ unify run
    [i] Running 'myProject.uni.yaml'

Hello World!

    [i] All tasks completed.
```
