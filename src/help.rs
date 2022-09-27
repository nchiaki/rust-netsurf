use std::path::Path;
use std::ffi::OsStr;
use std::sync::Mutex;

use url::Url;
use once_cell::sync::{OnceCell, Lazy};

#[derive(Debug)]
pub struct UrlParts {
    scheme: String,
    host: String,
    path: String,
    port: i32,
}
impl UrlParts {
    fn new (/*scheme: &str,host: String,path: String,port: i32*/) -> Self
    {
        let scheme = "".to_string();
        let host = "".to_string();
        let path = "".to_string();
        let port : i32 = -1;
        UrlParts{scheme,host,path,port}
    }

    pub fn to_url(&self) -> String
    {
        if self.port < 0
        {format!("{}://{}{}", self.scheme, self.host, self.path)}
        else
        {format!("{}://{}:{}{}", self.scheme, self.host, self.port, self.path)}
    }

    pub fn get_scheme(&self) -> String
    {
        format!("{}", self.scheme)
    }

}

pub struct ParamMap {
    stp_utf8 : bool,
    stp_utf8dump : bool,
    stp_builder : bool,
    log_pathparse : bool,
    data_save : bool,
    data_filename: String,
    ignr_list : Vec<String>,
}
impl ParamMap {
    pub fn new () -> Self
    {
        ParamMap {
            stp_utf8:false,
            stp_utf8dump:false,
            stp_builder:false,
            log_pathparse:false,
            data_save:false,
            data_filename: "".to_string(),
            ignr_list:Vec::new(),
        }
    }

    pub fn copy (&self) -> Self
    {
        ParamMap{
            stp_utf8: self.stp_utf8,
            stp_utf8dump: self.stp_utf8dump,
            stp_builder: self.stp_builder,
            log_pathparse: self.log_pathparse,
            data_save: self.data_save,
            data_filename: self.data_filename.clone(),
            ignr_list: self.ignr_list.clone(),
        }
    }

    pub fn set_stop_utf8(&mut self)
    {self.stp_utf8 = true;}
    pub fn is_stop_utf8(&self) -> bool
    {self.stp_utf8}
    pub fn set_stop_utf8dump(&mut self)
    {self.stp_utf8 = true;self.stp_utf8dump = true;}
    pub fn is_stop_utf8dump(&self) -> bool
    {self.stp_utf8dump}
    pub fn set_stop_builder(&mut self)
    {self.stp_builder = true;}
    pub fn is_stop_builder(&self) -> bool
    {self.stp_builder}

    pub fn set_log_pathparse(&mut self)
    {self.log_pathparse = true;}
    pub fn is_log_pathparse(&self) -> bool
    {self.log_pathparse}

    pub fn set_data_save(&mut self)
    {self.data_save = true;}
    pub fn is_data_save(&self) -> bool
    {self.data_save}
    pub fn put_data_filename(&mut self, fname:String)
    {self.data_filename = fname;}
    pub fn get_data_filename(&mut self) -> String
    {self.data_filename.clone()}

    pub fn push_ignor(&mut self, kywd:String)
    {self.ignr_list.push(kywd);}
    pub fn is_ignor(&mut self, url:String) -> bool
    {
        if self.ignr_list.iter().any(|e| None != url.find(e))
        {
            println!("Ignor : {}", url);
            return true;
        }
        else
        {return false;}
    }
}

const DEF_SHOWCONTENT : bool = false;
const DEF_RSPNDATHNRQ : bool = false;
const DEF_NESTLEVEL : i32 = 16;
const DEF_STORAGEDIR : &str = ".";
const DEF_INDENT : i32 = 2;

static IAM : OnceCell<String> = OnceCell::new();
static URLPARTS : OnceCell<UrlParts> = OnceCell::new();
static SHOWCONTENT : OnceCell<bool> = OnceCell::new();
static RSPNDATHNRQ : OnceCell<bool> = OnceCell::new();
static STORAGEDIR : OnceCell<String> = OnceCell::new();
pub static mut EXTIDLIST : Vec<String> = Vec::new();
static mut NESTLEVEL : i32 = DEF_NESTLEVEL;
static mut INDENT : i32 = DEF_INDENT;


fn set_iam(path:&String) -> bool
{
    let cmdnm = match Path::new(path).file_name()
    {
        Some(v) => v,
        None => {
            OsStr::new("Bye bye ...");
            return false;
        },
    };
    let _iam = match cmdnm.to_str()
    {
        Some(v) => crate::help::IAM.set(v.to_string()).unwrap(),
        None => todo!(),
    };

    // ネットサーフ用に拡張子 html/shtml は予め登録しておく
    unsafe {crate::help::EXTIDLIST.push("html".to_string());}
    unsafe {crate::help::EXTIDLIST.push("shtml".to_string());}

    true
}

pub fn get_strturl() -> &'static UrlParts
{
    match crate::help::URLPARTS.get()
    {
        Some(v) => v,
        None => todo!(),
    }
}

/***
pub fn get_extid() -> Vec<String>
{
    &'static crate::help::EXTIDLIST
}
***/
pub fn get_indent() -> i32
{
    unsafe{crate::help::INDENT}
}

pub fn is_content_display_enable() -> bool
{
    match crate::help::SHOWCONTENT.get()
    {
        Some(v) => *v,
        None => DEF_SHOWCONTENT,
    }
}

pub fn get_data_dir() -> String
{
    match crate::help::STORAGEDIR.get()
    {
        Some(v) => (&v).to_string(),
        None => DEF_STORAGEDIR.to_string(),
    }
}

pub fn usage()
{
    let iam = match crate::help::IAM.get()
    {
        Some(v) => v,
        None => todo!(),
    };
    println!("{} [-h|--help] [-d] [-a] {{[-r <NestLevel>] <StrURL>}} [<extId> ... [-storage <Dir>] [-stop <StopAttr>] [-ignor <KeyWord>] [-log <LogAttr>]]", iam);
    println!("\t-d : 読み出しているコンテンツソースを表示します。Def.off");
    println!("\t-a : 読み出そうとするコンテンツから認証を求められた時、そのコンテンツに該当するID/Passwordを尋ねます。");
    println!("\t     Def.該当のコンテンツは認証エラーとして読み飛ばします。");
    println!("\t-r <NestLevel> : <StrURL> 内より hrefに記述されているURLを<NestLevel>段階まで辿っていきます。Def.4");
    println!("\t<StrURL> : 最初に読み出すコンテンツURLを指定します。");
    println!("\t<extId> : コンテンツ内から読出すデータの拡張子を指定します。");
    println!("\t-storage <Dir> : <extID>で指定されたデータの読出し先ディレクトリを指定します。");
    println!("\t                 Def.カレントディレクトリ");
    println!("\t-stop <StopAttr> : <StopAttr>で指定したエラーが発生したときに停止します。");
    println!("\t      <StopAttr> : [utf8 | utf8dump] | builder");
    println!("\t-ignor <KeyWord> : <KeyWord>が含まれるURLは参照しません。");
    println!("\t       <KeyWord> : ascii文字");
    println!("\t-log <LogAttr>   : <LogAttr>で示されるログを出力します。");
    println!("\t     <LogAttr> : pathparse");
}

pub fn parse_argv(argc:usize, argv:Vec<String>, mut prmtbl: ParamMap) -> Result <ParamMap, bool>
{
    println!("[{}]{:?}", argc, argv);

    if set_iam(&argv[0]) == false
    {return Err(false);}

    if argc < 2
    {
        usage();
        Err(false)
    }
    else
    {
        let mut ax = 1;
        while ax < argc
        {
            let mut args = &argv[ax];
            if args == "--help" || args == "-h"
            {
                usage();
                return Err(false);
            }
            else if args == "-d"
            {crate::help::SHOWCONTENT.set(true).unwrap();}
            else if args == "-a"
            {crate::help::RSPNDATHNRQ.set(true).unwrap();}
            else if args == "-r"
            {
                ax += 1;
                if argc <= ax
                {
                    println!("<NestLevel>が指定されていません");
                    usage();
                    return Err(false);
                }
                args = &argv[ax];
                unsafe
                {
                    crate::help::NESTLEVEL = match args.parse::<i32>()
                    {
                        Ok(v) => v,
                        Err(_) => {
                            println!("<NestLevel>は数値を指定してください");
                            usage();
                            return Err(false);
                        },
                    };
                }
            }
            else if args == "-storage"
            {
                ax += 1;
                if argc <= ax
                {
                    println!("<Dir>が指定されていません");
                    usage();
                    return Err(false);
                }
                prmtbl.set_data_save();
                args = &argv[ax];
                crate::help::STORAGEDIR.set(args.to_string()).unwrap();
                //println!("storage:{:?}", crate::help::STORAGEDIR);
            }
            else if args == "-stop"
            {
                ax += 1;
                if argc <= ax
                {
                    println!("stop要因が指定されていません");
                    usage();
                    return Err(false);
                }
                loop
                {
                    if argc <= ax
                    {// パラメータ指定は終了
                        ax -= 1;
                        break;
                    }
                    args = &argv[ax];
                    if args.starts_with("-")
                    {// オプション指定が有るときはオプション解釈を続ける
                        ax -= 1;
                        break;
                    }
                    else if args == "utf8"
                    {prmtbl.set_stop_utf8();}
                    else if args == "utf8dump"
                    {prmtbl.set_stop_utf8dump();}
                    else if args == "builder"
                    {prmtbl.set_stop_builder();}
                    else
                    {
                        println!("正しいstop要因が指定されていません");
                        usage();
                        return Err(false);
                    }
                    ax += 1;
                }
            }
            else if args == "-log"
            {
                ax += 1;
                if argc <= ax
                {
                    println!("log要因が指定されていません");
                    usage();
                    return Err(false);
                }
                loop
                {
                    if argc <= ax
                    {// パラメータ指定は終了
                        ax -= 1;
                        break;
                    }
                    args = &argv[ax];
                    if args.starts_with("-")
                    {// オプション指定が有るときはオプション解釈を続ける
                        ax -= 1;
                        break;
                    }
                    else if args == "pathparse"
                    {prmtbl.set_log_pathparse();}
                    else
                    {
                        println!("正しいlog要因が指定されていません");
                        usage();
                        return Err(false);
                    }
                    ax += 1;
                }
            }
            else if args == "-ignor"
            {
                ax += 1;
                if argc <= ax
                {
                    println!("ignorするキーワードが指定されていません");
                    usage();
                    return Err(false);
                }
                loop
                {
                    if argc <= ax
                    {// パラメータ指定は終了
                        ax -= 1;
                        break;
                    }
                    args = &argv[ax];
                    if args.starts_with("-")
                    {// オプション指定が有るときはオプション解釈を続ける
                        ax -= 1;
                        break;
                    }
                    prmtbl.push_ignor(args.to_string());
                    ax += 1;
                }
            }
            else if args.starts_with('-')
            {
                println!("指定されたオプションはありません。");
                usage();
                return Err(false);
            }
            else
            {// 読出し開始URLの取得
                let crrcturl : String;
                if args.starts_with("http")
                {
                    println!("args start with http");
                    crrcturl = args.to_string();
                }
                else if args.starts_with("://")
                {
                    println!("args start with ://");
                    crrcturl = format!("http{}", args);
                }
                else if args.starts_with("//")
                {
                    println!("args start with //");
                    crrcturl = format!("http:{}", args);
                }
                else if args.starts_with("/")
                {
                    println!("args start with /");
                    crrcturl = format!("http:/{}", args);
                }
                else
                {
                    println!("args start no prefix");
                    crrcturl = format!("http://{}", args);
                }
                println!("correct url: {}", crrcturl);

                let urlparts = match get_url_parts(&crrcturl)
                    {
                        Ok(v) => v,
                        Err(e) =>
                        {
                            println!("{}", e);
                            return Err(false);
                        },
                    };
                //crate::help::URLPARTS.set(urlparts).unwrap();
                crate::help::URLPARTS.set(urlparts);
                //println!("{:?}", crate::help::URLPARTS);

                ax += 1;
                loop
                {
                    if argc <= ax
                    {// パラメータ指定は終了
                        ax -= 1;
                        break;
                    }
                    args = &argv[ax];
                    if args.starts_with("-")
                    {// オプション指定が有るときはオプション解釈を続ける
                        ax -= 1;
                        break;
                    }

                    // オプション指定が無いときは読出し対象データの拡張子の取得を続ける
                    // 拡張子は '.' を除いた内容で保存する
                    let mut extid: String = args.to_string();
                    extid.retain(|c| c != '.');
                    unsafe {crate::help::EXTIDLIST.push(extid);}
                    //unsafe {println!("{:?}", crate::help::EXTIDLIST);}
                    ax += 1;
                }
            }
            ax += 1;
        }
        Ok(prmtbl)
    }
}
pub fn get_url_parts(url: &String) -> Result<UrlParts, String>
{
    let mut urlparts = UrlParts::new();

    let strturl = match Url::parse(url)
    {
        Ok(v) => v,
        Err(e) => return Err(format!("UrlParse error {}",e)),
    };
    urlparts.scheme = strturl.scheme().to_string();
    urlparts.host = match strturl.host()
    {
        Some(v) => v.to_string(),
        None => return Err(format!("UrlHost error")),
    };
    urlparts.path = strturl.path().to_string();
    urlparts.port = match strturl.port()
    {
        Some(v) => v.into(),
        None => -1,
    };
    //println!("{:?}", urlparts);

    Ok(urlparts)
}

pub fn get_tls_url_parts(url: &String) -> Result<UrlParts, String>
{
    let mut urlparts = UrlParts::new();

    let strturl = match Url::parse(url)
    {
        Ok(v) => v,
        Err(e) => return Err(format!("UrlParse error {}",e)),
    };
    let scheme = strturl.scheme().to_string();
    if scheme == "http"
    {urlparts.scheme = "https".to_string();}
    else
    {urlparts.scheme = scheme;}
    urlparts.host = match strturl.host()
    {
        Some(v) => v.to_string(),
        None => return Err(format!("UrlHost error")),
    };
    urlparts.path = strturl.path().to_string();
    urlparts.port = match strturl.port()
    {
        Some(v) => v.into(),
        None => -1,
    };
    //println!("{:?}", urlparts);

    Ok(urlparts)
}
