use tex_rs;

fn main(){
    let cmd_ops: tex_rs::CmdOpt = argh::from_env();
    let papaer = tex_rs::Paper::new(cmd_ops);
}
