/// Definition of the Cmd type, and constant values for the commands.

pub trait EntryFunc {
    fn go() -> Result<(), ()>;
}

pub struct Cmd<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub longdesc: &'a str,
    pub usage: &'a str,
    pub aliases: [&'a str; 4],
}

pub const RUNCMD: Cmd = Cmd {
    name: "run",
    desc: "Executes a .uni.yaml file",
    longdesc: "Runs the content in the .uni.yaml file provide by <filename>.",
    usage: "run <filename>",
    aliases: ["run", "r", "--run", "-r"],
};

pub const HELPCMD: Cmd = Cmd {
    name: "help",
    desc: "This command",
    longdesc: "Provides help for other commands",
    usage: "help [command]",
    aliases: ["help", "h", "--help", "-h"],
};

pub const INITCMD: Cmd = Cmd {
    name: "init",
    desc: "Creates a new .uni.yaml file",
    longdesc: "Creates a new .uni.yaml file from <filename> If no filename is provided a wizard will launch to create one.",
    usage: "init <filename>",
    aliases: ["init", "i", "--init", "-i"],
};

pub const LOADCMD: Cmd = Cmd {
    name: "load",
    desc: "Load a .uni.yaml file",
    longdesc: "Loads and grabs the dependancies found in a unifile. If no filename is provided, unify will search the current directory for a unifile.",
    usage: "load [filename]",
    aliases: ["load", "l", "--load", "-l"],
};

pub const LISTCMD: Cmd = Cmd {
    name: "list",
    desc: "Lists all dependancies in a unifile",
    longdesc: "Lists all dependancies in a unifile. If no filename is provided, unify will search the current directory for a unifile.",
    usage: "list [filename]",
    aliases: ["list", "L", "--list", "-L"],
};

pub const ADDCMD: Cmd = Cmd {
    name: "add",
    desc: "Adds a dependancy to a unifile",
    longdesc: "Adds a dependancy to a unifile. If no filename is provided, unify will search the current directory for a unifile.",
    usage: "add <dependancy> [filename]",
    aliases: ["add", "a", "--add", "-a"],
};

pub const AVAILABLE_CMDS: [&Cmd; 6] = [&HELPCMD, &LOADCMD, &RUNCMD, &INITCMD, &LISTCMD, &ADDCMD];
