
use crate::help;
use crate::httpio;

pub async fn lets_surf(prmtbl: help::ParamMap) -> Result <(), Box<dyn std::error::Error + Send + Sync>>
{
    let urlparts = help::get_strturl();

    httpio::lets_raw_surf(0, &urlparts, prmtbl).await?;

    Ok(())
}
