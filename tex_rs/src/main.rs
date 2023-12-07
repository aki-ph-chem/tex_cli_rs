//!
//! ## usega
//!
//!```
//! $ target/debug/tex_rs my_project -a aki -t my_tittle
//!```
use std::io::Error;
use tex_rs;

fn main() -> Result<(), Error> {
    let cmd_ops: tex_rs::CmdOpt = argh::from_env();

    if let Some(project) = &cmd_ops.project {
        println!("project name is  {}", project);
        // mkdir project_name/src
        tex_rs::mkdir::mkdir(&format!("{}/{}", project, "src"))?;

        let papaer = tex_rs::Paper::new(&cmd_ops);
        tex_rs::mkdir::write_string(
            &format!("{}/{}", &project, "Makefile"),
            &tex_rs::Paper::gen_makfile(),
        )?;

        tex_rs::mkdir::write_string(
            &format!("{}/{}", &project, "build.sh"),
            &tex_rs::Paper::gen_build_sh(),
        )?;

        tex_rs::mkdir::write_string(
            &format!("{}/{}", &project, "main.tex"),
            &papaer.gen_latex_template(),
        )?;
    } else {
        eprintln!("Error: No project name");
        std::process::exit(1);
    }

    Ok(())
}
