// first line: title
// @: picture
// < >:code
// \:cancel

mod const_str;
mod html_generator;
mod core_logic;
mod repo;

use std::fs;
use std::env;

fn main(){
    if env::args().len()<2{
        panic!("no input directory");
    }
    if env::args().len()>2{
        panic!("too many arguments");
    }
    let origin_path:String=env::args().nth(1).unwrap();
    let origin;
    match fs::read_to_string(format!("{}/origin.txt",origin_path.clone())){
        Ok(o)=>
            origin=o,
        Err(_)=>
            panic!("failed reading text")
    }
    let repo_path;
    match fs::read_to_string("/home/mrchest/.blyagger"){
        Ok(rp)=>
            repo_path=rp,
        Err(_)=>
            panic!("failed reading text")
    }
    let blog_list;
    match fs::read_to_string(format!("{}/blog_list.html",repo_path.trim())){
        Ok(bl)=>
            blog_list=bl,
        Err(_)=>
            panic!("failed reading text")
    }
    let blog_id;
    match crate::repo::read_blog_count(blog_list.clone()){
        Ok(bi)=>blog_id=bi+1,
        Err(_)=>panic!("failed in reading blog id")
    }
    let article;
    match crate::core_logic::Article::new(origin,blog_id){
        Ok(a)=>article=a,
        Err(e)=>panic!("{}",e)
    }
    match fs::write(
        format!(
            "{}/blogs/{}.html",
            repo_path.trim(),
            blog_id
        ),
        article.clone().to_html()
    ){
        Err(e)=>panic!("failed to write html:{}",e),
        _=>()
    }
    let updated_blog_list;
    match crate::repo::update_blog_list(blog_list,blog_id,article.clone().title){
        Ok(ubl)=>updated_blog_list=ubl,
        Err(e)=>panic!("{}",e)
    }
    match fs::write(
        format!(
            "{}/blog_list.html",
            repo_path.trim()
        ),
        updated_blog_list
    ){
        Err(e)=>panic!("failed to write blog_list:{}",e),
        _=>()
    }
    if matches!(
        std::process::Command::new("cp").
            arg("-r").
            arg(format!("{}/img",origin_path)).
            arg(format!("{}/blogs/img/{}",repo_path.trim(),blog_id)).
            status(),
        Err(_)
    ){
        panic!("failed to copy images");
    }
    if!crate::repo::upload(blog_id,repo_path.trim().to_string()){
        panic!("failed to upload");
    }
}
