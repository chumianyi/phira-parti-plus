//! Parti+⁺ 内置示例曲播种：首次启动把内置的 3 首示例谱面包
//! （小星星 / 卡农 / 拔萝卜，WAV「录音」+ 已对齐 RPE 谱面）解压到自定义谱面目录。
//!
//! 这是基于 Phira 完整源码新增的 Parti+⁺ 功能模块。

use crate::{dir, scene::import_chart_to};
use anyhow::Result;
use std::io::Write;
use std::path::Path;

const BUILTIN_CHARTS: &[(&str, &[u8])] = &[
    ("twinkle", include_bytes!("../assets/builtin/twinkle.zip")),
    ("canon", include_bytes!("../assets/builtin/canon.zip")),
    ("carrot", include_bytes!("../assets/builtin/carrot.zip")),
];

/// 确保内置示例曲存在于本地谱面目录（幂等：已存在则跳过）。
/// 在 the_main 初始化阶段调用一次。
pub async fn ensure_builtin_charts() -> Result<()> {
    let custom = dir::custom_charts()?;
    for (name, bytes) in BUILTIN_CHARTS {
        let dest = format!("{custom}/{name}");
        if Path::new(&dest).exists() {
            continue;
        }
        // 通过临时文件走 phira 官方谱面导入流程（解压 + info.yml 校验）
        let path = format!("{}/.parti_builtin_{name}.zip", dir::cache()?);
        {
            let mut f = std::fs::File::create(&path)?;
            f.write_all(bytes)?;
        }
        let file = std::fs::File::open(&path)?;
        let r = import_chart_to(Path::new(&dest), format!("custom/{name}"), file).await;
        let _ = std::fs::remove_file(&path);
        r?;
    }
    Ok(())
}
