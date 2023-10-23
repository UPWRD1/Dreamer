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
    pub aliases: [&'a str; 2],
}

pub const RUNCMD: Cmd = Cmd {
    name: "run",
    desc: "Executes a .zzz.yaml file",
    longdesc: "Runs the content in the .zzz.yaml file provide by <filename>.",
    usage: "run <filename>",
    aliases: ["run", "r"],
};

pub const HELPCMD: Cmd = Cmd {
    name: "help",
    desc: "This command",
    longdesc: "Provides help for other commands",
    usage: "help [command]",
    aliases: ["help", "-h"],
};

pub const NEWCMD: Cmd = Cmd {
    name: "new",
    desc: "Creates a new .zzz.yaml file",
    longdesc: "Creates a new .zzz.yaml file from <filename>. If no filename is provided a wizard will launch to create one.",
    usage: "new <filename>",
    aliases: ["new", "n"],
};

pub const LOADCMD: Cmd = Cmd {
    name: "load",
    desc: "Load a .zzz.yaml file",
    longdesc: "Loads and grabs the dependancies found in a dreamfile. If no filename is provided, zzz will prompt for one.",
    usage: "load [filename]",
    aliases: ["load", "l"],
};

pub const LISTCMD: Cmd = Cmd {
    name: "list",
    desc: "Lists all dependancies in a dreamfile",
    longdesc: "Lists all dependancies in a dreamfile. If no filename is provided, zzz will prompt for one.",
    usage: "list [filename]",
    aliases: ["list", "L"],
};

pub const ADDCMD: Cmd = Cmd {
    name: "add",
    desc: "Adds a dependancy to a dreamfile",
    longdesc: "Adds a dependancy to a dreamfile. If arguments are missing, a wizard will launch to choose one.",
    usage: "add <dependancy> <filename>",
    aliases: ["add", "a"],
};

pub const EXTCMD: Cmd = Cmd {
    name: "ext",
    desc: "Runs an extension",
    longdesc: "Runs an extension. Extensions are found in '$HOME/.zzz/ext'. !If arguments are missing, a wizard will launch to choose one.",
    usage: "ext <extension> [arguments]",
    aliases: ["ext", "@"],
};

pub const REMOVECMD: Cmd = Cmd {
    name: "remove",
    desc: "Removes a dependancy from a .zzz.yaml file",
    longdesc: "Removes a dependancy from a .zzz.yaml file provide by <filename>.",
    usage: "remove <filename>",
    aliases: ["remove", "rm"],
};

pub const AVAILABLE_CMDS: [&Cmd; 8] = [
    &HELPCMD, &LOADCMD, &RUNCMD, &NEWCMD, &LISTCMD, &ADDCMD, &EXTCMD, &REMOVECMD,
];
