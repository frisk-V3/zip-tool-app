// zip-tool-app/src/main.ts
import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from "@tauri-apps/api/dialog";

// 圧縮ボタンの処理
async function handleCompress() {
  // 1. 固めたいファイルを選択（複数可、パスが配列で返る）
  const selectedPaths = await open({ multiple: true }) as string[] | null;
  if (!selectedPaths) return;

  // 2. 保存先のパスを決める
  const outputZipPath = await save({ 
    filters: [{ name: 'Zip Archive', extensions: ['zip'] }] 
  }) as string | null;
  if (!outputZipPath) return;

  // 3. Rustにパスを渡して実行
  try {
    const message = await invoke("compress_files", { 
      inputPaths: selectedPaths, 
      outputZipPath: outputZipPath 
    });
    alert(message);
  } catch (e) {
    alert("エラー: " + e);
  }
}
