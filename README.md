# Unify 🔄
> Does what you want, not what you say.

Unify is a universal tool management system for any language,
any build system, and any framework. 

## Here's how:

* Lightning Fast project creation:
```console
$ unify new myProject
    [i] Creating new project 'myProject' 
    [i] Done!
```

* Work with your favorite tools without version conflicts:
```console
$ unify add myTool
    [!] Tool 'myTool' has multiple versions!
    [?] Choose a version:
        1: myTool 6.3.2 (latest)
        2: myTool 5.4.0 
        3: myTool 1.0.0 
    ==> 1
    [i] Installing tool 'myTool' (6.3.2)...
```

* New computer? No problem:
```console
$ git clone myProject

$ unify
    [?] Project 'myProject' requires the following tools:
        - myTool
        - otherTool
        Would you like to install these? (y/n)
    ==> y
    [i] Installing...

Hello World!

```

* Extensions? Just a rust project.
  ```console
  $ unify install https://github.com/UPWRD1/unify.git
  [i] Fetching...
  [i] Building...
  [i] Done
  ```
