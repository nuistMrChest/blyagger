pub fn geterate_title(title:String)->String{
    format!(
        "<div id=\"title-box\"><t2 id=\"sub-title\">{}</t2></div>",
        title
    )
}

pub fn set_br(txt:String)->String{
    format!(
        "{}<br>\n",
        txt
    )
}

pub fn put_img(blog_id:u64,img_id:u64)->String{
    format!(
        "<img src=\"img/{}/{}.png\">",
        blog_id,
        img_id
    )
}

pub fn set_code(code:String)->String{
    format!(
        "<pre class=\"code\"><code>\n{}\n</code></pre>",
        code
    )
}

pub fn set_txt(txt:String)->String{
    format!(
        "<div class=\"txt\">\n{}\n</div>",
        txt
    )
}

pub fn geterate_blog_li(title:String,blog_id:u64)->String{
    format!(
        "<li><a href=\"blogs/{}.html\">{}</a></li>",
        blog_id,
        title
    )
}

pub fn set_main(origin:String)->String{
    format!(
        "<div id=\"main\">\n{}\n</div>",
        origin
    )
}
