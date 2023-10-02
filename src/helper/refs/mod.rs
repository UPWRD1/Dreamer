pub struct Cmd<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub usage: &'a str,
    pub aliases: [&'a str; 4],
}
/*
impl Cmd<'_> {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_desc(&self) -> &str {
        self.desc
    }

    fn get_usage(&self) -> &str {
        self.usage
    }

    fn get_aliases(&self) -> [&str; 4] {
        self.aliases
    }
}
*/

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
