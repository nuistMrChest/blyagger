pub fn read_blog_count(origin:String)->Result<u64,String>{
    let mut buffer="".to_string();
    for i in 4..(origin.len()){
        let c:char=origin.chars().nth(i).ok_or("bad blog.html".to_string())?;
        if!c.is_ascii_digit(){
            break;
        }
        buffer.push(c);
    };
    match buffer.parse::<u64>(){
        Ok(n)=>Ok(n),
        Err(_)=>Err("unable to read count".to_string())
    }
}

pub fn update_blog_list(origin:String,blog_id:u64,title:String)->Result<String,String>{
    let res;
    let mut i=4;
    let mut c:char=origin.chars().nth(i).ok_or("bad blog.html".to_string())?;
    while c.is_ascii_digit(){
        i+=1;
        c=origin.chars().nth(i).ok_or("bad blog.html".to_string())?;
    }
    res=format!(
        "<!--{}{}{}",
        blog_id,
        origin.chars().skip(i).collect::<String>(),
        crate::html_generator::geterate_blog_li(title,blog_id)
    );
    Ok(res)
}

pub fn upload(blog_id:u64,repo_path:String)->bool{
    if matches!(std::process::Command::new("git").arg("add").arg(".").current_dir(repo_path.clone()).status(),Err(_)){
        return false;
    }
    if matches!(std::process::Command::new("git").arg("commit").arg("-m").arg(format!("added blog {}",blog_id)).current_dir(repo_path.clone()).status(),Err(_)){
        return false;
    }
    if matches!(std::process::Command::new("git").arg("push").current_dir(repo_path.clone()).status(),Err(_)){
        return false;
    }
    true
}
