# Unify Ideas:

## Functionality:
    1. A user gets a git repo which uses a tool they don't have 
    2. They run 'unify' in terminal
    3. Unify searches for a unifile in the directory.
    4. Parses the yaml for the needed tools.
    5. In ~/unify creates a folder with the unique project id hash as a name
    6. It finds the correct tools and packages and puts the binaries and libs in the folder
    7. It launches a custom shell which uses those things (wait this is nix)