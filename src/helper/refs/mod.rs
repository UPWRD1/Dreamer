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

pub const HELPCMD: Cmd = Cmd {
    name: "help",
    desc: "This command",
    longdesc: "Provides help for other commands",
    usage: "help [command]",
    aliases: ["help", "-h"],
};

pub const NEWCMD: Cmd = Cmd {
    name: "new",
    desc: "Creates a new dreamfile",
    longdesc: "Creates a new dreamfile from <filename>. If no filename is provided a wizard will launch to create one.",
    usage: "new <filename>",
    aliases: ["new", "n"],
};

pub const STARTCMD: Cmd = Cmd {
    name: "start",
    desc: "Starts dreaming a dreamfile",
    longdesc: "Starts and grabs the dependancies found in a dreamfile. If no filename is provided, zzz will prompt for one.",
    usage: "start [filename]",
    aliases: ["start", "s"],
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

pub const REMOVECMD: Cmd = Cmd {
    name: "remove",
    desc: "Removes a dependancy from a dreamfile",
    longdesc: "Removes a dependancy from a dreamfile provide by <filename>.",
    usage: "remove <filename>",
    aliases: ["remove", "rm"],
};

pub const FORGETCMD: Cmd = Cmd {
    name: "forget",
    desc: "Removes dream binaries",
    longdesc:
        "Removes the directory containing dream binaries given a dreamfile provided by <filename>.",
    usage: "forget <filename>",
    aliases: ["forget", "f"],
};

///DEPRECATED
pub const EXTCMD: Cmd = Cmd {
    name: "ext",
    desc: "Runs an extension",
    longdesc: "Runs an extension. Extensions are found in '$HOME/.zzz/ext'. !If arguments are missing, a wizard will launch to choose one.",
    usage: "ext <extension> [arguments]",
    aliases: ["ext", "@"],
};

///DEPRECATED
pub const RUNCMD: Cmd = Cmd {
    name: "run",
    desc: "Executes a dreamfile",
    longdesc: "Runs the content in the dreamfile provide by <filename>.",
    usage: "run <filename>",
    aliases: ["run", "r"],
};

///DEPRECATED
pub const GRABCMD: Cmd = Cmd {
    name: "grab",
    desc: "Grabs the dependancies of a dreamfile",
    longdesc: "Grabs the dependancies found in a dreamfile. If no filename is provided, zzz will prompt for one.",
    usage: "grab [filename]",
    aliases: ["grab", "g"],
};

pub const AVAILABLE_CMDS: [&Cmd; 8] = [
    &HELPCMD, &STARTCMD, &NEWCMD, &LISTCMD, &ADDCMD, &EXTCMD, &REMOVECMD, &GRABCMD,
];

pub const COMMON_CMDS: [&Cmd; 5] = [&HELPCMD, &NEWCMD, &STARTCMD, &ADDCMD, &REMOVECMD];
