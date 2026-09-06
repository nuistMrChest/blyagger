#[derive(Clone)]
pub enum ArticleComponent{
    Text(String),
    Code(String),
    Img(String)
}

impl ArticleComponent{
    pub fn create_text(origin:String)->Self{
        let mut res="".to_string();
        let mut buffer="".to_string();
        for i in origin.chars(){
            if i=='\n'{
                res=format!(
                    "{}{}",
                    res,
                    crate::html_generator::set_br(buffer)
                );
                buffer="".to_string();
            }
            else{
                buffer.push(i);
            }
        }
        res=format!("{}{}",res,buffer);
        res=crate::html_generator::set_txt(res);
        Self::Text(res)
    }

    pub fn create_code(origin:String)->Self{
        Self::Code(crate::html_generator::set_code(origin))
    }

    pub fn create_img(blog_id:u64,img_id:u64)->Self{
        Self::Img(crate::html_generator::put_img(blog_id,img_id))
    }
}

#[derive(Clone)]
pub struct Article{
    pub title:String,
    pub components:Vec<ArticleComponent>
}

impl Article{
    pub fn new(origin:String,blog_id:u64)->Result<Self,String>{
        let mut title="".to_string();
        let mut components=Vec::<ArticleComponent>::new();
        let mut buffer="".to_string();
        let mut got_title=false;
        let mut img_id=1;
        let mut in_code=false;
        let mut escaping=false;
        for i in origin.chars(){
            if i=='\\'{
                if!escaping{
                    escaping=true;
                    continue;
                }
            }
            if escaping{
                match i{
                    '<'=>{
                        buffer=format!("{}&lt;",buffer);
                    },
                    '>'=>{
                        buffer=format!("{}&gt;",buffer);
                    },
                    '&'=>{
                        buffer=format!("{}&amp;",buffer);
                    },
                    '\"'=>{
                        buffer=format!("{}&quot;",buffer);
                    },
                    '\''=>{
                        buffer=format!("{}&apos;",buffer);
                    },
                    _=>buffer.push(i)
                };
            }
            else{
                buffer.push(i);
            }
            if i=='\n'{
                if !got_title{
                    title=buffer;
                    buffer="".to_string();
                }
                got_title=true;
            }
            if!escaping{
                if i=='@'{
                    buffer.pop();
                    components.push(
                        ArticleComponent::create_text(
                            buffer
                        )
                    );
                    buffer="".to_string();
                    components.push(
                        ArticleComponent::create_img(
                            blog_id,
                            img_id
                        )
                    );
                    img_id+=1;
                }
                if i=='<'{
                    if in_code{
                        return Err("bad code".to_string());
                    }
                    in_code=true;
                    buffer.pop();
                    components.push(
                        ArticleComponent::create_text(
                            buffer
                        )
                    );
                    buffer="".to_string();
                }
                if i=='>'{
                    if !in_code{
                        return Err("bad code".to_string());
                    }
                    in_code=false;
                    components.push(
                        ArticleComponent::create_code(
                            buffer.
                                chars().
                                skip(1).
                                take(buffer.chars().count()-2).
                                collect()
                        )
                    );
                    buffer="".to_string();
                }
            }
            escaping=false;
        }
        if in_code{
            return Err("code not closed".to_string());
        }
        components.push(ArticleComponent::create_text(buffer));
        if got_title{
            Ok(
                Self{
                    title:title,
                    components:components
                }
            )
        }
        else{
            Err("the article has no title".to_string())
        }
    }

    pub fn to_html(self)->String{
        let mut res="".to_string();
        res=format!("{}{}",res,crate::const_str::HEADER);
        res=format!(
            "{}{}",
            res,
            crate::html_generator::geterate_title(self.title)
        );
        res=format!("{}{}",res,crate::const_str::NAV);
        let mut main="".to_string();
        for i in self.components{
            match i{
                ArticleComponent::Text(text)=>{
                    main=format!("{}{}",main,text);
                },
                ArticleComponent::Code(code)=>{
                    main=format!("{}{}",main,code);
                },
                ArticleComponent::Img(img)=>{
                    main=format!("{}{}",main,img);
                },
            }
        }
        res=format!(
            "{}{}",
            res,
            crate::html_generator::set_main(main)
        );
        res=format!("{}{}",res,crate::const_str::FOOTER);
        res
    }
}
