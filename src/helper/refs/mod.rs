pub struct Cmd<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub usage: &'a str,
    pub aliases: [&'a str; 4],
}

pub const RUNCMD: Cmd = Cmd {
    name: "run",
    desc: "Executes a .uni.yaml file",
    usage: "run <filename>",
    aliases: ["run", "r", "--run", "-r"],
};

pub const HELPCMD: Cmd = Cmd {
    name: "help",
    desc: "This command",
    usage: "help",
    aliases: ["help", "h", "--help", "-h"],
};

pub const INITCMD: Cmd = Cmd {
    name: "init",
    desc: "Creates a new .uni.yaml file",
    usage: "init <filename>",
    aliases: ["init", "i", "--init", "-i"],
};

pub const LOADCMD: Cmd = Cmd {
    name: "load",
    desc: "Load a .uni.yaml file",
    usage: "load [filename]",
    aliases: ["load", "l", "--load", "-l"],
};