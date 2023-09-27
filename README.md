# Unify 🔄
> Does what you want, not what you say.

Unify is a universal package and project management system for any language,
any build system, and any framework. Here's how:

```console
$ unify init myProject --python
    [i] Creating new project 'myProject' with python template...

$ unify add myTool
    [!] Tool 'myTool' has multiple versions!
    [?] Choose a version:
        1: myTool 6.3.2 (latest)
        2: myTool 5.4.0 
        3: myTool 1.0.0 
    ==> 1
    [i] Installing tool 'myTool' (6.3.2)...

$ unify archive myProject
    [i] Archive file 'myProject.ark' created!

$ unify build myProject
    [i] Build file 'myProject.uni.yaml' created!

# On another machine:
$ git clone myProject

$ unify
    [?] Project 'myProject' requires the following tools:
        - myTool
        - python
        Would you like to install these? (y/n)
    ==> y
    [i] Installing...

Hello World!

```
