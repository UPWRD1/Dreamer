/// Defnew
//ion of the Cmd type, and constant values for the commands.

pub trait EntryFunc {
    fn go() -> Result<(), ()>;
}

pub struct Cmd<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub longdesc: &'a str,
    pub usage: &'a str,
    pub aliases: [&'a str; 3],
}

pub const RUNCMD: Cmd = Cmd {
    name: "run",
    desc: "Executes a .uni.yaml file",
    longdesc: "Runs the content in the .uni.yaml file provide by <filename>.",
    usage: "run <filename>",
    aliases: ["run", "r", "--run"],
};

pub const HELPCMD: Cmd = Cmd {
    name: "help",
    desc: "This command",
    longdesc: "Provides help for other commands",
    usage: "help [command]",
    aliases: ["help", "-h", "--help"],
};

pub const NEWCMD: Cmd = Cmd {
    name: "new",
    desc: "Creates a new .uni.yaml file",
    longdesc: "Creates a new .uni.yaml file from <filename>. If no filename is provided a wizard will launch to create one.",
    usage: "new <filename>",
    aliases: ["new", "n", "--new"],
};

pub const LOADCMD: Cmd = Cmd {
    name: "get",
    desc: "Load a .uni.yaml file",
    longdesc: "Loads and grabs the dependancies found in a unifile. If no filename is provided, unify will prompt for one.",
    usage: "load [filename]",
    aliases: ["load", "l", "--load"],
};

pub const LISTCMD: Cmd = Cmd {
    name: "list",
    desc: "Lists all dependancies in a unifile",
    longdesc: "Lists all dependancies in a unifile. If no filename is provided, unify will prompt for one.",
    usage: "list [filename]",
    aliases: ["list", "L", "--list"],
};

pub const ADDCMD: Cmd = Cmd {
    name: "add",
    desc: "Adds a dependancy to a unifile",
    longdesc: "Adds a dependancy to a unifile. If arguments are missing, a wizard will launch to choose one.",
    usage: "add <dependancy> <filename>",
    aliases: ["add", "a", "--add"],
};

pub const EXTCMD: Cmd = Cmd {
    name: "ext",
    desc: "Runs an extension",
    longdesc: "Runs an extension. Extensions are found in '$HOME/.unify/ext'. !If arguments are missing, a wizard will launch to choose one.",
    usage: "ext <extension> [arguments]",
    aliases: ["ext", "@", "--ext"],
};


pub const AVAILABLE_CMDS: [&Cmd; 7] = [&HELPCMD, &LOADCMD, &RUNCMD, &NEWCMD, &LISTCMD, &ADDCMD, &EXTCMD];
