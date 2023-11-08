# Dreamer 💤
> Does what you want, not what you say.

Dreamer is a universal tool management system for any language,
any build system, and any framework. 

## Here's how:

* Work with your favorite tools without wherever, whenever:
```console
$ zzz add myTool
[i] Installing tool 'myTool' (6.3.2)...
```

* New computer? No problem:
```console
$ git clone myProject.git

$ zzz start myProject
[?] 'myProject' requires the following tools:
        - myTool
        - otherTool
    Would you like to install these? (y/n)
    ==> y
[i] Installing...

Hello World!

```

* Packages? More like Git repositories:
  ```console
    $ zzz add myPackage.git
      [i] Cloning 'myPackage.git'...
      [i] Fetch hook found. Building...
      [i] Cleaning up...
      [i] 'myPackage' added!
      
  ```
