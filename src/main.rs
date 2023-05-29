use anyhow::Result;
use thiserror::Error;
use tracing::{error, info, instrument};

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    info!("num: {:?}", num(1)?);

    info!("num: {:?}", num(10)?);
    Ok(())
}

#[instrument(ret, err)]
fn num(i: i32) -> Result<i32> {
    // 基本系。エラーメッセージなしの場合の実装。
    let result = judge(i).map_err(SampleError::NumError)?;
    Ok(result)

    // // パターン１: 一番やりたい事だが、動かない。どうにかすれば動きそうな気がする。
    // let result = judge(i).map_err(|e| {
    //     error!("何かのエラーメッセージ: {:?}", e);
    //     Err(SampleError::NumError(e.into()).into())
    // })?;
    // Ok(result)

    // // パターン2: matchで書けば、やりたい事はできる。
    // let result = judge(i).map_err(|e| {
    //     error!("何かのエラーメッセージ: {:?}", e);
    //     SampleError::NumError(e).into()
    // });
    // match result {
    //     Ok(v) => Ok(v),
    //     Err(e) => Err(e),
    // }

    // // パターン3: うまくいったが、SampleError::NumErrorで返せてない。
    // let result = judge(i).map(|e| {
    //     error!("何かのエラーメッセージ: {:?}", e);
    //     e
    // })?;
    // Ok(result)

    // // パターン4: ?演算子使わないが、一番理想系な気がするが、この処理の後に処理があったとき、let resultに入れる必要があるのでパターン2に落ち着きそう。
    // judge(i).map_err(|e| {
    //     error!("何かのエラーメッセージ: {:?}", e);
    //     SampleError::NumError(e).into()
    // })
}

#[instrument(ret, err)]
fn judge(i: i32) -> Result<i32> {
    if i < 10 {
        Ok(i)
    } else {
        Err(SampleError::JudgeError("Value is too large.".into()).into())
    }
}

#[derive(Debug, Error)]
enum SampleError {
    #[error("NumError: {0}")]
    NumError(#[from] anyhow::Error),
    #[error("JudgeError: {0}")]
    JudgeError(String),
}
