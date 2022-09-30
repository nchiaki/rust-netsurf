
use std::fs;
use std::fs::{ReadDir, File};
use std::io;
use std::io::{Write, BufWriter, Read, BufReader, Bytes};
use hyper_tls::{HttpsConnector, TlsStream};
use hyper::{Client, Body, Method, Request, Uri};
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};
use bytes::BufMut;
use async_recursion::async_recursion;

use crate::help;
use crate::letssurf;

const ERR_BUILDER : &str = "998";
const ERR_UTF8 : &str = "997";
const ERR_NOTFOUND : &str = "404";

static mut VSTDLST_HTTP : Vec<String> = Vec::new();
static mut VSTDLST_HTTPS : Vec<String> = Vec::new();

                         //123456789a123456789b123456789c123456789d123456789e123456789f123456789g123456789h123456789i123456789j123456789k123456789l12345678
static SPECEDATA : &str = "                                                                                                                                ";
fn indent_out(indnt:i32)
{
    if SPECEDATA.len() <= indnt as usize
    {
        print!("{}", SPECEDATA);
        let resnum = indnt as usize - SPECEDATA.len();
        let mut cnt = 0;
        while cnt < resnum
        {
            print!(" ");
            cnt += 1;
        }
    }
    else
    {print!("{}", &SPECEDATA[..indnt as usize]);}
}

fn current_path(urls:String) -> String
{
    if urls.ends_with(".htm")||urls.ends_with(".html")||urls.ends_with(".shtm")||urls.ends_with(".shtml")
    {
        match urls.rfind('/')
        {
            Some(v) => (&urls[..v+1]).to_string(),
            None => urls,
        }
    }
    else
    {urls}
}

fn is_visited_http(urls:String) -> bool
{
    unsafe
    {
        if VSTDLST_HTTP.iter().any(|e| e.to_string()==urls)
        {
            //println!("Already location: {}", urls);
            return true;
        }
        else
        {
            VSTDLST_HTTP.push(urls);
            return false;
        }
    }
}
fn is_visited_https(urls:String) -> bool
{
    unsafe
    {
        if VSTDLST_HTTPS.iter().any(|e| e.to_string()==urls)
        {
            //println!("Already location: {}", urls);
            return true;
        }
        else
        {
            VSTDLST_HTTPS.push(urls);
            return false;
        }
    }
}

#[async_recursion]
pub async fn get_content(indnt:i32, urls:String, mut prmtbl: help::ParamMap) -> Result <String, Box<dyn std::error::Error + Send + Sync>>
{
    if prmtbl.is_ignor(urls.clone())
    {return Ok("200".to_string());}
    if is_visited_http(urls.clone())
    {return Ok("200".to_string());}

    let url = urls.parse()?;
    let client = Client::new();
    //let mut resp = client.get(url).await?;
    let mut resp = match client.get(url).await
        {
            Ok(v) => v,
            Err(e) =>
            {
                indent_out(indnt);
                println!("http client get error:{}", e);
                return Ok("999".to_string());
            },
        };
    let stat = resp.status().to_string();

    //print!("\n");
    //indent_out(indnt);
    //println!("GET = {} ============================================================", stat);

    //indent_out(indnt);
    //println!("{}", urls);
    //println!(":{:?}\n", resp);

    if stat.starts_with("200")
    {
        //let mut crrntpth : String  = String::new();
        let crrntpth = current_path(urls.clone());
        //indent_out(indnt);
        //println!("Current Path:{}", crrntpth);

        // httpサイトは utf-8 の保証がない
        //let mut cnt = 0;
        let mut charbody : Vec<char> = Vec::new();
        while let Some(chunk) = resp.body_mut().data().await
        {
            let binbody = match chunk
                {
                    Ok(v) => v,
                    Err(_) => break,
                };
            // 取得したバイナリデータを char型バイトデータ として保存
            for bd in binbody.iter()
            {
                charbody.push(*bd as char);
            }
            //cnt += 1;
        }
        // char型バイトデータを一文字ごとにストリングデータに変換
        let strngbody:Vec<String> = charbody.iter().map(|x| x.to_string()).collect();

        // 一文字ごとのストリングデータを一つのストリングとしてまとめる
        let mut bodybuf : String = "".to_string();
        for strdt in strngbody.iter()
        {
            bodybuf = format!("{}{}", bodybuf, strdt);
        }

        body_surf(indnt, crrntpth, bodybuf.clone(), prmtbl).await?;

        if help::is_content_display_enable()
        {
            println!("{}",bodybuf);
        }
    }

    Ok(stat)
}

#[async_recursion]
pub async fn get_tls_content(indnt:i32, urls:String, mut prmtbl: help::ParamMap) -> Result <String, Box<dyn std::error::Error + Send + Sync>>
{
    if prmtbl.is_ignor(urls.clone())
    {return Ok("200".to_string());}
    if is_visited_https(urls.clone())
    {return Ok("200".to_string());}

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    //let mut resp = client.get(urls.parse()?).await?;
    let mut resp = match client.get(urls.parse()?).await
        {
            Ok(v) => v,
            Err(e) =>
            {
                indent_out(indnt);
                println!("https builder error:{}", e);
                return Ok(ERR_BUILDER.to_string());
            }
        };
    let stat = resp.status().to_string();

    //print!("\n");
    //indent_out(indnt);
    //println!("GET HTTPS = {} ====================================================", stat);
    //indent_out(indnt);
    //println!("{}", urls);
    //println!(":{:?}\n", resp);

    if stat.starts_with("200")
    {
        //let mut crrntpth : String  = String::new();
        let crrntpth = current_path(urls.clone());
        //indent_out(indnt);
        //println!("Current Path:{}", crrntpth);

        let strm = resp.body_mut();
        //println!("body_mut:{:?}", strm);

        let mut _cnt = 0;
        let mut bodybuf : String = "".to_string();
        loop
        {
            let okbody = match strm.data().await
                {
                    Some(v) => v,
                    None => break,
                };
            //println!("{:?}", okbody);
            let body : String;
            if prmtbl.is_chk_utf8()
            {
                body = match String::from_utf8(okbody.expect("Error").to_vec())
                {
                    Ok(v) => v,
                    Err(_e) =>
                    {
                        indent_out(indnt);
                        if prmtbl.is_stop_utf8()
                        {
                            println!("Illegal UTF-8");
                            if prmtbl.is_stop_utf8dump()
                            {println!("{}", bodybuf);}
                            return Ok(ERR_UTF8.to_string());
                        }
                        else
                        {
                            println!("Illegal UTF-8 No encode read");
                            let https = HttpsConnector::new();
                            let client = Client::builder().build::<_, hyper::Body>(https);
                            let mut resp = match client.get(urls.parse()?).await
                                {
                                    Ok(v) => v,
                                    Err(e) =>
                                    {
                                        indent_out(indnt);
                                        println!("https builder error:{}", e);
                                        return Ok(ERR_BUILDER.to_string());
                                    },
                                };
                            let stat = resp.status().to_string();
                            if stat.starts_with("200")
                            {
                                let mut _cnt = 0;
                                let mut charbody : Vec<char> = Vec::new();
                                while let Some(strm) = resp.body_mut().data().await
                                {
                                    let binbody = match strm
                                        {
                                            Ok(v) => v,
                                            Err(_e) => break,
                                        };
                                    // 取得したバイナリデータを char型バイトデータ として保存
                                    for bd in binbody.iter()
                                    {
                                        charbody.push(*bd as char);
                                    }
                                    _cnt += 1;
                                }
                                // char型バイトデータを一文字ごとにストリングデータに変換
                                let strngbody:Vec<String> = charbody.iter().map(|x| x.to_string()).collect();
                                // 一文字ごとのストリングデータを一つのストリングとしてまとめる
                                let mut bodybuf : String = "".to_string();
                                for strdt in strngbody.iter()
                                {
                                    bodybuf = format!("{}{}", bodybuf, strdt);
                                }
                                body_surf(indnt, crrntpth, bodybuf.clone(), prmtbl).await?;
                            }
                            return Ok(stat);
                        }
                    },
                };
            }
            else
            {
                body = unsafe{String::from_utf8_unchecked(okbody.expect("Error").to_vec())};
            }
            bodybuf = format!("{}{}", bodybuf, body);

            //println!("[{}]", cnt);
            _cnt += 1;
        }

        body_surf(indnt, crrntpth, bodybuf.clone(), prmtbl).await?;

        if help::is_content_display_enable()
        {
            println!("{}", bodybuf);
        }
    }

    Ok(stat)
}

#[async_recursion]
pub async fn get_data(indnt:i32, urls:String, mut prmtbl: help::ParamMap) -> Result <String, Box<dyn std::error::Error + Send + Sync>>
{
    if prmtbl.is_ignor(urls.clone())
    {return Ok("200".to_string());}
    if is_visited_http(urls.clone())
    {return Ok("200".to_string());}

    let url = urls.parse()?;
    let client = Client::new();
    let mut resp = match client.get(url).await
        {
            Ok(v) => v,
            Err(e) =>
            {
                indent_out(indnt);
                println!("http client get error:{}", e);
                return Ok("999".to_string());
            },
        };
    let stat = resp.status().to_string();

    if stat.starts_with("200")
    {
        let mut fsiz : usize = 0;
        let trgtdir = help::get_data_dir();
        let filepath = format!("{}/{}", trgtdir, prmtbl.get_data_filename());
        let dtfile = File::create(filepath.clone())?;
        let mut dtwrite = BufWriter::new(dtfile);
        /***
        let mut crrntpth : String  = String::new();
        crrntpth = current_path(urls.clone());
        ***/
        // httpサイトは utf-8 の保証がない
        //let mut cnt = 0;
        //let mut charbody : Vec<char> = Vec::new();
        while let Some(chunk) = resp.body_mut().data().await
        {
            let binbody = match chunk
                {
                    Ok(v) => v,
                    Err(_) => break,
                };
            dtwrite.write_all(&binbody);
            //println!("http:{:?}", binbody);
            fsiz += binbody.len();
            //cnt += 1;
        }
        dtwrite.flush()?;

        indent_out(indnt);
        println!("{} {}", filepath, fsiz);

        if prmtbl.is_chk_duplicate()
        {check_convergence(&trgtdir, &filepath);}
    }

    Ok(stat)
}
#[async_recursion]
pub async fn get_tls_data(indnt:i32, urls:String, mut prmtbl: help::ParamMap) -> Result <String, Box<dyn std::error::Error + Send + Sync>>
{
    if prmtbl.is_ignor(urls.clone())
    {return Ok("200".to_string());}
    if is_visited_https(urls.clone())
    {return Ok("200".to_string());}

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut resp = match client.get(urls.parse()?).await
        {
            Ok(v) => v,
            Err(e) =>
            {
                indent_out(indnt);
                println!("https builder error:{}", e);
                return Ok(ERR_BUILDER.to_string());
            }
        };
    let stat = resp.status().to_string();
    if stat.starts_with("200")
    {
        let mut fsiz : usize = 0;
        let trgtdir = help::get_data_dir();
        let filepath = format!("{}/{}", trgtdir, prmtbl.get_data_filename());
        let dtfile = File::create(filepath.clone())?;
        let mut dtwrite = BufWriter::new(dtfile);
        //let mut _cnt = 0;
        //let mut charbody : Vec<char> = Vec::new();
        while let Some(strm) = resp.body_mut().data().await
        {
            let binbody = match strm
                {
                    Ok(v) => v,
                    Err(_e) => break,
                };
            dtwrite.write_all(&binbody);
            //println!("https:{:?}", binbody);
            fsiz += binbody.len();
            //_cnt += 1;
        }
        dtwrite.flush()?;

        indent_out(indnt);
        println!("{} {}", filepath, fsiz);

        if prmtbl.is_chk_duplicate()
        {check_convergence(&trgtdir, &filepath);}
    }
    Ok(stat)
}

pub fn check_convergence(targetdir: &str, basefile:&str)
{
    let dirs = match fs::read_dir(targetdir)
    {
        Ok(v) => v,
        Err(_) => return,
    };

    let basemeta = match fs::metadata(basefile)
    {
        Ok(v) => v,
        Err(e) =>
        {
            println!("{}: {}", e, basefile);
            return;
        },
    };
    //println!("{:?}{}", basemeta, basemeta.len());

    //println!("{:?}", dirs);
    for mmbr in dirs
    {
        let name = match mmbr
        {
            Ok(v) => v,
            Err(_) => continue,
        };
        let chkname = name.path().display().to_string();
        if basefile.to_string() != chkname
        {
            let checkmeta = match fs::metadata(chkname.clone())
            {
                Ok(v) => v,
                Err(e) =>
                {
                    println!("{}: {}", e, chkname);
                    return;
                },
            };
            if basemeta.len() == checkmeta.len()
            {
                //println!("{}", chkname);
                //println!("{:?}{}", checkmeta, checkmeta.len());
                compare_file(basemeta.len().try_into().unwrap(), basefile, &chkname);
            }
        }
    }
}

pub fn compare_file(fsize:usize, basefile:&str, chkname:&str) -> std::io::Result<()>
{
    let mut trgtf = File::open(basefile)?;
    let mut chkf = File::open(chkname)?;
    let mut chksize : usize = 0;

    let mut trgtbuf = [0;1];
    let mut chkbuf = [0;1];
    loop
    {
        if fsize < chksize
        {break;}

        match trgtf.read(&mut trgtbuf)?
        {
            0 => break,
            n =>
            {
                match chkf.read(&mut chkbuf)?
                {
                    0 => break,
                    n =>
                    {
                            if trgtbuf[0] != chkbuf[0]
                            {break;}
                    },
                }
            },
        }

        chksize += 1;
    }


    /***
    let trgtrdr = BufReader::new(trgtf);
    let chkrdr = BufReader::new(chkf);
    let mut trgtbytes:Vec<u8> = Vec::new();
    for trgtdat in trgtrdr.bytes()
    {
        trgtbytes.push(trgtdat?);
        chksize += 1;
    }
    if chksize != fsize
    {
        println!("{} size unmatch {}/{}", basefile, chksize, fsize);
        return Ok(());
    }
    chksize = 0;
    let mut chkbytes:Vec<u8> = Vec::new();
    for chkdat in chkrdr.bytes()
    {
        chkbytes.push(chkdat?);
        chksize += 1;
    }
    if chksize != fsize
    {
        println!("{} size unmatch {}/{}", chkname, chksize, fsize);
        return Ok(());
    }

    chksize = 0;
    loop
    {
        let trgt = match trgtbytes.pop()
        {
            Some(v) => v,
            None => break,
        };
        let chk = match chkbytes.pop()
        {
            Some(v) => v,
            None => break,
        };
        //println!("{}:{}", trgt, chk);
        if trgt != chk
        {break;}

        chksize += 1;
    }
    ***/

    if chksize == fsize
    {
        println!("{} x {} is same !!!", basefile, chkname);
        match fs::remove_file(chkname)
        {
            Ok(_) => println!("remove {}", chkname),
            Err(e) => println!("{} remove error: {}", chkname, e),
        }
    }
    else
    {println!("{}/{} {} x {} is Illegal ???", chksize, fsize, basefile, chkname);}

    Ok(())
}

async fn body_surf(indnt:i32, currentpath:String, body:String, prmtbl: help::ParamMap) -> Result <(), Box<dyn std::error::Error + Send + Sync>>
{
    let spltws = body.split_whitespace();
    for sim in spltws
    {
        let ext_iter = unsafe{help::EXTIDLIST.iter()};
        for ext in ext_iter
        {
            let dotext = format!(".{}", ext);
            match sim.find(&dotext)
            {
            Some(v) =>
                {
                    //println!("{}:{}", v, sim);
                    let simtmp = &sim[..v+dotext.len()];
                    //println!("{}", simtmp);
                    match simtmp.find("=")
                    {
                    Some(v) =>
                        {
                            let simtmp2 = &simtmp[v+1..];
                            //println!("{}", simtmp2);

                            let mut simtmp4 : &str;
                            if &simtmp2[0..1] == "\""
                            {simtmp4 = &simtmp2[1..];}
                            else
                            {simtmp4 = simtmp2;}

                            let mut currenttmp : &str = &currentpath;

                            if prmtbl.is_log_pathparse()
                            {
                                indent_out(indnt);
                                println!("Bfr  [{}] [{}] ", currenttmp, simtmp4);
                            }
                            while simtmp4.starts_with("../")
                            {

                                simtmp4 = &simtmp4[3..];
                                currenttmp = match currenttmp.rfind('/')
                                    {
                                        Some(v) =>
                                            {
                                                let mut clm = v;
                                                while clm == (currenttmp.len()-1)
                                                {
                                                    currenttmp = &currenttmp[..v];
                                                    clm = match currenttmp.rfind('/')
                                                        {
                                                            Some(vv) => vv,
                                                            None => currenttmp.len(),
                                                        };
                                                }
                                                if clm == currenttmp.len()
                                                {"/"}
                                                else
                                                {
                                                    let slaslax = match currenttmp.find("://")
                                                        {
                                                            Some(vv) =>
                                                                {
                                                                    if (vv + 2) == clm
                                                                    {currenttmp.len()}
                                                                    else
                                                                    {clm}
                                                                },
                                                            None => currenttmp.len(),
                                                        };
                                                    if slaslax == currenttmp.len()
                                                    {currenttmp}
                                                    else
                                                    {&currenttmp[..clm]}
                                                }
                                            },
                                        None => "/",
                                    };
                                if prmtbl.is_log_pathparse()
                                {
                                    indent_out(indnt);
                                    println!("     [{}] [{}] ", currenttmp, simtmp4);
                                }
                            }
                            let currentpathtmp = currenttmp.to_string();
                            let simtmp3 : String = simtmp4.to_string();

                            if prmtbl.is_log_pathparse()
                            {
                                indent_out(indnt);
                                println!("Aftr [{}] [{}]", currentpathtmp, simtmp3);
                            }
                            /*=================*/
                            let trgtpth = {
                                if simtmp3.starts_with("http")
                                {simtmp3}// 外部絶対パス
                                else if simtmp3.starts_with("./")
                                {// コンテンツ内相対パス
                                    if currentpathtmp.ends_with("/")
                                    {format!("{}{}", currentpathtmp, &simtmp3[2..])}
                                    else
                                    {format!("{}{}", currentpathtmp, &simtmp3[1..])}
                                }
                                else if simtmp3.starts_with("/")
                                {// コンテンツ内絶対パス
                                    let sitenamex = match currentpathtmp.find("://")
                                        {
                                            Some(v) => v+3,
                                            None => return Ok(()),
                                        };
                                    let prefix = &currentpathtmp[..sitenamex];
                                    let mut sitename = &currentpathtmp[sitenamex..];
                                    sitename = match sitename.find('/')
                                        {
                                            Some(v) => &sitename[..v],
                                            None => sitename,
                                        };
                                    if sitename.ends_with("/")
                                    {format!("{}{}{}", prefix, sitename, &simtmp3[1..])}
                                    else
                                    {format!("{}{}{}", prefix, sitename, simtmp3)}
                                }
                                else
                                {// コンテンツ内相対パス
                                    if currentpathtmp.ends_with("/")
                                    {format!("{}{}", currentpathtmp, simtmp3)}
                                    else
                                    {format!("{}/{}", currentpathtmp, simtmp3)}
                                }
                            };

                            let urlparts : help::UrlParts = match help::get_url_parts(&trgtpth)
                                {
                                    Ok(v) => v,
                                    Err(e) => {
                                        println!("get_url_parts error:{}",e);
                                        return Ok(());
                                    },
                                };
                            /*========*/

                            if (ext == "html") || (ext == "shtml")
                            {// ネットサーフ（リカーシブル処理）
                                /***
                                let trgtpth = {
                                    if simtmp3.starts_with("http")
                                    {simtmp3}// 外部絶対パス
                                    else if simtmp3.starts_with("./")
                                    {// コンテンツ内相対パス
                                        if currentpathtmp.ends_with("/")
                                        {format!("{}{}", currentpathtmp, &simtmp3[2..])}
                                        else
                                        {format!("{}{}", currentpathtmp, &simtmp3[1..])}
                                    }
                                    else if simtmp3.starts_with("/")
                                    {// コンテンツ内絶対パス
                                        let sitenamex = match currentpathtmp.find("://")
                                            {
                                                Some(v) => v+3,
                                                None => return Ok(()),
                                            };
                                        let prefix = &currentpathtmp[..sitenamex];
                                        let mut sitename = &currentpathtmp[sitenamex..];
                                        sitename = match sitename.find('/')
                                            {
                                                Some(v) => &sitename[..v],
                                                None => sitename,
                                            };
                                        if sitename.ends_with("/")
                                        {format!("{}{}{}", prefix, sitename, &simtmp3[1..])}
                                        else
                                        {format!("{}{}{}", prefix, sitename, simtmp3)}
                                    }
                                    else
                                    {// コンテンツ内相対パス
                                        if currentpathtmp.ends_with("/")
                                        {format!("{}{}", currentpathtmp, simtmp3)}
                                        else
                                        {format!("{}/{}", currentpathtmp, simtmp3)}
                                    }
                                };

                                let urlparts : help::UrlParts = match help::get_url_parts(&trgtpth)
                                    {
                                        Ok(v) => v,
                                        Err(e) => {
                                            println!("get_url_parts error:{}",e);
                                            return Ok(());
                                        },
                                    };
                                ***/

                                //indent_out(indnt);
                                //println!("TargetURL: {}", trgtpth);
                                lets_raw_surf(indnt+help::get_indent(), &urlparts, prmtbl.copy()).await?;
                            }
                            else if prmtbl.is_data_save()
                            {// データセーブ
                                //indent_out(indnt);
                                //println!("DataSave: {}", trgtpth);
                                lets_web_get(indnt+help::get_indent(), &urlparts, prmtbl.copy()).await?;
                            }
                        },
                    None => (),
                    }
                },
            None => (),
            }
        }
    }
    Ok(())
}

pub async fn lets_raw_surf(indnt:i32, urlparts: &help::UrlParts, prmtbl: help::ParamMap) -> Result <(), Box<dyn std::error::Error + Send + Sync>>
{
    let url = urlparts.to_url();

    indent_out(indnt);
    println!("@{}", url);
    //println!("Let's surf! :{:?} => {}", urlparts, url);
    //println!("Let's surf! {}", url);

    if urlparts.get_scheme() == "http".to_string()
    {
        let mut tlsurl = url.clone();
        let mut rtn = crate::httpio::get_content(indnt, url, prmtbl.copy()).await?;
        if ! rtn.starts_with("200")
        {indent_out(indnt);println!("get_content:{}", rtn);}
        if rtn.starts_with("301")  // Moved Permanently
        {
            let tlsurlparts = match help::get_tls_url_parts(&tlsurl)
                {
                    Ok(v) => v,
                    Err(e) =>
                    {
                        println!("{}", e);
                        return Ok(());
                    },
                };
            tlsurl = tlsurlparts.to_url();
            rtn = crate::httpio::get_tls_content(indnt, tlsurl, prmtbl.copy()).await?;
            if ! rtn.starts_with("200")
            {
                indent_out(indnt);
                println!("get_tls_content:{}", rtn);
                if prmtbl.is_stop_utf8() && rtn.starts_with(ERR_UTF8)
                {panic!("Stop UTF-8");}
                if prmtbl.is_stop_builder() && rtn.starts_with(ERR_BUILDER)
                {panic!("Stop Builder");}
                if prmtbl.is_stop_notfound() && rtn.starts_with(ERR_NOTFOUND)
                {panic!("Stop NotFound");}
            }
        }
    }
    else if urlparts.get_scheme() == "https".to_string()
    {
        let tlsurl = urlparts.to_url();
        let rtn = crate::httpio::get_tls_content(indnt, tlsurl, prmtbl.copy()).await?;
        if ! rtn.starts_with("200")
        {
            indent_out(indnt);
            println!("get_tls_content:{}", rtn);
            if prmtbl.is_stop_utf8() && rtn.starts_with(ERR_UTF8)
            {panic!("Stop UTF-8");}
            if prmtbl.is_stop_builder() && rtn.starts_with(ERR_BUILDER)
            {panic!("Stop Builder");}
            if prmtbl.is_stop_notfound() && rtn.starts_with(ERR_NOTFOUND)
            {panic!("Stop NotFound");}
        }
    }
    Ok(())
}

pub async fn lets_web_get(indnt:i32, urlparts: &help::UrlParts, mut prmtbl: help::ParamMap) -> Result <(), Box<dyn std::error::Error + Send + Sync>>
{
    let url = urlparts.to_url();
    let fx = match url.as_str().rfind('/')
    {
        Some(v) => v,
        None => {return Ok(());},
    };
    let fname = &url[fx+1..];

    indent_out(indnt);
    println!("*{} {}", url, fname);

    prmtbl.put_data_filename(fname.to_string());

    if urlparts.get_scheme() == "http".to_string()
    {
        let mut tlsurl = url.clone();
        let mut rtn = crate::httpio::get_data(indnt, url, prmtbl.copy()).await?;
        if ! rtn.starts_with("200")
        {indent_out(indnt);println!("get_data:{}", rtn);}
        if rtn.starts_with("301")  // Moved Permanently
        {
            let tlsurlparts = match help::get_tls_url_parts(&tlsurl)
                {
                    Ok(v) => v,
                    Err(e) =>
                    {
                        println!("{}", e);
                        return Ok(());
                    },
                };
            tlsurl = tlsurlparts.to_url();
            rtn = crate::httpio::get_tls_data(indnt, tlsurl, prmtbl.copy()).await?;
            if ! rtn.starts_with("200")
            {
                indent_out(indnt);
                println!("get_tls_data:{}", rtn);
                if prmtbl.is_stop_builder() && rtn.starts_with(ERR_BUILDER)
                {panic!("Stop Builder");}

            }
        }
    }
    else if urlparts.get_scheme() == "https".to_string()
    {
        let tlsurl = urlparts.to_url();
        let rtn = crate::httpio::get_tls_data(indnt, tlsurl, prmtbl.copy()).await?;
        if ! rtn.starts_with("200")
        {
            indent_out(indnt);
            println!("get_tls_data:{}", rtn);
            if prmtbl.is_stop_builder() && rtn.starts_with(ERR_BUILDER)
            {panic!("Stop Builder");}
        }
    }
    Ok(())
}
