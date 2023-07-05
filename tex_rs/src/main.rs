//!
//! ## usega
//!
//!```
//! $ target/debug/tex_rs my_project -a aki -t my_tittle 
//!```
use tex_rs;
use std::io::Error;

fn main() -> Result<(),Error>{
    let cmd_ops: tex_rs::CmdOpt = argh::from_env();

    if let Some(project) = &cmd_ops.project {
        println!("project name is  {}", project);
        // mkdir project_name/src
        tex_rs::mkdir::mkdir(&format!("{}/{}",project,"src"))?; 
    } else {
        eprintln!("Error: No project name");
        std::process::exit(1);
    }

    let papaer = tex_rs::Paper::new(&cmd_ops);
    tex_rs::mkdir::write_string(&format!("{}/{}",&cmd_ops.project.clone().unwrap(),"Makefile")
                                ,&tex_rs::Paper::gen_makfile())?;
    tex_rs::mkdir::write_string(&format!("{}/{}",&cmd_ops.project.clone().unwrap(),"main.tex")
                                ,&papaer.gen_latex_template())?;

    Ok(())
}
