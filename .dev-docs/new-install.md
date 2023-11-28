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
        ├── Dep.2.1.1
        └── Dep2.1.2


```


1. Load User Project
2. For each dep in User Project:
   1. Identify deps
   2. Download deps to ztemp.
   3. Add deps to tree
   4. Remove the ztemp folder
   5. Repeat 1-3 until latest download has no further dependancies
3. Flatten into new dep vector
4. remove duplicates
5. install all from new deps.