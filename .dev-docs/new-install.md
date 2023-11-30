# Install process:

Lets say you have the following structure:

```
User Project
├── Dep1
│   ├── Dep1.1
│   │   └── Dep1.1.1
│   └── Dep1.2
└── Dep2
    └── Dep2.1
        ├── Dep.1.1.1
        └── Dep2.1.2
```


1. Load User Project
2. For each dep in User Project:
   1. Identify subdeps
   2. Download subdeps to ztemp.
   3. Add subdeps to tree
   4. Remove the ztemp folder
   5. Repeat 1-3 until latest download has no further dependancies
3. Flatten into new dep vector
4. remove duplicates
5. install all from new deps.


Or...

1. Load User Project
2. For each dep in User Project:
   1. Check cache for dep
   2. If it exists:
      1. Append to needed_Dep Vector
   3. Else:
      1. Complain
3. Remove duplicates from Needed dep vector
4. install.