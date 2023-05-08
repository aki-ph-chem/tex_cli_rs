use argh::FromArgs;
pub mod mkdir;

#[derive(FromArgs)]
/// Struct to get authour name, title from command line option
pub struct CmdOpt {
    /// authour name
    #[argh(option, short = 'a' , default = "String::from(\"Authour\")")] 
    pub authour: String,

    /// title 
    #[argh(option, short = 't', default = "String::from(\"Title\")")] pub title: String,
}

/// Struct to hold author name and title
#[derive(Debug)]
pub struct Paper {
    /// authour name
    authour: String,

    /// tittle of paper
    title: String, 
}

impl Paper {
    /// constructor
    pub fn new(cmd_ops: CmdOpt) -> Paper {
        Paper {
            authour: cmd_ops.authour,
            title: cmd_ops.title,
        }
    }
}

